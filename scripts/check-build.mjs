#!/usr/bin/env node
// Layress 构建 / 类型 / 单元测试门禁（add-tauri-page-host 任务 5.1）
//
// 用法：
//   node scripts/check-build.mjs --all        # 实跑三条检查，任一失败即整体失败
//   node scripts/check-build.mjs --selftest   # 反证：失败必须被传播，不能被静默吞掉
//
// 存在理由：5.1 之前，构建 / 类型 / 测试要手工单独跑，pre-commit 与 CI 的公共链里没有它们，
// 于是「文档门禁全绿但代码编译不过」可以一路混到归档。这里让同一条链覆盖。
//
// 平台适用范围：本机 macOS arm64 已实跑；CI ubuntu 见 .github/workflows/ci.yml；
// Windows 未验证（单列，不混写）。工具链缺失时明确失败，不提供跳过开关。
//
// 与 --staged 的关系：索引快照里没有 node_modules / target 缓存，无法编译，
// 因此构建类检查由 check-repo.mjs 指定在真实检出目录执行；洁净度 / 链接 / 双语仍按索引快照。
// 这个差异是如实记录的口径，不是绕过：构建失败照样让整条链退出非 0。

import { spawnSync } from 'node:child_process';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const TAURI_DIR = path.join(ROOT, 'apps', 'desktop', 'src-tauri');
const args = process.argv.slice(2);
const SELFTEST = args.includes('--selftest');

/** 解析可执行文件：优先 PATH；cargo 额外尝试 rustup 的默认安装位置。
 *  工具链缺失属于环境阻塞，必须明确报错，不能静默跳过（任务 5.1）。 */
function resolveTool(name) {
  const probe = spawnSync(name, ['--version'], { encoding: 'utf8' });
  if (!probe.error) return name;
  if (name === 'cargo') {
    const home = os.homedir();
    const fallback = path.join(home, '.cargo', 'bin', 'cargo');
    if (fs.existsSync(fallback)) return fallback;
  }
  return null;
}

function runStep(step) {
  const started = Date.now();
  console.log(`[check-build] ${step.name}：${step.cmd} ${step.args.join(' ')}`);
  const r = spawnSync(step.cmd, step.args, { cwd: step.cwd, stdio: 'inherit' });
  const ms = Date.now() - started;
  if (r.error) {
    console.error(`[check-build] ✗ ${step.name}：无法启动（${r.error.message}）`);
    return { name: step.name, ok: false, detail: r.error.message };
  }
  console.log(`[check-build] ${r.status === 0 ? '✓' : '✗'} ${step.name}：退出码 ${r.status} · ${ms} ms`);
  return { name: step.name, ok: r.status === 0, detail: `退出码 ${r.status}` };
}

function realSteps() {
  const pnpm = resolveTool('pnpm');
  const cargo = resolveTool('cargo');
  const missing = [];
  if (!pnpm) missing.push('pnpm');
  if (!cargo) missing.push('cargo');
  if (missing.length) {
    console.error(`[check-build] 工具链缺失：${missing.join(' / ')}；请先安装再跑门禁，不提供跳过开关`);
    process.exit(1);
  }
  return [
    { name: '前端类型检查与构建', cmd: pnpm, args: ['build:desktop'], cwd: ROOT },
    { name: 'Rust 编译检查', cmd: cargo, args: ['check'], cwd: TAURI_DIR },
    { name: 'Rust lib 单元测试', cmd: cargo, args: ['test', '--lib'], cwd: TAURI_DIR },
  ];
}

/** 工具层版本徽标一致性：改了 main.ts 却忘了同步 index.html 的徽标时，
 *  GUI 上的「是不是新窗口」自证会失效（旧 DOM / CSP 挡 HMR 的坑已踩三次）。
 *  这是静态断言，不依赖 GUI，却能把「靠记性防」变成代码拦住。 */
function checkBuildTag() {
  const html = fs.readFileSync(path.join(ROOT, 'apps', 'desktop', 'index.html'), 'utf8');
  const ts = fs.readFileSync(path.join(ROOT, 'apps', 'desktop', 'src', 'main.ts'), 'utf8');
  const inHtml = /id="build-tag"[^>]*data-build="([^"]+)"/.exec(html)?.[1];
  const inTs = /const BUILD_TAG = "([^"]+)"/.exec(ts)?.[1];
  if (!inHtml || !inTs || inHtml !== inTs) {
    console.error(
      `[check-build] ✗ 工具层版本徽标不一致或缺失：index.html=${inHtml ?? '缺失'} / main.ts=${inTs ?? '缺失'}`,
    );
    return false;
  }
  console.log(`[check-build] ✓ 工具层版本徽标一致：${inTs}`);
  return true;
}

/** 样例夹具的三条静态不变量，都是 2026-11-14 人工复验踩出来的：
 *  下载 MIME 必须不可渲染（否则 on_download 不触发）；坐标参照块不能拦截点击；
 *  地理位置负例必须有超时兜底（否则权限请求挂起时页面一片空白，像「点了没反应」）。 */
function checkFixtureInvariants() {
  const fixture = fs.readFileSync(
    path.join(ROOT, 'apps', 'desktop', 'src-tauri', 'src', 'fixture.rs'),
    'utf8',
  );
  const html = fs.readFileSync(
    path.join(ROOT, 'apps', 'desktop', 'src-tauri', 'fixtures', 'sample-page.html'),
    'utf8',
  );
  const checks = [
    [
      '下载夹具可触发 WKWebView 下载回调',
      fixture.includes('Content-Type: application/octet-stream') &&
        fixture.includes('Content-Disposition: attachment'),
    ],
    [
      '坐标参照块不拦截点击',
      /#coord-anchor\s*\{[^}]*pointer-events:\s*none/.test(html),
    ],
    [
      '地理位置负例有超时兜底',
      html.includes('权限请求 2s 内未得到系统回调'),
    ],
  ];
  let ok = true;
  for (const [name, pass] of checks) {
    console.log(`[check-build] ${pass ? '✓' : '✗'} 夹具不变量：${name}`);
    if (!pass) ok = false;
  }
  return ok;
}

function main() {
  console.log(
    `Layress 构建门禁 · 模式 ${SELFTEST ? 'selftest' : 'all'} · ${process.platform}/${process.arch} · Node ${process.version}`,
  );
  if (!SELFTEST && (!checkBuildTag() || !checkFixtureInvariants())) process.exit(1);
  const steps = SELFTEST
    ? [
        { name: '正例：恒成功命令应通过', cmd: process.execPath, args: ['-e', 'process.exit(0)'], cwd: ROOT },
        { name: '反例：恒失败命令必须被拦', cmd: process.execPath, args: ['-e', 'process.exit(7)'], cwd: ROOT },
      ]
    : realSteps();
  const results = steps.map(runStep);

  if (SELFTEST) {
    const passed = results[0]?.ok === true && results[1]?.ok === false;
    console.log(
      passed
        ? '自测通过：成功命令放行、失败命令未被静默吞掉'
        : '自测失败：退出码传播口径被破坏',
    );
    process.exit(passed ? 0 : 1);
  }

  const failed = results.filter((r) => !r.ok);
  console.log(`结论：${failed.length} 项失败 ⇒ ${failed.length ? '拒绝' : '放行'}`);
  for (const f of failed) console.error(`  ✗ ${f.name}：${f.detail}`);
  process.exit(failed.length ? 1 : 0);
}

main();
