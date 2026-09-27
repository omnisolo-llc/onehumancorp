issue_title: "Economics and outcomes data collection"
issue_description: |
  **Title**: Collect economics and outcomes data for `docs/research/native_migration_and_remediation.md` (F14)

  **Problem Statement**:
  The F14 finding requires measured representative serving costs and owner outcomes to ensure informed business decisions. However, the data collection step failed, causing the issue to remain open/blocked in the `native_migration_and_remediation.md` report. The current environment cannot fetch this evidence, and without the proper measurements (workload usage records, build/resource timing), we cannot make claims about customer outcomes or OHC performance.

  **Research Report**:
  - The F14 audit requirement asks for: "Workload/cost instrumentation and repeatable benchmark/export; do not claim interviews, customer acceptance, real costs or competitive advantage without evidence."
  - My test execution `make test-node` generated results about metrics/telemetry tests, but it does not supply external user-evidence or workload costs from an actual owner case study, which is the missing ingredient for F14.
  - As directed by `RESEARCH.md` and the audit instructions, "A missing SDK, provider sandbox, signing credential or owner interview is a specific outstanding verification dependency, not permission to report success." Therefore, this task is genuinely blocked as a no-work finding because no such data exists natively in the test bench.
  - Final evidence: Loaded skills (using-superpowers, brainstorming, writing-plans). Exact Git revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d. Executed test commands: `make test-e2e` (timeout), `make test-node` (passed unit checks). Checks, and outcomes as final evidence.

  **Design Doc**: ""
  **Implementation Prompt**: ""
  **Priority**: ""
  **Estimated Scope**: ""
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
