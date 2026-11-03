#!/usr/bin/env node
// Layress 双语完整性门禁
//
// 用法：
//   node scripts/check-i18n.mjs            检查 i18n/docs-manifest.json 登记的全部成对文档
//   node scripts/check-i18n.mjs --selftest 反证：人为构造缺对/断链/漂移，门禁必须判红
//
// 存在理由：口径写在文档里而没人执行，英文版本就会悄悄停在旧口径上——读者以为它说的是当前事实，
// 比没有英文版更糟。所以"成对"由脚本判定，且 --selftest 保证门禁本身不是假绿。
//
// 本门禁只管**文档层**。产品字符串层（zh-CN / en-US 资源目录与批次归属）等 packages/ 有代码后
// 再接进来，口径见 i18n/README.md。

import { execFileSync } from 'node:child_process';
import { existsSync, mkdirSync, mkdtempSync, readdirSync, readFileSync, rmSync, writeFileSync } from 'node:fs';
import { tmpdir } from 'node:os';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const SELFTEST = process.argv.includes('--selftest');
const ROOT = path.join(path.dirname(fileURLToPath(import.meta.url)), '..');

// 英文版本低于中文版四成字节 ⇒ 疑似只翻了一半（中文信息密度高，正常翻译会膨胀不会缩水这么多）
const MIN_RATIO = 0.4;

export const enOf = (zh) => zh.replace(/\.md$/, '.en.md');
export const zhOf = (en) => en.replace(/\.en\.md$/, '.md');

/** 去掉围栏代码块与行内代码后的正文，以及逐行的「是否在围栏内」标记。 */
function scan(text) {
  const stripped = text.replace(/```[\s\S]*?```/g, '').replace(/`[^`\n]*`/g, '');
  let inFence = false;
  const lines = text.split(/\r?\n/).map((line) => {
    if (/^\s*(?:```|~~~)/.test(line)) inFence = !inFence;
    return { line, inFence };
  });
  return { stripped, lines };
}

function headings2(text) {
  const { lines } = scan(text);
  return lines.filter(({ line, inFence }) => !inFence && /^##\s+\S/.test(line)).map(({ line }) => line.slice(3).trim());
}

function links(text) {
  const { stripped } = scan(text);
  const md = [...stripped.matchAll(/\[[^\]]*\]\(\s*<?([^)\s>]+)>?(?:\s+"[^"]*")?\s*\)/g)].map((m) => m[1]);
  // 语言入口用的是 HTML <a href>（GitHub 上居中排版），必须一起抽，否则成对检查会漏判
  const html = [...stripped.matchAll(/href=["']([^"']+)["']/g)].map((m) => m[1]);
  return [...md, ...html];
}

const fenceCount = (text) => Math.floor((text.match(/^\s*```/gm) || []).length / 2);
const isExternal = (href) => /^(?:https?:|mailto:|#|data:)/.test(href) || href.startsWith('/');

/**
 * git 可用时只看已跟踪文件（避免把 artifacts/ 的临时副本算成孤儿）；
 * 仓库尚未 `git init` 时退化成文件系统扫描，同样跳过垃圾场。
 */
function listFiles(root, predicate) {
  try {
    return execFileSync('git', ['ls-files', '-z'], {
      cwd: root, encoding: 'utf8', stdio: ['ignore', 'pipe', 'ignore'],
    })
      .split('\0').filter(Boolean).map((p) => p.replace(/\\/g, '/')).filter(predicate);
  } catch {
    const skip = new Set(['.git', 'node_modules', 'artifacts', 'output', '.layress', '.pnpm-store', 'dist']);
    const out = [];
    const walk = (rel) => {
      for (const entry of readDirSafe(path.join(root, rel))) {
        const p = rel ? `${rel}/${entry.name}` : entry.name;
        if (entry.isDirectory()) { if (!skip.has(entry.name) && !entry.name.startsWith('.')) walk(p); }
        else if (predicate(p)) out.push(p);
      }
    };
    walk('');
    return out;
  }
}

function readDirSafe(abs) {
  try { return readdirSync(abs, { withFileTypes: true }); } catch { return []; }
}

/** 主校验：返回问题列表 [{code, where, message}]。 */
export function validateDocs(root, docs) {
  const errors = [];
  const add = (code, where, message) => errors.push({ code, where, message });
  const registered = new Set(docs);

  for (const zh of docs) {
    const en = enOf(zh);
    const zhPath = path.join(root, zh);
    const enPath = path.join(root, en);

    if (!existsSync(zhPath)) { add('ZH_MISSING', zh, '登记表里的中文原文不存在'); continue; }
    if (!existsSync(enPath)) { add('TWIN_MISSING', zh, `缺英文双胞胎 ${en}；成对文档必须同轮提交`); continue; }

    const zhText = readFileSync(zhPath, 'utf8');
    const enText = readFileSync(enPath, 'utf8');

    if (!/^\s*#\s+\S/m.test(zhText)) add('NO_TITLE', zh, '没有一级标题');
    if (!/^\s*#\s+\S/m.test(enText)) add('NO_TITLE', en, '没有一级标题');

    // 语言入口：中文要链到英文，英文要链回中文
    const pairs = (file, target) => {
      const want = path.posix.basename(target);
      return links(file).some((h) => path.posix.basename(h) === want);
    };
    if (!pairs(zhText, en)) add('NO_LANG_LINK', zh, `正文没有指向 ${path.posix.basename(en)} 的语言入口`);
    if (!pairs(enText, zh)) add('NO_LANG_LINK', en, `正文没有指回 ${path.posix.basename(zh)} 的语言入口`);

    // 结构对齐
    const zhH = headings2(zhText);
    const enH = headings2(enText);
    if (zhH.length !== enH.length) {
      add('HEADING_DRIFT', en, `二级标题 ${enH.length} 个 vs 中文 ${zhH.length} 个 ⇒ 有段落没跟着翻`);
    }
    if (fenceCount(zhText) !== fenceCount(enText)) {
      add('CODE_BLOCK_DRIFT', en, `围栏代码块 ${fenceCount(enText)} 组 vs 中文 ${fenceCount(zhText)} 组`);
    }

    // 体积漂移
    const ratio = Buffer.byteLength(enText) / Math.max(1, Buffer.byteLength(zhText));
    if (ratio < MIN_RATIO) add('THIN_TWIN', en, `英文仅为中文的 ${(ratio * 100).toFixed(0)}% ⇒ 疑似节译或漏段`);

    // 双向断链
    for (const [file, text] of [[zh, zhText], [en, enText]]) {
      for (const href of links(text)) {
        if (isExternal(href)) continue;
        const target = path.resolve(path.join(root, path.posix.dirname(file)), href.split('#')[0]);
        if (!existsSync(target)) add('BROKEN_LINK', file, `相对链接解析不到文件：${href}`);
      }
    }
  }

  // 孤儿英文：有 .en.md 却没登记，等于绕开门禁
  for (const en of listFiles(root, (p) => p.endsWith('.en.md'))) {
    if (!registered.has(zhOf(en))) {
      add('TWIN_UNREGISTERED', en, '有英文版本但未登记进 docs-manifest.json，单边改动不会触发门禁');
    }
  }

  return errors;
}

// ── 反证自测：每条规则必须能判红，否则门禁是假绿 ────────────────
function writeFixture(into, p, body) {
  const abs = path.join(into, p);
  mkdirSync(path.dirname(abs), { recursive: true });
  writeFileSync(abs, body);
}

function selftest() {
  const dir = mkdtempSync(path.join(tmpdir(), 'layress-i18n-'));
  const write = (p, body) => writeFixture(dir, p, body);
  const failures = [];
  const expect = (name, docs, code) => {
    const errs = validateDocs(dir, docs);
    if (!errs.some((e) => e.code === code)) failures.push(`反例未判红：${name}（期望 ${code}）`);
  };

  write('a.md', '# 标题 A\n\n[English](a.en.md)\n\n## 一\n\n内容\n');
  write('a.en.md', '# Title A\n\n[简体中文](a.md)\n\n## One\n\nContent long enough to look proportionate.\n');
  write('b.md', '# 标题 B\n\n[English](b.en.md)\n\n## 一\n\n内容\n');
  write('c.md', '# 标题 C\n\n[English](c.en.md)\n\n内容见 [缺失目标](missing.md)\n');
  write('c.en.md', '# Title C\n\n[简体中文](c.md)\n\n内容指向缺失\n');
  write('d.md', '# 标题 D\n\n[English](d.en.md)\n\n## 一\n\n## 二\n\n内容比较长一些\n');
  write('d.en.md', '# Title D\n\n[简体中文](d.md)\n\n## One\n\nShort.\n');
  write('nolang.md', '# 标题 N\n\n内容\n');
  write('nolang.en.md', '# Title N\n\nContent that is clearly longer than the source would suggest\n');
  write('orphan.en.md', '# Orphan\n\n[中文](orphan.md)\n');

  expect('缺英文双胞胎', ['b.md'], 'TWIN_MISSING');
  expect('中文原文不存在', ['nope.md'], 'ZH_MISSING');
  expect('相对链接断', ['c.md'], 'BROKEN_LINK');
  expect('标题数漂移', ['d.md'], 'HEADING_DRIFT');
  expect('语言入口缺失', ['nolang.md'], 'NO_LANG_LINK');
  expect('英文孤儿未登记', ['orphan.md'], 'TWIN_UNREGISTERED');

  const ok = validateDocs(dir, ['a.md']);
  // 正例必须用干净目录：同目录里其他反例的孤儿英文会合法地判红，混在一起等于自测失真
  const clean = mkdtempSync(path.join(tmpdir(), 'layress-i18n-ok-'));
  writeFixture(clean, 'a.md', '# 标题 A\n\n[English](a.en.md)\n\n## 一\n\n内容\n');
  writeFixture(clean, 'a.en.md', '# Title A\n\n[简体中文](a.md)\n\n## One\n\nContent long enough to look proportionate.\n');
  const okClean = validateDocs(clean, ['a.md']);
  if (ok.length && !ok.every((e) => e.code === 'TWIN_UNREGISTERED')) {
    failures.push(`正例被误判：a.md ⇒ ${ok.map((e) => `${e.code}:${e.where}`).join(', ')}`);
  }
  if (okClean.length) {
    failures.push(`干净正例被误判 ⇒ ${okClean.map((e) => `${e.code}:${e.where}`).join(', ')}`);
  }

  rmSync(dir, { recursive: true, force: true });
  rmSync(clean, { recursive: true, force: true });
  if (failures.length) {
    for (const f of failures) console.error(`[check-i18n] --selftest FAIL: ${f}`);
    process.exit(1);
  }
  console.log('[check-i18n] --selftest 通过（6 个反例 + 1 个正例）');
  process.exit(0);
}

function main() {
  const manifestPath = path.join(ROOT, 'i18n', 'docs-manifest.json');
  if (!existsSync(manifestPath)) {
    console.error('✗ 缺 i18n/docs-manifest.json：双语门禁没有登记表就等于不存在。');
    process.exit(1);
  }
  const manifest = JSON.parse(readFileSync(manifestPath, 'utf8'));
  const docs = (manifest.docs ?? []).map((d) => (typeof d === 'string' ? d : d.zh));
  const errors = validateDocs(ROOT, docs);

  console.log(`Layress 双语门禁 · 登记 ${docs.length} 篇成对文档`);
  for (const code of [...new Set(errors.map((e) => e.code))]) {
    console.log(`\n✗ 【${code}】`);
    for (const e of errors.filter((x) => x.code === code)) console.log(`  - ${e.where}\n    ↳ ${e.message}`);
  }
  console.log(`\n结论：问题 ${errors.length} ⇒ ${errors.length ? '不放行' : '放行'}`);
  process.exit(errors.length ? 1 : 0);
}

if (SELFTEST) selftest(); else main();
