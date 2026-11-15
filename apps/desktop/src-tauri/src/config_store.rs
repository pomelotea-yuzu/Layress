//! 项目配置的文件存储（add-project-device-profiles 任务 1.3）。
//!
//! 落点：`<data-root>/config/projects.json`，锁与临时文件同在该目录；
//! 不使用 WebView localStorage、系统用户配置目录或第二套数据根（I12 / I13）。
//!
//! 本轮只落地**可复用的存储模块与机器证据**：独占写锁、进程内串行写队列、
//! 同目录临时文件 + fsync + 原子替换、revision 单调与过期拒绝、读取 / 损坏 /
//! 未知版本 / 写入失败的分层错误码，以及真实子进程的双实例与中断测试。
//! **尚未接入 Tauri 命令与启动流程**（归任务 1.1 / 2.x），因此模块接线前允许未使用项；
//! 备份重建、只读实例完整交互、启动错误红条分别归 1.5 / 1.4，不在这里假称已实现。
#![allow(dead_code)]

use crate::project_config::{ConfigError, ConfigErrorCode, ProjectConfig};
use std::fs::{self, File, OpenOptions, TryLockError};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::sync::atomic::{AtomicU64, Ordering};
use std::sync::Mutex;

pub const CONFIG_FILE: &str = "projects.json";
pub const LOCK_FILE: &str = "projects.lock";

fn err(code: ConfigErrorCode, message: impl Into<String>) -> ConfigError {
    ConfigError::new(code, message)
}

/// 写入故障注入点：仅测试构建会传入非 `None` 值，产品路径恒为 `None`，
/// 因此这些分支不是产品可触发的行为（`Crash` 甚至不编进产品二进制）。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum WriteFault {
    None,
    Write,
    Sync,
    Rename,
    #[cfg(test)]
    Crash,
}

/// 一个数据根上的配置存储句柄。`lock` 持有期间独占；进程退出由系统自动释放，
/// 不依赖 PID 文件或手工清理，也不会把「锁文件存在」误当成「锁仍被占用」。
#[derive(Debug)]
pub struct ConfigStore {
    dir: PathBuf,
    path: PathBuf,
    lock_path: PathBuf,
    lock: Option<File>,
    writable: bool,
    state: Mutex<ProjectConfig>,
    seq: AtomicU64,
}

impl ConfigStore {
    /// 打开数据根：文件缺失表示空项目集（不是损坏）；拿不到锁的第二实例只读展示。
    pub fn open(root: &Path) -> Result<Self, ConfigError> {
        let dir = root.join("config");
        fs::create_dir_all(&dir).map_err(|e| {
            err(
                ConfigErrorCode::WriteFailed,
                format!("配置目录不可创建（{}）：{e}", dir.display()),
            )
        })?;
        let lock_path = dir.join(LOCK_FILE);
        let lock = OpenOptions::new()
            .create(true)
            .read(true)
            .write(true)
            .open(&lock_path)
            .map_err(|e| {
                err(
                    ConfigErrorCode::ReadFailed,
                    format!("配置锁不可打开（{}）：{e}", lock_path.display()),
                )
            })?;
        let writable = match lock.try_lock() {
            Ok(()) => true,
            Err(TryLockError::WouldBlock) => false,
            Err(e) => {
                return Err(err(
                    ConfigErrorCode::ReadFailed,
                    format!("配置锁检测失败：{e:?}"),
                ))
            }
        };
        let path = dir.join(CONFIG_FILE);
        let config = read_config(&path)?;
        Ok(Self {
            dir,
            path,
            lock_path,
            lock: Some(lock),
            writable,
            state: Mutex::new(config),
            seq: AtomicU64::new(0),
        })
    }

    pub fn config(&self) -> ProjectConfig {
        self.state.lock().map(|s| s.clone()).unwrap_or_else(|_| ProjectConfig::empty())
    }

    pub fn revision(&self) -> u64 {
        self.state.lock().map(|s| s.revision).unwrap_or(0)
    }

    pub fn is_writable(&self) -> bool {
        self.writable
    }

    pub fn path(&self) -> &Path {
        &self.path
    }

    pub fn lock_path(&self) -> &Path {
        &self.lock_path
    }

    /// 写入前的 revision 必须由调用方从自己读到的配置带入；过期写入不得提交、不改内存模型。
    pub fn commit(&self, next: ProjectConfig, expected: u64) -> Result<u64, ConfigError> {
        self.commit_with_fault(next, expected, WriteFault::None)
    }

    /// 故障注入入口只供单测调用；产品路径走 `commit`（fault 恒 `None`）。
    pub fn commit_with_fault(
        &self,
        mut next: ProjectConfig,
        expected: u64,
        fault: WriteFault,
    ) -> Result<u64, ConfigError> {
        if !self.writable {
            return Err(err(
                ConfigErrorCode::Locked,
                "另一实例占用配置，本实例只读",
            ));
        }
        // 进程内只有一个串行写队列：整段「校验 → 写临时 → 同步 → 替换 → 发布」都在锁内。
        let mut state = self
            .state
            .lock()
            .map_err(|_| err(ConfigErrorCode::WriteFailed, "配置状态锁不可用"))?;
        if state.revision != expected {
            return Err(err(
                ConfigErrorCode::Stale,
                format!(
                    "配置已被更新：期望 revision {expected}，当前 {}",
                    state.revision
                ),
            ));
        }
        next.commit_revision()?;
        let bytes = next.to_json()?;
        atomic_write(&self.dir, &self.path, &bytes, &self.seq, fault)?;
        let revision = next.revision;
        *state = next;
        Ok(revision)
    }

    /// 重读磁盘（重取锁后必须调用，避免沿用旧 revision）。
    pub fn reload(&self) -> Result<(), ConfigError> {
        let next = read_config(&self.path)?;
        let mut state = self
            .state
            .lock()
            .map_err(|_| err(ConfigErrorCode::WriteFailed, "配置状态锁不可用"))?;
        *state = next;
        Ok(())
    }

    /// 第二实例在锁释放后显式重取；成功即重读最新 revision，再启用写入。
    pub fn try_acquire(&mut self) -> Result<bool, ConfigError> {
        if self.writable {
            return Ok(true);
        }
        let Some(file) = self.lock.as_ref() else {
            return Ok(false);
        };
        match file.try_lock() {
            Ok(()) => {
                self.writable = true;
                self.reload()?;
                Ok(true)
            }
            Err(TryLockError::WouldBlock) => Ok(false),
            Err(e) => Err(err(
                ConfigErrorCode::ReadFailed,
                format!("重取配置锁失败：{e:?}"),
            )),
        }
    }
}

fn read_config(path: &Path) -> Result<ProjectConfig, ConfigError> {
    match fs::read(path) {
        Ok(bytes) => ProjectConfig::from_json(&bytes),
        Err(e) if e.kind() == io::ErrorKind::NotFound => Ok(ProjectConfig::empty()),
        Err(e) => Err(err(
            ConfigErrorCode::ReadFailed,
            format!("配置读取失败（{}）：{e}", path.display()),
        )),
    }
}

/// 同目录唯一临时文件 → 完整写入 → fsync → 原子替换；任一阶段失败都保留原文件、
/// 清理本次临时文件，并把 revision 留在旧值。断电耐久性与未实测平台单列。
fn atomic_write(
    dir: &Path,
    path: &Path,
    bytes: &[u8],
    seq: &AtomicU64,
    fault: WriteFault,
) -> Result<(), ConfigError> {
    let n = seq.fetch_add(1, Ordering::Relaxed);
    let tmp = dir.join(format!("projects.tmp.{}.{}.json", std::process::id(), n));
    let cleanup = |t: &Path| {
        let _ = fs::remove_file(t);
    };
    if fault == WriteFault::Write {
        return Err(err(
            ConfigErrorCode::WriteFailed,
            "注入：临时文件写入阶段失败",
        ));
    }
    let mut file = OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(&tmp)
        .map_err(|e| {
            err(
                ConfigErrorCode::WriteFailed,
                format!("配置临时文件不可创建：{e}"),
            )
        })?;
    if let Err(e) = file.write_all(bytes) {
        drop(file);
        cleanup(&tmp);
        return Err(err(
            ConfigErrorCode::WriteFailed,
            format!("配置临时文件写入失败：{e}"),
        ));
    }
    if fault == WriteFault::Sync {
        drop(file);
        cleanup(&tmp);
        return Err(err(
            ConfigErrorCode::WriteFailed,
            "注入：临时文件同步阶段失败",
        ));
    }
    if let Err(e) = file.sync_all() {
        drop(file);
        cleanup(&tmp);
        return Err(err(
            ConfigErrorCode::WriteFailed,
            format!("配置临时文件同步失败：{e}"),
        ));
    }
    drop(file);
    #[cfg(test)]
    if fault == WriteFault::Crash {
        // 模拟进程在替换前中断：临时文件残留，正式文件必须仍是完整旧版本。
        std::process::exit(9);
    }
    if fault == WriteFault::Rename {
        cleanup(&tmp);
        return Err(err(
            ConfigErrorCode::WriteFailed,
            "注入：原子替换阶段失败",
        ));
    }
    if let Err(e) = fs::rename(&tmp, path) {
        cleanup(&tmp);
        return Err(err(
            ConfigErrorCode::WriteFailed,
            format!("配置原子替换失败：{e}"),
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::project_config::PresetId;
    use std::process::Command;
    use std::thread;
    use std::time::{Duration, Instant};

    /// 测试夹具统一放仓库忽略的 `artifacts/`，不写系统用户目录（I12）。
    fn temp_root(tag: &str) -> PathBuf {
        let base = Path::new(env!("CARGO_MANIFEST_DIR"))
            .join("../../../artifacts/config-store-tests")
            .join(format!("{tag}-{}-{}", std::process::id(), crate::capture::unix_ms()));
        fs::create_dir_all(&base).unwrap();
        base
    }

    fn tmp_count(root: &Path) -> usize {
        fs::read_dir(root.join("config"))
            .unwrap()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name().to_string_lossy().starts_with("projects.tmp."))
            .count()
    }

    fn sample() -> ProjectConfig {
        let mut config = ProjectConfig::empty();
        config
            .create_project("p-a", "项目 A", "https://example.com/a", PresetId::Desktop)
            .unwrap();
        config
    }

    #[test]
    fn 首次打开为空配置且不创建正式文件() {
        let root = temp_root("fresh");
        let store = ConfigStore::open(&root).unwrap();
        assert!(store.is_writable());
        assert_eq!(store.revision(), 0);
        assert_eq!(store.config(), ProjectConfig::empty());
        assert!(!store.path().exists(), "首次没有文件表示空项目集，不是损坏");
        assert!(store.lock_path().exists(), "锁文件随数据根创建");
        drop(store);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn 提交后往返恢复且_revision_递增() {
        let root = temp_root("roundtrip");
        let store = ConfigStore::open(&root).unwrap();
        let rev = store.commit(sample(), 0).unwrap();
        assert_eq!(rev, 1);
        assert_eq!(store.revision(), 1);
        let bytes = fs::read(store.path()).unwrap();
        let parsed = ProjectConfig::from_json(&bytes).unwrap();
        assert_eq!(parsed.revision, 1);
        assert!(parsed.project("p-a").is_some());
        drop(store);
        let again = ConfigStore::open(&root).unwrap();
        assert_eq!(again.revision(), 1);
        assert_eq!(again.config(), parsed);
        drop(again);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn 过期_revision_拒绝且原文件字节不变() {
        let root = temp_root("stale");
        let store = ConfigStore::open(&root).unwrap();
        store.commit(sample(), 0).unwrap();
        let before = fs::read(store.path()).unwrap();
        let mut stale = store.config();
        stale.projects[0].rename("晚到的改动").unwrap();
        let e = store.commit(stale, 0).unwrap_err();
        assert_eq!(e.code_str(), "CONFIG_STALE");
        assert_eq!(fs::read(store.path()).unwrap(), before);
        assert_eq!(store.revision(), 1, "失败不推进 revision");
        drop(store);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn 写入同步替换失败均保留原文件且清理临时文件() {
        let root = temp_root("fault");
        let store = ConfigStore::open(&root).unwrap();
        store.commit(sample(), 0).unwrap();
        let before = fs::read(store.path()).unwrap();
        for fault in [WriteFault::Write, WriteFault::Sync, WriteFault::Rename] {
            let mut next = store.config();
            next.projects[0].rename("不该落盘").unwrap();
            let e = store.commit_with_fault(next, store.revision(), fault).unwrap_err();
            assert_eq!(e.code_str(), "CONFIG_WRITE_FAILED");
            assert_eq!(fs::read(store.path()).unwrap(), before, "原文件逐字节不变");
            assert_eq!(tmp_count(&root), 0, "本次临时文件已清理");
        }
        assert_eq!(store.revision(), 1);
        drop(store);
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn 读取失败损坏与未知版本分码() {
        let root = temp_root("codes");
        let dir = root.join("config");
        fs::create_dir_all(&dir).unwrap();
        let path = dir.join(CONFIG_FILE);
        // ① 读取 I/O 失败（正式路径被目录占据），区别于「读取成功但内容坏」
        fs::create_dir(&path).unwrap();
        assert_eq!(
            ConfigStore::open(&root).unwrap_err().code_str(),
            "CONFIG_READ_FAILED"
        );
        fs::remove_dir(&path).unwrap();
        // ② 截断 JSON
        fs::write(&path, b"{not json").unwrap();
        assert_eq!(
            ConfigStore::open(&root).unwrap_err().code_str(),
            "CONFIG_CORRUPT"
        );
        // ③ 可读但未知 schema，不自动降级
        let mut value: serde_json::Value =
            serde_json::from_slice(&ProjectConfig::empty().to_json().unwrap()).unwrap();
        value["schema"] = serde_json::json!("layress.project-config/v99");
        fs::write(&path, serde_json::to_vec(&value).unwrap()).unwrap();
        assert_eq!(
            ConfigStore::open(&root).unwrap_err().code_str(),
            "CONFIG_VERSION_UNSUPPORTED"
        );
        fs::remove_dir_all(root).unwrap();
    }

    /// 子角色入口：由父测试 spawn 当前测试二进制并带环境变量进入；
    /// 正常 libtest 运行时没有该变量，直接返回（不占锁、不做任何事）。
    #[test]
    fn 子进程持锁() {
        let Ok(root) = std::env::var("LAYRESS_CONFIG_TEST_ROOT") else {
            return;
        };
        if std::env::var("LAYRESS_CONFIG_TEST_ROLE").as_deref() != Ok("hold_lock") {
            return;
        }
        let root = PathBuf::from(root);
        let store = ConfigStore::open(&root).unwrap();
        assert!(store.is_writable(), "第一个实例应拿到写锁");
        fs::write(root.join("child-ready"), b"ok").unwrap();
        thread::sleep(Duration::from_secs(20));
    }

    #[test]
    fn 双实例真实子进程持锁时只读且写请求被拒() {
        let root = temp_root("lock");
        let exe = std::env::current_exe().unwrap();
        let mut child = Command::new(&exe)
            .env("LAYRESS_CONFIG_TEST_ROOT", &root)
            .env("LAYRESS_CONFIG_TEST_ROLE", "hold_lock")
            .args(["--exact", "config_store::tests::子进程持锁", "--nocapture"])
            .spawn()
            .unwrap();
        let sentinel = root.join("child-ready");
        let deadline = Instant::now() + Duration::from_secs(10);
        while !sentinel.exists() && Instant::now() < deadline {
            thread::sleep(Duration::from_millis(50));
        }
        assert!(sentinel.exists(), "子进程未在超时内持锁就绪");

        let mut store = ConfigStore::open(&root).unwrap();
        assert!(!store.is_writable(), "第二实例只读");
        let e = store.commit(sample(), 0).unwrap_err();
        assert_eq!(e.code_str(), "CONFIG_LOCKED");
        assert!(!store.path().exists(), "只读实例不得创建配置");

        child.kill().unwrap();
        child.wait().unwrap();
        assert!(store.try_acquire().unwrap(), "锁释放后可重取");
        assert!(store.is_writable());
        assert_eq!(store.revision(), 0, "重取锁后重读最新 revision");
        drop(store);
        fs::remove_dir_all(root).unwrap();
    }

    /// 子角色：在原子替换前被 `WriteFault::Crash` 中断（退出码 9）。
    #[test]
    fn 子进程中断提交() {
        let Ok(root) = std::env::var("LAYRESS_CONFIG_TEST_ROOT") else {
            return;
        };
        if std::env::var("LAYRESS_CONFIG_TEST_ROLE").as_deref() != Ok("crash") {
            return;
        }
        let store = ConfigStore::open(Path::new(&root)).unwrap();
        let mut next = store.config();
        next.create_project("p-new", "新项目", "https://example.org", PresetId::Phone)
            .unwrap();
        let _ = store.commit_with_fault(next, store.revision(), WriteFault::Crash);
        panic!("Crash 注入应已退出进程");
    }

    #[test]
    fn 进程中断后正式文件仍是完整旧版本() {
        let root = temp_root("crash");
        {
            let store = ConfigStore::open(&root).unwrap();
            store.commit(sample(), 0).unwrap();
        }
        let path = root.join("config").join(CONFIG_FILE);
        let before = fs::read(&path).unwrap();

        let exe = std::env::current_exe().unwrap();
        let status = Command::new(&exe)
            .env("LAYRESS_CONFIG_TEST_ROOT", &root)
            .env("LAYRESS_CONFIG_TEST_ROLE", "crash")
            .args(["--exact", "config_store::tests::子进程中断提交", "--nocapture"])
            .status()
            .unwrap();
        assert_eq!(status.code(), Some(9), "子进程应在替换前中断");
        let after = fs::read(&path).unwrap();
        assert_eq!(before, after, "中断不得产生截断或半新配置");
        let parsed = ProjectConfig::from_json(&after).unwrap();
        assert_eq!(parsed.revision, 1);
        assert!(parsed.project("p-a").is_some());
        assert!(parsed.project("p-new").is_none());
        assert!(tmp_count(&root) > 0, "中断点前应有残留临时文件");
        // 重新启动只读正式文件，残留临时文件不参与恢复
        let store = ConfigStore::open(&root).unwrap();
        assert_eq!(store.revision(), 1);
        assert_eq!(store.config(), parsed);
        drop(store);
        fs::remove_dir_all(root).unwrap();
    }
}
