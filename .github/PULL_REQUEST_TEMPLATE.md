<!-- Title format: <type>(<scope>): <one-line change>, e.g. feat(feedback): preserve the marked scene -->
<!-- 标题格式：<类型>(<范围>): <一句话改动>，如 feat(feedback): 保留圈画对应的现场 -->

## Summary / 改动说明

<!-- Why and what changed. For behaviour changes state before / after. -->
<!-- 为什么改、改了什么。行为变化请写清"之前 / 之后"。 -->

## Related issue / change / 关联 Issue / Change

- Related issue / 关联 issue: #
- OpenSpec change (if applicable): active `openspec/changes/<name>/`, or archived `openspec/changes/archive/<date>-<name>/`
  / 涉及的 change：活动目录或归档目录，无则写"无"

## Boundary check / 边界自检

<!-- Use the current product requirements and roadmap. Describe which user or agent workflow this changes. -->
<!-- 以当前产品依据与路线图为准，说明影响了哪段人或 Agent 的使用流程。 -->

- [ ] Scope matches docs/product/product.md and the related change / 范围与产品依据及对应 change 一致
- [ ] User feedback, observations, check candidates and agent results retain their sources / 用户意见、观测、检查候选与 Agent 结果保留来源
- [ ] Scene and annotation context stays accurate; missing information is explicit / 现场与圈画上下文对应，缺失信息明确
- [ ] Data, logs, caches and temporary files stay under the selected data root / 数据、日志、缓存与临时文件留在指定数据根
- [ ] Agent completion does not silently close human review; reruns stay within authorization / Agent 完成不自动代替人工复核，重跑不越过授权范围

## Repository hygiene / 仓库洁净度

- [ ] `node scripts/check-repo.mjs --staged` green **without** `--no-verify`
      本地按索引快照跑通公共门禁，**没有**用 `--no-verify` 绕过
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
     A machine PASS is not a human approval: list unexecuted checks as "not executed" with the reason —
     never ticked, never reported as delivered. Stale results from an earlier commit do not count. -->
<!-- 写明对应提交、实际执行的命令与结果。涉及构建附产物哈希；涉及界面/视觉附实际点击、键盘与布局证据，
     或明确写"未做人工复核"。机器 PASS 不等于人工通过：未执行项如实写"未执行"与原因，
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
