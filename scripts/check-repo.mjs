#!/usr/bin/env node
import { execFileSync, spawnSync } from 'node:child_process';
import { mkdirSync, mkdtempSync, rmSync } from 'node:fs';
import path from 'node:path';
import { fileURLToPath } from 'node:url';

const ROOT = path.resolve(path.dirname(fileURLToPath(import.meta.url)), '..');
// 门禁链跨平台：本地 macOS (zsh) 与 CI ubuntu 跑同一条命令链；只用 node:path 与 git 子进程，不依赖 shell 方言。
let snapshot;

try {
  let checkRoot = ROOT;
  let env = process.env;
  if (process.argv.includes('--staged')) {
    const artifacts = path.join(ROOT, 'artifacts');
    mkdirSync(artifacts, { recursive: true });
    snapshot = mkdtempSync(path.join(artifacts, 'staged-checks-'));
    const prefix = `${snapshot.replace(/\\/g, '/')}/`;
    execFileSync('git', ['checkout-index', '--all', `--prefix=${prefix}`], { cwd: ROOT, stdio: 'inherit' });
    const gitDir = execFileSync('git', ['rev-parse', '--absolute-git-dir'], { cwd: ROOT, encoding: 'utf8' }).trim();
    const index = execFileSync('git', ['rev-parse', '--git-path', 'index'], { cwd: ROOT, encoding: 'utf8' }).trim();
    checkRoot = snapshot;
    env = { ...process.env, GIT_DIR: gitDir, GIT_WORK_TREE: snapshot, GIT_INDEX_FILE: path.resolve(ROOT, index) };
  }
  // 每项可指定 cwd / env。构建类检查必须在真实检出目录跑：--staged 的索引快照
  // 不含 node_modules / target，无法编译；洁净度 / 链接 / 双语仍按索引快照。
  const checks = [
    { script: 'check-hygiene.mjs', args: ['--all'] },
    { script: 'check-hygiene.mjs', args: ['--selftest'] },
    { script: 'check-i18n.mjs', args: ['--selftest'] },
    { script: 'check-i18n.mjs', args: ['--links'] },
    { script: 'check-i18n.mjs', args: [] },
    { script: 'check-build.mjs', args: ['--all'], cwd: ROOT, env: process.env },
    { script: 'check-build.mjs', args: ['--selftest'], cwd: ROOT, env: process.env },
  ];
  for (const { script, args, cwd, env: stepEnv } of checks) {
    const base = cwd ?? checkRoot;
    const result = spawnSync(process.execPath, [path.join(base, 'scripts', script), ...args], {
      cwd: base, env: stepEnv ?? env, stdio: 'inherit',
    });
    if (result.error) throw result.error;
    if (result.status !== 0) {
      process.exitCode = result.status ?? 1;
      break;
    }
  }
} catch (error) {
  console.error(`仓库门禁执行失败：${error.message}`);
  process.exitCode = 1;
} finally {
  if (snapshot) rmSync(snapshot, { recursive: true, force: true });
}
