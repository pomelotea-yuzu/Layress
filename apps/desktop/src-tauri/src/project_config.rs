//! 项目与设备配置记忆：配置模型、校验与稳定错误码（add-project-device-profiles 任务 1.2）。
//!
//! 本轮只落地**纯模型层**，不接 Tauri 命令、不读写文件、不碰数据根解析：
//! - 文件存储 / 独占锁 / 原子替换属任务 1.3，数据根相对路径拒绝与启动红条属任务 1.4；
//! - 工具层 UI 与编排属任务 0.1 / 2.x / 3.x，界面可行性前置尚未人工验证。
//! 因此模型校验通过**不等于**任何 GUI 或持久化行为已实现，证据与未执行项见 change 的 evidence-index。
//!
//! 接线后已移除 crate 级 `#![allow(dead_code)]`；个别尚未被产品路径使用的 API
//! 按所属任务加窄范围 allow 并注明归属，不允许整模块静默放行。

use serde::{Deserialize, Serialize};

/// 配置文件的版本标识；与采集元数据 `layress.capture/v1` 相互独立。
pub const PROJECT_SCHEMA: &str = "layress.project-config/v1";

/// 内部应用接口使用的稳定错误码；前端按此映射中文提示，不对外宣称 CLI 协议。
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ConfigErrorCode {
    InvalidInput,
    ReadFailed,
    Corrupt,
    VersionUnsupported,
    WriteFailed,
    Locked,
    Stale,
    /// 数据根相对路径拒绝属任务 1.4，尚未有实现路径抛出；接线时删除本 allow。
    #[allow(dead_code)]
    DataRootRelativeUnsupported,
}

impl ConfigErrorCode {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::InvalidInput => "CONFIG_INVALID_INPUT",
            Self::ReadFailed => "CONFIG_READ_FAILED",
            Self::Corrupt => "CONFIG_CORRUPT",
            Self::VersionUnsupported => "CONFIG_VERSION_UNSUPPORTED",
            Self::WriteFailed => "CONFIG_WRITE_FAILED",
            Self::Locked => "CONFIG_LOCKED",
            Self::Stale => "CONFIG_STALE",
            Self::DataRootRelativeUnsupported => "DATA_ROOT_RELATIVE_UNSUPPORTED",
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConfigError {
    pub code: ConfigErrorCode,
    pub message: String,
}

impl ConfigError {
    pub fn new(code: ConfigErrorCode, message: impl Into<String>) -> Self {
        Self {
            code,
            message: message.into(),
        }
    }
    pub fn code_str(&self) -> &'static str {
        self.code.as_str()
    }
}

impl std::fmt::Display for ConfigError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}：{}", self.code_str(), self.message)
    }
}

impl std::error::Error for ConfigError {}

type ConfigResult<T> = Result<T, ConfigError>;

fn invalid(message: impl Into<String>) -> ConfigError {
    ConfigError::new(ConfigErrorCode::InvalidInput, message)
}
fn corrupt(message: impl Into<String>) -> ConfigError {
    ConfigError::new(ConfigErrorCode::Corrupt, message)
}

/// 通用视口预设；只代表一组起始尺寸，不启用 DPR / UA / 触摸等手机模拟。
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum PresetId {
    Desktop,
    Tablet,
    Phone,
    Custom,
}

impl PresetId {
    /// 预设的初始 CSS px 尺寸；旋转与手改后的尺寸按项目保存，不由此重新计算。
    pub fn default_size(self) -> (u32, u32) {
        match self {
            Self::Desktop => (1280, 800),
            Self::Tablet => (768, 1024),
            Self::Phone => (390, 844),
            Self::Custom => (1280, 800),
        }
    }
}

pub const MIN_VIEWPORT: u32 = 16;
pub const MAX_VIEWPORT: u32 = 4096;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Viewport {
    pub preset_id: PresetId,
    pub width: u32,
    pub height: u32,
}

impl Viewport {
    pub fn for_preset(preset: PresetId) -> Self {
        let (width, height) = preset.default_size();
        Self {
            preset_id: preset,
            width,
            height,
        }
    }

    /// 校验尺寸范围；非有限 / 非整数由 serde 与前端输入类型挡住，坏值不改已保存值。
    pub fn validate(self) -> ConfigResult<()> {
        if !(MIN_VIEWPORT..=MAX_VIEWPORT).contains(&self.width)
            || !(MIN_VIEWPORT..=MAX_VIEWPORT).contains(&self.height)
        {
            return Err(invalid(format!(
                "视口宽高须为 {MIN_VIEWPORT}–{MAX_VIEWPORT} 的整数，实际 {}×{}",
                self.width, self.height
            )));
        }
        Ok(())
    }

    /// 旋转交换宽高但保留预设身份；方形尺寸交换后不变。
    pub fn rotate(&mut self) {
        std::mem::swap(&mut self.width, &mut self.height);
    }

    /// 手改任一宽高即变为自定义；坏值先拒绝，不修改当前视口。
    pub fn set_custom(&mut self, width: u32, height: u32) -> ConfigResult<()> {
        let next = Self {
            preset_id: PresetId::Custom,
            width,
            height,
        };
        next.validate()?;
        *self = next;
        Ok(())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Project {
    pub id: String,
    pub name: String,
    pub start_url: String,
    pub last_loaded_url: Option<String>,
    pub viewport: Viewport,
}

impl Project {
    /// 只由调用方传入稳定 ID；名称不是主键，改名不得改 ID。
    pub fn new(id: &str, name: &str, start_url: &str, preset: PresetId) -> ConfigResult<Self> {
        let project = Self {
            id: id.to_string(),
            name: name.trim().to_string(),
            start_url: normalize_url(start_url)?,
            last_loaded_url: None,
            viewport: Viewport::for_preset(preset),
        };
        project.validate()?;
        Ok(project)
    }

    pub fn validate(&self) -> ConfigResult<()> {
        validate_project_id(&self.id)?;
        validate_name(&self.name)?;
        // 已保存的 URL 同样要过既有校验，避免坏文件被发布为可编辑模型。
        normalize_url(&self.start_url)?;
        if let Some(url) = &self.last_loaded_url {
            normalize_url(url)?;
        }
        self.viewport.validate()
    }

    pub fn rename(&mut self, name: &str) -> ConfigResult<()> {
        let next = name.trim().to_string();
        validate_name(&next)?;
        self.name = next;
        Ok(())
    }

    /// 编辑起始地址不导航、不覆盖最近地址；这两点由编排层保证，本函数只改字段。
    pub fn set_start_url(&mut self, raw: &str) -> ConfigResult<()> {
        self.start_url = normalize_url(raw)?;
        Ok(())
    }

    /// 仅由「可归属的主文档完成加载」回调调用；失败 / 取消 / 过期结果不得调用。
    /// 最近地址记忆归任务 2.3；接线时删除本 allow。
    #[allow(dead_code)]
    pub fn set_last_loaded_url(&mut self, raw: &str) -> ConfigResult<()> {
        self.last_loaded_url = Some(normalize_url(raw)?);
        Ok(())
    }
}

fn validate_project_id(id: &str) -> ConfigResult<()> {
    let ok = !id.is_empty()
        && id.len() <= 64
        && id
            .chars()
            .all(|c| c.is_ascii_alphanumeric() || c == '-' || c == '_');
    if !ok {
        return Err(invalid("项目 ID 须为 1–64 位字母、数字、- 或 _，且不能用作路径"));
    }
    Ok(())
}

fn validate_name(name: &str) -> ConfigResult<()> {
    let trimmed = name.trim();
    let len = trimmed.chars().count();
    if !(1..=80).contains(&len) {
        return Err(invalid("项目名称去首尾空白后须为 1–80 个字符"));
    }
    Ok(())
}

/// 复用宿主既有的 HTTP(S) 地址校验（含 scheme 白名单与 authority 检查），
/// 保存规范化后的字符串，避免大小写 scheme 与草稿混进配置。
fn normalize_url(raw: &str) -> ConfigResult<String> {
    crate::validate_url(raw)
        .map(|url| url.to_string())
        .map_err(invalid)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProjectConfig {
    pub schema: String,
    pub revision: u64,
    pub active_project_id: Option<String>,
    pub projects: Vec<Project>,
}

impl ProjectConfig {
    /// 文件不存在表示空项目集，不是损坏；此函数只产生内存中的空配置，不写盘。
    pub fn empty() -> Self {
        Self {
            schema: PROJECT_SCHEMA.to_string(),
            revision: 0,
            active_project_id: None,
            projects: Vec::new(),
        }
    }

    pub fn validate(&self) -> ConfigResult<()> {
        if self.schema != PROJECT_SCHEMA {
            return Err(ConfigError::new(
                ConfigErrorCode::VersionUnsupported,
                format!("未知配置版本「{}」", self.schema),
            ));
        }
        let mut seen = std::collections::BTreeSet::new();
        for project in &self.projects {
            if !seen.insert(project.id.as_str()) {
                return Err(invalid(format!("项目 ID 重复：{}", project.id)));
            }
            project.validate()?;
        }
        if let Some(active) = &self.active_project_id {
            if !seen.contains(active.as_str()) {
                return Err(invalid(format!("活动项目 {active} 没有对应的项目记录")));
            }
        }
        Ok(())
    }

    /// 解析磁盘字节：JSON 损坏 / 字段缺失 / 重复 ID / 坏 URL / 坏尺寸一律不发布模型。
    /// 先按 schema 值区分「未知版本」，再做完整反序列化，避免高版本文件被误判成损坏。
    pub fn from_json(bytes: &[u8]) -> ConfigResult<Self> {
        let text = std::str::from_utf8(bytes).map_err(|e| corrupt(format!("配置不是 UTF-8：{e}")))?;
        let raw: serde_json::Value =
            serde_json::from_str(text).map_err(|e| corrupt(format!("JSON 解析失败：{e}")))?;
        match raw.get("schema") {
            Some(serde_json::Value::String(s)) if s != PROJECT_SCHEMA => {
                return Err(ConfigError::new(
                    ConfigErrorCode::VersionUnsupported,
                    format!("未知配置版本「{s}」，不自动降级"),
                ));
            }
            Some(serde_json::Value::String(_)) => {}
            _ => return Err(corrupt("配置缺少合法的 schema 字段")),
        }
        let config: Self = serde_json::from_value(raw)
            .map_err(|e| corrupt(format!("配置字段不完整或类型错误：{e}")))?;
        // 来自文件的坏值属于「损坏」，不是用户当前输入的 INVALID_INPUT。
        config.validate().map_err(|e| match e.code {
            ConfigErrorCode::InvalidInput => corrupt(e.message),
            _ => e,
        })?;
        Ok(config)
    }

    pub fn to_json(&self) -> ConfigResult<Vec<u8>> {
        self.validate()?;
        serde_json::to_vec_pretty(self).map_err(|e| {
            ConfigError::new(
                ConfigErrorCode::WriteFailed,
                format!("配置序列化失败：{e}"),
            )
        })
    }

    pub fn project(&self, id: &str) -> Option<&Project> {
        self.projects.iter().find(|p| p.id == id)
    }

    pub fn create_project(
        &mut self,
        id: &str,
        name: &str,
        start_url: &str,
        preset: PresetId,
    ) -> ConfigResult<()> {
        if self.project(id).is_some() {
            return Err(invalid(format!("项目 ID 已存在：{id}")));
        }
        let project = Project::new(id, name, start_url, preset)?;
        self.projects.push(project);
        self.active_project_id = Some(id.to_string());
        Ok(())
    }

    /// 写入前核对持有者读到的 revision；过期写入不得提交，也不得改内存模型。
    /// 任务 1.3 的过期检查在 `config_store::commit` 内完成；本方法供 3.x
    /// 编排层直接持模型校验时使用，接线时删除本 allow。
    #[allow(dead_code)]
    pub fn expect_revision(&self, expected: u64) -> ConfigResult<()> {
        if self.revision != expected {
            return Err(ConfigError::new(
                ConfigErrorCode::Stale,
                format!(
                    "配置已被更新：期望 revision {expected}，当前 {}",
                    self.revision
                ),
            ));
        }
        Ok(())
    }

    /// 仅在原子替换成功后由存储层调用，成功提交才发布新 revision。
    pub fn commit_revision(&mut self) -> ConfigResult<u64> {
        self.revision = self.revision.checked_add(1).ok_or_else(|| {
            ConfigError::new(ConfigErrorCode::WriteFailed, "revision 溢出，拒绝继续写入")
        })?;
        Ok(self.revision)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn sample_config() -> ProjectConfig {
        let mut config = ProjectConfig::empty();
        config
            .create_project("p-a", "项目 A", "https://example.com/a", PresetId::Desktop)
            .unwrap();
        config
            .create_project("p-b", "项目 B", "http://localhost:4173/b", PresetId::Phone)
            .unwrap();
        config
    }

    #[test]
    fn 空配置往返且版本固定() {
        let bytes = ProjectConfig::empty().to_json().unwrap();
        let parsed = ProjectConfig::from_json(&bytes).unwrap();
        assert_eq!(parsed, ProjectConfig::empty());
        assert_eq!(parsed.schema, PROJECT_SCHEMA);
        assert_eq!(parsed.revision, 0);
        assert!(parsed.active_project_id.is_none());
    }

    #[test]
    fn 截断_json_判损坏且不发布模型() {
        let bytes = sample_config().to_json().unwrap();
        let truncated = &bytes[..bytes.len() / 2];
        let err = ProjectConfig::from_json(truncated).unwrap_err();
        assert_eq!(err.code_str(), "CONFIG_CORRUPT");
    }

    #[test]
    fn 未知版本判版本不支持且不自动降级() {
        let mut value: serde_json::Value =
            serde_json::from_slice(&sample_config().to_json().unwrap()).unwrap();
        value["schema"] = serde_json::json!("layress.project-config/v99");
        let err = ProjectConfig::from_json(&serde_json::to_vec(&value).unwrap()).unwrap_err();
        assert_eq!(err.code_str(), "CONFIG_VERSION_UNSUPPORTED");
    }

    #[test]
    fn 重复项目_id_判损坏() {
        let mut value: serde_json::Value =
            serde_json::from_slice(&sample_config().to_json().unwrap()).unwrap();
        let duplicate = value["projects"][0].clone();
        let arr = value["projects"].as_array_mut().unwrap();
        arr.push(duplicate);
        let err = ProjectConfig::from_json(&serde_json::to_vec(&value).unwrap()).unwrap_err();
        assert_eq!(err.code_str(), "CONFIG_CORRUPT");
    }

    #[test]
    fn 悬空活动项目_id_判损坏() {
        let mut value: serde_json::Value =
            serde_json::from_slice(&sample_config().to_json().unwrap()).unwrap();
        value["activeProjectId"] = serde_json::json!("p-missing");
        let err = ProjectConfig::from_json(&serde_json::to_vec(&value).unwrap()).unwrap_err();
        assert_eq!(err.code_str(), "CONFIG_CORRUPT");
    }

    #[test]
    fn 空名称或超长名称被拒且不改原值() {
        let mut project = Project::new("p-a", "原名", "https://example.com", PresetId::Phone)
            .unwrap();
        assert_eq!(
            project.rename("   ").unwrap_err().code_str(),
            "CONFIG_INVALID_INPUT"
        );
        let long_name = "字".repeat(81);
        assert_eq!(
            project.rename(&long_name).unwrap_err().code_str(),
            "CONFIG_INVALID_INPUT"
        );
        assert_eq!(project.name, "原名");
    }

    #[test]
    fn 非法起始地址与危险_scheme_被拒() {
        assert_eq!(
            Project::new("p-a", "A", "file:///etc/passwd", PresetId::Desktop)
                .unwrap_err()
                .code_str(),
            "CONFIG_INVALID_INPUT"
        );
        assert_eq!(
            Project::new("p-a", "A", "http:///path", PresetId::Desktop)
                .unwrap_err()
                .code_str(),
            "CONFIG_INVALID_INPUT"
        );
    }

    #[test]
    fn 非法尺寸被拒且不改已保存视口() {
        let mut view = Viewport::for_preset(PresetId::Phone);
        let before = view;
        assert_eq!(
            view.set_custom(10, 844).unwrap_err().code_str(),
            "CONFIG_INVALID_INPUT"
        );
        assert_eq!(view, before);
        assert_eq!(
            view.set_custom(5000, 844).unwrap_err().code_str(),
            "CONFIG_INVALID_INPUT"
        );
        assert_eq!(view, before);
    }

    #[test]
    fn 旋转保留预设身份手改变自定义且方形不变() {
        let mut view = Viewport::for_preset(PresetId::Phone);
        assert_eq!((view.width, view.height), (390, 844));
        view.rotate();
        assert_eq!((view.width, view.height), (844, 390));
        assert_eq!(view.preset_id, PresetId::Phone);
        view.set_custom(844, 390).unwrap();
        assert_eq!(view.preset_id, PresetId::Custom);
        let mut square = Viewport {
            preset_id: PresetId::Custom,
            width: 600,
            height: 600,
        };
        square.rotate();
        assert_eq!((square.width, square.height), (600, 600));
    }

    #[test]
    fn 两项目_id_稳定且改名不改_id() {
        let mut config = sample_config();
        let id_before = config.project("p-a").unwrap().id.clone();
        config.projects[0].rename("改名后的 A").unwrap();
        assert_eq!(config.project("p-a").unwrap().id, id_before);
        assert_ne!(
            config.project("p-a").unwrap().id,
            config.project("p-b").unwrap().id
        );
        assert_eq!(config.active_project_id.as_deref(), Some("p-b"));
    }

    #[test]
    fn 合法两项目往返恢复确切尺寸与地址() {
        let mut config = sample_config();
        config.projects[1].viewport.rotate();
        config
            .projects[1]
            .set_last_loaded_url("https://example.org/final")
            .unwrap();
        assert_eq!(config.commit_revision().unwrap(), 1);
        let restored = ProjectConfig::from_json(&config.to_json().unwrap()).unwrap();
        assert_eq!(restored, config);
        let b = restored.project("p-b").unwrap();
        assert_eq!((b.viewport.width, b.viewport.height), (844, 390));
        assert_eq!(b.viewport.preset_id, PresetId::Phone);
        assert_eq!(b.last_loaded_url.as_deref(), Some("https://example.org/final"));
        assert_eq!(b.start_url, "http://localhost:4173/b");
    }

    #[test]
    fn 起始地址规范化且不覆盖最近地址() {
        let mut project =
            Project::new("p-a", "A", "  HTTP://Example.com:8080/path?q=1  ", PresetId::Desktop)
                .unwrap();
        assert_eq!(project.start_url, "http://example.com:8080/path?q=1");
        project.set_last_loaded_url("https://example.com/final").unwrap();
        project.set_start_url("https://example.com/new-start").unwrap();
        assert_eq!(project.start_url, "https://example.com/new-start");
        assert_eq!(project.last_loaded_url.as_deref(), Some("https://example.com/final"));
    }

    #[test]
    fn 过期_revision_判_stale() {
        let mut config = sample_config();
        config.commit_revision().unwrap();
        assert_eq!(
            config.expect_revision(0).unwrap_err().code_str(),
            "CONFIG_STALE"
        );
        config.expect_revision(1).unwrap();
    }

    #[test]
    fn 八个错误码字符串稳定() {
        let codes = [
            ConfigErrorCode::InvalidInput,
            ConfigErrorCode::ReadFailed,
            ConfigErrorCode::Corrupt,
            ConfigErrorCode::VersionUnsupported,
            ConfigErrorCode::WriteFailed,
            ConfigErrorCode::Locked,
            ConfigErrorCode::Stale,
            ConfigErrorCode::DataRootRelativeUnsupported,
        ];
        let strings: Vec<&str> = codes.iter().map(|c| c.as_str()).collect();
        assert_eq!(
            strings,
            vec![
                "CONFIG_INVALID_INPUT",
                "CONFIG_READ_FAILED",
                "CONFIG_CORRUPT",
                "CONFIG_VERSION_UNSUPPORTED",
                "CONFIG_WRITE_FAILED",
                "CONFIG_LOCKED",
                "CONFIG_STALE",
                "DATA_ROOT_RELATIVE_UNSUPPORTED",
            ]
        );
    }
}
