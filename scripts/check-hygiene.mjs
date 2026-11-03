#!/usr/bin/env node
// Layress 仓库洁净度门禁
//
// 用法：
//   node scripts/check-hygiene.mjs            # 检查全部已跟踪文件（CI 用）
//   node scripts/check-hygiene.mjs --staged   # 只检查暂存区（pre-commit 用）
//   node scripts/check-hygiene.mjs --strict   # 把警告也判为失败
//
// 存在理由：.gitignore 是默认放行的，一个 `git add -f` 就能把 300MB 安装包写进历史，
// 而历史里的 blob 删不掉。所以拒绝必须由**脚本**执行，并且 pre-commit 与 CI 跑同一条。

import { execFileSync } from 'node:child_process';
import { readFileSync, existsSync } from 'node:fs';
import path from 'node:path';

const args = process.argv.slice(2);
const MODE = args.includes('--staged') ? 'staged' : 'all';
const STRICT = args.includes('--strict');

const MAX_BYTES = 512 * 1024;
const WARN_BYTES = 256 * 1024;

// 出现即失败的路径片段：本仓认定的"垃圾场"与本机配置。
const FORBIDDEN_PATHS = [
  '.layress/', 'artifacts/', 'output/', 'node_modules/', 'dist/', 'out/', 'build/',
  'release/', '.tmp', '.pnpm-store/',
  '.workbuddy/', '.trae/', '.cursor/', '.claude/', '.vscode/', '.idea/', '.qoder/', 'CLAUDE.md',
  'docs/drafts/',
];
const FORBIDDEN_EXACT = ['.env', '.mcp.json'];
const FORBIDDEN_PREFIX = ['.env.'];
const FORBIDDEN_EXT = new Set([
  '.msi', '.exe', '.dll', '.node', '.pdb', '.lib', '.7z', '.rar', '.iso', '.zip',
  '.gz', '.tgz', '.tar', '.dmg', '.deb', '.rpm', '.AppImage', '.mp4', '.webm',
]);

const SECRET_RULES = [
  ['私钥块', /-----BEGIN (?:[A-Z ]+ )?PRIVATE KEY-----/],
  ['OpenAI 形态 key', /\bsk-[A-Za-z0-9_-]{16,}\b/],
  ['GitHub token', /\b(?:ghp|gho|ghu|ghs)_[A-Za-z0-9]{20,}\b/],
  ['AWS AccessKeyId', /\bAKIA[0-9A-Z]{16}\b/],
  ['Slack token', /\bxox[baprs]-[A-Za-z0-9-]{10,}\b/],
  ['赋值形态密钥', /\b(?:api[_-]?key|secret|token|password)\b\s*[:=]\s*['"][^'"\s]{12,}['"]/i],
];

// 机器特定绝对路径：换台机器就失效，也是"本机配置被误提交"的典型指纹。文档不参与本项检查。
const ABS_PATH = /\b[A-Za-z]:[\\/]{2}(?:Users|home|Program Files)\b|[A-Za-z]:\\\\Users\\\\/;

// 不变量 I12「不碰 C 盘」的机器落点：产品代码里出现系统用户目录形态即判红。
// 语义（"这是默认值还是用户显式选的"）脚本判不了，所以只拦最典型的写法，其余靠 review。
const SYSTEM_DIR_RULES = [
  ['%APPDATA% / %LOCALAPPDATA%', /%(?:APPDATA|LOCALAPPDATA)%/],
  ['读取用户目录环境变量', /process\.env\.(?:APPDATA|LOCALAPPDATA|USERPROFILE|HOME)\b/],
  ['Windows 用户目录', /[A-Za-z]:[\\/]{2}Users[\\/]/],
  ['macOS 支持目录', /Library\/(?:Application Support|Preferences)[\\/]/],
  ['Electron 默认 userData', /getPath\(\s*['"](?:userData|appData)['"]\s*\)/],
  ['Node 主目录默认值', /os\.homedir\(\)\s*,\s*['"]\./],
];

const fails = [];
const warns = [];
const add = (bag, rule, detail, hint) => bag.push({ rule, detail, hint });

function git(gitArgs, opts = {}) {
  return execFileSync('git', gitArgs, { encoding: 'utf8', maxBuffer: 64 << 20, ...opts });
}

function globToRe(glob) {
  let re = '';
  for (let i = 0; i < glob.length; i += 1) {
    const c = glob[i];
    if (c === '*') {
      if (glob[i + 1] === '*') {
        i += 1;
        if (glob[i + 1] === '/') { i += 1; re += '(?:[^/]*/)?'; } else re += '.*';
      } else re += '[^/]*';
    } else if (c === '?') re += '[^/]';
    else re += c.replace(/[.*+?^${}()|[\]\\]/g, '\\$&');
  }
  // 不含 / 的模式按 .gitignore 语义匹配任意层级下的同名文件
  if (!glob.includes('/')) re = `(?:.*/)?${re}`;
  return new RegExp(`^${re}$`);
}

function loadAllowlist(root) {
  const file = path.join(root, '.git-hygiene-allowlist');
  if (!existsSync(file)) return [];
  const out = [];
  for (const raw of readFileSync(file, 'utf8').split(/\r?\n/)) {
    const line = raw.trim();
    if (!line || line.startsWith('#')) continue;
    const hash = line.indexOf('#');
    const glob = (hash === -1 ? line : line.slice(0, hash)).trim();
    const note = hash === -1 ? '' : line.slice(hash + 1).trim();
    if (!glob) continue;
    if (!note) add(fails, '豁免无理由', `.git-hygiene-allowlist: ${glob}`, '豁免必须写一句话理由，否则等于没有审计线索');
    out.push({ glob, note, re: globToRe(glob), maxBytes: MAX_BYTES, used: false });
    const size = /(?:^|\s)size=(\d+)([KM])/i.exec(note);
    if (size) entry(out.length - 1).maxBytes = Number(size[1]) * (size[2].toUpperCase() === 'M' ? 1 << 20 : 1 << 10);
  }
  return out;
  function entry(i) { return out[i]; }
}

function findExemption(list, file) {
  return list.find((e) => e.re.test(file));
}

function collectFiles(root) {
  const nul = (s) => s.split('\0').filter(Boolean);
  const indexRows = nul(git(['ls-files', '-s', '-z']))
    .map((row) => {
      const [meta, p] = row.split('\t');
      const [, sha, stage] = meta.split(' ');
      return { file: p.replace(/\\/g, '/'), sha, stage };
    })
    .filter((r) => r.stage === '0');

  if (MODE === 'all') return indexRows;

  const staged = new Set(nul(git(['diff', '--cached', '--name-only', '--diff-filter=ACMR', '-z']))
    .map((p) => p.replace(/\\/g, '/')));
  return indexRows.filter((r) => staged.has(r.file));
}

function sizesOf(rows) {
  if (rows.length === 0) return new Map();
  const out = git(['cat-file', '--batch-check=%(objectsize)'], { input: rows.map((r) => r.sha).join('\n') });
  const lines = out.split('\n');
  return new Map(rows.map((r, i) => [r.file, Number(lines[i]) >= 0 ? Number(lines[i]) : Number.NaN]));
}

function looksBinary(buf) {
  const n = Math.min(buf.length, 8000);
  for (let i = 0; i < n; i += 1) if (buf[i] === 0) return true;
  return false;
}

function main() {
  let root;
  try {
    root = git(['rev-parse', '--show-toplevel']).trim();
  } catch {
    console.error('✗ 不在 git 仓库内，无法执行洁净度门禁。');
    process.exit(1);
  }

  const allow = loadAllowlist(root);
  const rows = collectFiles(root);
  const sizes = sizesOf(rows);

  for (const { file } of rows) {
    const ex = findExemption(allow, file);
    if (ex) ex.used = true;
    const base = path.posix.basename(file);
    const ext = path.extname(file);

    if (!ex) {
      const hitPath = FORBIDDEN_PATHS.find((p) => file.includes(p) || file === p.replace(/\/$/, ''));
      const hitExact = FORBIDDEN_EXACT.includes(base) || FORBIDDEN_PREFIX.some((p) => base.startsWith(p));
      if (hitExact) add(fails, '本机配置/凭据', file, '`.env*`、`.mcp.json` 一类永不入库；需要模板就提交 `.env.example`');
      else if (hitPath) add(fails, '垃圾场路径', file, '该文件属于本地产物或本机记忆，移到 docs/ 或 datasets/ 的正确位置，或留在 artifacts/ 与 .layress/');
      else if (FORBIDDEN_EXT.has(ext)) add(fails, '二进制/归档后缀', file, `${ext} 不入历史；先登记 .git-hygiene-allowlist 理由再 add -f`);
    }

    const bytes = sizes.get(file);
    const cap = ex ? ex.maxBytes : MAX_BYTES;
    if (Number.isFinite(bytes)) {
      if (bytes > cap) add(fails, '体积超限', `${file}（${(bytes / 1024 / 1024).toFixed(2)} MB > ${(cap / 1024 / 1024).toFixed(2)} MB）`, '拆分内容、改为可下载产物，或抬高豁免行的 size=');
      else if (bytes > WARN_BYTES && !ex) add(warns, '体积偏大', `${file}（${Math.round(bytes / 1024)} KB）`, '确认它真该进历史');
    }
    if (bytes === 0) add(warns, '空文件', file, '空占位请改用目录内一份说明性 README');

    if (ex || file.includes('/fixtures/')) continue;
    const abs = path.join(root, file);
    if (!existsSync(abs)) continue;
    let buf;
    try { buf = readFileSync(abs); } catch { continue; }
    if (buf.length > 2 << 20 || looksBinary(buf)) continue;
    const text = buf.toString('utf8');
    const isDoc = file.endsWith('.md');
    const isProductCode = /^(?:packages|apps)\//.test(file);

    // 凭据形态连文档一起查：README 里贴一个真 key 和历史里埋一个同样删不掉
    for (const [name, re] of SECRET_RULES) {
      const m = re.exec(text);
      if (m) add(fails, '凭据形态', `${file}: ${name} → ${m[0].slice(0, 24)}…`, '移除真实凭据；密钥只由用户在应用内填写，仓库里留占位符');
    }
    if (isDoc) continue; // 文档里写 `C:\Users\…` 是正当讨论，路径类检查不看文档

    // 策略与模板文本存在的意义就是**禁止**这些路径（openspec 不变量、Issue 表单、门禁规则表自身）。
    // 对它们报警告只会训练贡献者忽略警告，所以本项跳过；凭据扫描仍然照查。
    const isPolicyText = /^(?:openspec\/|\.github\/|\.gitee\/|\.workflow\/|scripts\/check-hygiene\.mjs$)/.test(file);
    if (isPolicyText) continue;

    for (const [name, re] of SYSTEM_DIR_RULES) {
      const m = re.exec(text);
      if (!m) continue;
      const detail = `${file}: ${name} → ${m[0].slice(0, 32)}`;
      if (isProductCode) {
        add(fails, '系统用户目录', detail, '违反不变量 I12「不碰 C 盘」：数据/缓存/临时文件必须落在数据根之下；不可写时报错并要求 --data-dir，禁止静默回落');
      } else {
        add(warns, '系统用户目录', detail, '确认这是测试夹具或脚本临时目录，不是产品默认路径');
      }
    }

    const absHit = ABS_PATH.exec(text);
    if (absHit) {
      const detail = `${file}: ${absHit[0]}`;
      if (/\.(json|ts|tsx|js|mjs|cjs|ya?ml)$/.test(file)) add(fails, '机器绝对路径', detail, '改用相对数据根的路径或运行时解析的目录');
      else add(warns, '机器绝对路径', detail, '示例路径请标注为本机示例');
    }
  }

  const checked = new Set(rows.map((r) => r.file));
  const ignoredTracked = git(['ls-files', '-i', '-c', '--exclude-standard', '-z'])
    .split('\0').filter(Boolean)
    .map((p) => p.replace(/\\/g, '/'))
    .filter((p) => checked.has(p) && !findExemption(allow, p));
  for (const file of ignoredTracked) {
    add(fails, '忽略却已跟踪', file, '.gitignore 说不要、历史里却留着：`git rm --cached <file>` 后重新提交');
  }

  // 只在 --all 下判定"豁免失效"：staged 模式里本次提交没碰到那个文件，不代表豁免是死的
  if (MODE === 'all') {
    for (const e of allow.filter((x) => !x.used)) {
      add(warns, '豁免失效', e.glob, '没有任何文件命中，删掉这行以免豁免清单变成考古现场');
    }
  }

  const report = (bag, mark) => {
    if (bag.length === 0) return;
    const byRule = new Map();
    for (const item of bag) {
      if (!byRule.has(item.rule)) byRule.set(item.rule, []);
      byRule.get(item.rule).push(item);
    }
    console.log(`\n${mark} ${bag.length} 项`);
    for (const [rule, items] of byRule) {
      console.log(`\n  【${rule}】`);
      for (const it of items.slice(0, 20)) console.log(`    - ${it.detail}\n      ↳ ${it.hint}`);
      if (items.length > 20) console.log(`    … 另有 ${items.length - 20} 项同类`);
    }
  };

  console.log(`Layress 洁净度门禁 · 模式 ${MODE} · 待查文件 ${rows.length} 个`);
  report(fails, '✗ 失败');
  report(warns, '! 警告');

  const failed = fails.length > 0 || (STRICT && warns.length > 0);
  console.log(`\n结论：失败 ${fails.length} / 警告 ${warns.length}${STRICT ? '（--strict：警告计入失败）' : ''} ⇒ ${failed ? '不放行' : '放行'}`);
  process.exit(failed ? 1 : 0);
}

main();
