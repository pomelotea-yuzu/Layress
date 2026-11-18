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
import { createHash } from 'node:crypto';
import fs from 'node:fs';
import os from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
const TAURI_DIR = path.join(ROOT, 'apps', 'desktop', 'src-tauri');
const args = process.argv.slice(2);
const SELFTEST = args.includes('--selftest');
const PRINT_FINGERPRINT = args.includes('--print-fingerprint');

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

/** 任务 4.1：工具层前端源码集合的内容指纹。
 *
 *  为什么需要：checkBuildTag 只比「main.ts 与 index.html 的徽标是否相等」，改了业务源码却忘递增
 *  徽标、两个旧值仍相等时门禁全绿，人工复验会跑在旧 DOM 上（I16 已命中三次，review P1-3）。
 *  这里把指纹与确定的源码清单绑定：只改业务源码而保留旧指纹 ⇒ 判红；新增工具层源文件未纳入
 *  清单 ⇒ 判红；更新源码、递增徽标并同步登记指纹后才放行。
 *
 *  自引用处理：指纹常量本身在门禁脚本内（不在源码清单里），改它不改变被计算的内容；
 *  徽标两处（BUILD_TAG / data-build）在规范化时替换为固定占位符，不参与内容哈希——
 *  因此指纹只反映业务源码，而「版本 ↔ 指纹」的绑定由下面 EXPECTED_TOOL_SOURCE.tag 承担。
 *
 *  维护规则：改工具层前端源码 ⇒ 递增 BUILD_TAG 与 data-build、把新徽标与新指纹一起登记到
 *  EXPECTED_TOOL_SOURCE；新增 src/ 下源文件 ⇒ 同时加入 TOOL_SOURCE_FILES 清单。 */
const TOOL_SOURCE_FILES = [
  'index.html',
  'status.html',
  'src/main.ts',
  'src/coords.ts',
  'vite.config.ts',
];

// 当前工具层版本 ↔ 业务源码指纹。两者必须与磁盘实际值同时匹配；单独改任一处都会判红。
// 新指纹用 `node scripts/check-build.mjs --print-fingerprint` 取得。
const EXPECTED_TOOL_SOURCE = {
  tag: 'v20261118-4',
  sha256: 'a18d1b8156fd13a9d1b6f7cb817d3d60f4bb38509213e548baee9136e494e697',
};

/** 规范化：换行统一为 LF、行尾空白剔除、徽标两处替换为占位符。
 *  纯函数，selftest 用内存样本直接校验，不改磁盘。 */
function normalizeToolSource(text) {
  return text
    .replace(/\r\n?/g, '\n')
    .split('\n')
    .map((line) => line.replace(/[ \t]+$/, '').replace(/const BUILD_TAG = "[^"]*"/, 'const BUILD_TAG = "<BUILD>"')
      .replace(/data-build="[^"]*"/, 'data-build="<BUILD>"'))
    .join('\n');
}

/** 按固定清单顺序拼接「相对路径 \0 规范化内容」，算 sha256。 */
function computeToolFingerprint(fileMap) {
  const hash = createHash('sha256');
  for (const rel of TOOL_SOURCE_FILES) {
    const text = fileMap[rel];
    if (text === undefined) return null;
    hash.update(rel);
    hash.update('\0');
    hash.update(normalizeToolSource(text));
    hash.update('\0');
  }
  return hash.digest('hex');
}

/** src/ 下所有相对路径，用于检测「新增模块未纳入清单」。 */
function listSrcFiles(fileMap) {
  return Object.keys(fileMap).filter((p) => p.startsWith('src/')).sort();
}

/** 指纹判据核心：返回失败原因数组（空数组=通过）。纯函数，selftest 复用。 */
function evaluateToolSource(fileMap, expected, diskSrcFiles) {
  const reasons = [];
  const actual = computeToolFingerprint(fileMap);
  if (actual === null) {
    reasons.push('源码清单有文件缺失或无法读取');
  } else if (actual !== expected.sha256) {
    reasons.push(`业务源码指纹不符：期望 ${expected.sha256}，实际 ${actual}`);
  }
  if (expected.tag !== (/\bconst BUILD_TAG = "([^"]+)"/.exec(fileMap['src/main.ts'] ?? '')?.[1] ?? null)) {
    reasons.push(`登记的徽标版本与源码不一致：期望 ${expected.tag}`);
  }
  const unlisted = diskSrcFiles.filter((p) => !TOOL_SOURCE_FILES.includes(p));
  if (unlisted.length) reasons.push(`src/ 下有未纳入清单的模块：${unlisted.join(', ')}`);
  return reasons;
}

/** 读磁盘上确定清单文件 + src/ 实际文件列表，交给纯函数判定。 */
function readToolSourceFromDisk() {
  const desktop = path.join(ROOT, 'apps', 'desktop');
  const fileMap = {};
  for (const rel of TOOL_SOURCE_FILES) {
    const abs = path.join(desktop, ...rel.split('/'));
    try {
      fileMap[rel] = fs.readFileSync(abs, 'utf8');
    } catch (error) {
      console.error(`[check-build] ✗ 读取工具层源码失败：${rel}（${error.message}）`);
      return null;
    }
  }
  const diskSrcFiles = [];
  const srcDir = path.join(desktop, 'src');
  try {
    for (const entry of fs.readdirSync(srcDir, { withFileTypes: true })) {
      if (entry.isFile()) diskSrcFiles.push(`src/${entry.name}`);
    }
  } catch (error) {
    console.error(`[check-build] ✗ 扫描 src/ 失败：${error.message}`);
    return null;
  }
  return { fileMap, diskSrcFiles };
}

function checkToolSourceFingerprint() {
  const disk = readToolSourceFromDisk();
  if (!disk) return false;
  const reasons = evaluateToolSource(disk.fileMap, EXPECTED_TOOL_SOURCE, disk.diskSrcFiles);
  for (const reason of reasons) console.error(`[check-build] ✗ 工具层源码指纹：${reason}`);
  if (!reasons.length) console.log(`[check-build] ✓ 工具层源码指纹一致：${EXPECTED_TOOL_SOURCE.tag} / ${EXPECTED_TOOL_SOURCE.sha256.slice(0, 12)}…`);
  return reasons.length === 0;
}

/** selftest 反例：验证指纹机制本身——改业务源码必须判红、改徽标不污染指纹、未纳入模块判红。
 *  全部用内存样本，不动磁盘；任何一条不符 ⇒ selftest 退出非 0（失败传播）。 */
function sourceFingerprintSelfTest() {
  const base = {
    'index.html': '<span id="build-tag" data-build="vTEST-1">x</span>\n<div>workspace</div>\n',
    'status.html': '<html><body>status</body></html>\n',
    'src/main.ts': 'const BUILD_TAG = "vTEST-1";\nexport function business() { return 1; }\n',
    'src/coords.ts': 'export const scale = 1;\n',
    'vite.config.ts': 'export default {};\n',
  };
  const srcFiles = listSrcFiles(base);
  const expected = { tag: 'vTEST-1', sha256: computeToolFingerprint(base) };
  const cases = [
    { name: '基准样本应通过', fileMap: base, expected, srcFiles, wantOk: true },
    {
      name: '业务源码变更但保留旧指纹必须判红',
      fileMap: { ...base, 'src/coords.ts': 'export const scale = 2;\n' },
      expected, srcFiles, wantOk: false,
    },
    {
      name: '仅改徽标不污染业务源码指纹（自引用字段已排除）',
      fileMap: {
        ...base,
        'src/main.ts': 'const BUILD_TAG = "vTEST-9";\nexport function business() { return 1; }\n',
        'index.html': '<span id="build-tag" data-build="vTEST-9">x</span>\n<div>workspace</div>\n',
      },
      expected: { ...expected, tag: 'vTEST-9' },
      srcFiles, wantOk: true,
    },
    {
      name: '业务源码变更后同时更新指纹才通过',
      fileMap: { ...base, 'src/coords.ts': 'export const scale = 2;\n' },
      expected: { tag: 'vTEST-1', sha256: computeToolFingerprint({ ...base, 'src/coords.ts': 'export const scale = 2;\n' }) },
      srcFiles, wantOk: true,
    },
    {
      name: 'src/ 新增模块未纳入清单必须判红',
      fileMap: { ...base, 'src/newModule.ts': 'export const x = 1;\n' },
      expected, srcFiles: listSrcFiles({ ...base, 'src/newModule.ts': '' }), wantOk: false,
    },
  ];
  let ok = true;
  for (const c of cases) {
    const gotOk = evaluateToolSource(c.fileMap, c.expected, c.srcFiles).length === 0;
    const pass = gotOk === c.wantOk;
    console.log(`[check-build] ${pass ? '✓' : '✗'} 指纹反例：${c.name}`);
    if (!pass) ok = false;
  }
  return ok;
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

/** Tauri 2 会把 Rust 命令的 snake_case 参数映射为前端 camelCase 键。
 *  2026-11-15 1.1 首轮人工复验报 `missing required key startUrl` ⇒ 前端误传 `start_url`，
 *  把「靠记性记参数名」升级成构建门禁断言，避免下次改回去又只靠 GUI 发现。 */
function checkCommandArgs() {
  const ts = fs.readFileSync(path.join(ROOT, 'apps', 'desktop', 'src', 'main.ts'), 'utf8');
  const rs = fs.readFileSync(path.join(ROOT, 'apps', 'desktop', 'src-tauri', 'src', 'lib.rs'), 'utf8');
  const invokeBlock = /invoke<ConfigStateView>\("config_create_project",\s*\{([\s\S]*?)\}\)/.exec(ts)?.[1] ?? '';
  const checks = [
    [
      'Rust 命令参数保持 snake_case',
      rs.includes('start_url: String') && rs.includes('preset_id: String'),
    ],
    [
      '前端 invoke 使用 camelCase 键',
      /\bstartUrl\b/.test(invokeBlock) && /\bpresetId\b/.test(invokeBlock),
    ],
    [
      '前端不再传 snake_case 键',
      !/\bstart_url\s*:/.test(invokeBlock) && !/\bpreset_id\s*:/.test(invokeBlock),
    ],
  ];
  let ok = true;
  for (const [name, pass] of checks) {
    console.log(`[check-build] ${pass ? '✓' : '✗'} 命令参数不变量：${name}`);
    if (!pass) ok = false;
  }
  return ok;
}

function main() {
  console.log(
    `Layress 构建门禁 · 模式 ${PRINT_FINGERPRINT ? 'print-fingerprint' : SELFTEST ? 'selftest' : 'all'} · ${process.platform}/${process.arch} · Node ${process.version}`,
  );
  if (PRINT_FINGERPRINT) {
    const disk = readToolSourceFromDisk();
    if (!disk) process.exit(1);
    const fingerprint = computeToolFingerprint(disk.fileMap);
    console.log(`工具层业务源码指纹（排除徽标字段）：${fingerprint}`);
    console.log(`当前徽标：${/\bconst BUILD_TAG = "([^"]+)"/.exec(disk.fileMap['src/main.ts'])?.[1] ?? '缺失'}`);
    console.log(`登记格式：{ tag: '<当前徽标>', sha256: '${fingerprint}' }`);
    process.exit(0);
  }
  if (!SELFTEST && (!checkBuildTag() || !checkFixtureInvariants() || !checkCommandArgs() || !checkToolSourceFingerprint())) {
    process.exit(1);
  }
  const steps = SELFTEST
    ? [
        { name: '正例：恒成功命令应通过', cmd: process.execPath, args: ['-e', 'process.exit(0)'], cwd: ROOT },
        { name: '反例：恒失败命令必须被拦', cmd: process.execPath, args: ['-e', 'process.exit(7)'], cwd: ROOT },
      ]
    : realSteps();
  const results = steps.map(runStep);

  if (SELFTEST) {
    const exitPropagation = results[0]?.ok === true && results[1]?.ok === false;
    const fingerprintRefutations = sourceFingerprintSelfTest();
    const passed = exitPropagation && fingerprintRefutations;
    console.log(
      passed
        ? '自测通过：成功命令放行、失败命令未被静默吞掉；指纹反例全拦'
        : '自测失败：退出码传播或指纹反例口径被破坏',
    );
    process.exit(passed ? 0 : 1);
  }

  const failed = results.filter((r) => !r.ok);
  console.log(`结论：${failed.length} 项失败 ⇒ ${failed.length ? '拒绝' : '放行'}`);
  for (const f of failed) console.error(`  ✗ ${f.name}：${f.detail}`);
  process.exit(failed.length ? 1 : 0);
}

main();
