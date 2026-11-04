<!-- Title format: <type>(<scope>): <one-line change>, e.g. feat(reduce): re-verify failure after each dropped condition -->
<!-- 标题格式：<类型>(<范围>): <一句话改动>，如 feat(reduce): 逐条件回退时保持失败才算缩减成功 -->

## Summary / 改动说明

<!-- Why and what changed. For behaviour changes state before / after. -->
<!-- 为什么改、改了什么。行为变化请写清"之前 / 之后"。 -->

## Related issue / change / 关联 Issue / Change

- Related issue / 关联 issue: #
- OpenSpec change (if applicable): active `openspec/changes/<name>/`, or archived `openspec/changes/archive/<date>-<name>/`
  / 涉及的 change：活动目录或归档目录，无则写"无"

## Boundary check / 边界自检

<!-- Layress deliberately separates four jobs. Say which one this serves, and that it does not blur them. -->
<!-- 四类工作是刻意分开的。说明本 PR 服务哪一类，以及它没有把边界抹糊。 -->

- [ ] Serves one of: scan / confirm / reduce / replay / reporting / integration — stated above
      服务于扫描 / 确认 / 缩减 / 回放 / 报告 / 集成 之一，已在上文说明
- [ ] A geometric anomaly is still reported as a **candidate**, not as a defect (I1)
      几何异常仍然按**候选**输出，没有被写成缺陷（I1）
- [ ] Reachability verdicts use real hit-testing, not programmatic clicks (I2)
      可达性判定走真实命中测试，没用程序化点击（I2）
- [ ] Trigger conditions keep the full four-tuple; nothing guessed in when unknown (I3, I4)
      触发条件保持四元组完整；不知道就标 unknown，没有猜着补全（I3、I4）
- [ ] No new default write outside the data root; no `%APPDATA%` / registry / system PATH change (I12, I13)
      没有在数据根之外新增默认写入；未碰 `%APPDATA%`／注册表／系统 PATH（I12、I13）
- [ ] Replay still adds no real external side effects (I5) / 回放仍未新增真实外部副作用（I5）

## Repository hygiene / 仓库洁净度

- [ ] `node scripts/check-hygiene.mjs --staged` green **without** `--no-verify`
      本地跑通洁净度门禁，**没有**用 `--no-verify` 绕过
- [ ] No new file lands in `artifacts/` / `.layress/`; hand-written conclusions went to the right `docs/` subdirectory
      没有把手写内容落在垃圾场；结论已进 `docs/` 正确子目录
- [ ] Any exemption added to `.git-hygiene-allowlist` carries a one-line reason (reasonless exemptions fail)
      新增豁免都带一句理由（无理由会判红）
- [ ] No binary, credential, absolute machine path or real scanned page content committed
      未提交二进制、凭据、机器绝对路径或真实被扫描页面内容

## Bilingual docs / 双语文档

- [ ] Edited public documents listed in the manifest's `docs` have both languages updated **in this same PR**; internal documents need only Chinese
      清单 `docs` 中的对外文件已在**本 PR 内**同步中英；内部文档只需中文
- [ ] Any newly provided public English version is registered in `i18n/docs-manifest.json`
      本次新增的对外英文版已登记进 `i18n/docs-manifest.json`
- [ ] New or changed terminology entered in `i18n/glossary.md`; no English copy of the glossary required
      术语新增/变更已写进 `i18n/glossary.md`，术语表本身无需英文副本
- [ ] `node scripts/check-i18n.mjs` green / 双语门禁本地通过

## Verification record / 验证记录

<!-- State the commit, the exact commands run and their output. For builds attach artifact hashes; for UI or
     visual checks attach the actual click / keyboard / layout evidence, or state that human review was NOT done.
     A machine PASS is not a human approval (I11): list unexecuted checks as "not executed" with the reason —
     never ticked, never reported as delivered. Stale results from an earlier commit do not count. -->
<!-- 写明对应提交、实际执行的命令与结果。涉及构建附产物哈希；涉及界面/视觉附实际点击、键盘与布局证据，
     或明确写"未做人工复核"。机器 PASS 不等于人工通过（I11）：未执行项如实写"未执行"与原因，
     不许勾完成、不许写成已交付；旧提交的结果不能拿来勾本轮。 -->

- 提交 / commit:
- 命令与结果 / commands and output:
- 人工复核 / human review: 已执行 · 未执行（原因：）
- 未覆盖范围 / not covered:

## Notes / 备注

- [ ] Purely documentation-only? Then no build, app launch or model call is required — say so here
      纯文档改动？那无需构建、启动应用或调用模型，请在此说明
- [ ] Third-party code, data, icons or text added? List source, version and license, keep required notices;
      write "none" if there is nothing
      引入了第三方代码/数据/图标/文案？列出来源、版本与许可并保留声明；没有就写"无"
