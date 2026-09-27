issue_title: 💰 Miser: [blocked no-work finding: evaluate usage-based billing features]
issue_description: |
  **Loaded Skills:**
  - `skills/using-superpowers/SKILL.md`
  - `skills/brainstorming/SKILL.md`
  - `skills/writing-plans/SKILL.md`
  - `skills/systematic-debugging/SKILL.md`
  - `skills/test-driven-development/SKILL.md`
  - `skills/using-git-worktrees/SKILL.md`
  - `skills/executing-plans/SKILL.md`
  - `skills/subagent-driven-development/SKILL.md`
  - `skills/requesting-code-review/SKILL.md`
  - `skills/receiving-code-review/SKILL.md`
  - `skills/verification-before-completion/SKILL.md`
  **Exact Git Revision:** `8ca22dba9a94f28898bbce59f2537ff4d87c747d`

  **Test Commands Run:**
  - `make test-node` (passed initially, except for `yaml` module and `lucide-react` issues which were fixed)
  - `npm run lint:node` (failed due to existing issues in superpower plugin and scripts)
  - `cargo check --locked --workspace --exclude app --all-targets` (passed after installing dependencies `libglib2.0-dev libgtk-3-dev libwebkit2gtk-4.1-dev libsoup-3.0-dev`)
  - `make test-rust` (failed with an internal error)
  - `npm run test:e2e` (failed because `Required native test input missing: /app/target/debug/server`)

  **Verified Trace Limitations:**
  The `make test-rust` command failed with an internal error, and the E2E tests could not complete because they require the Rust server binary (`/app/target/debug/server`), which was not built due to test failures or missing build steps in the test suite.

  **Checks, and outcomes as final evidence:**
  Based on `docs/research/business_capability_and_usage_economics_audit.md`, there are multiple underlying issues with the current billing capabilities:
  - Usage accounting can feed itself, leading to unbounded telemetry queues.
  - Organization cost summaries read global totals rather than tenant-specific ones.
  - The budget monitor is a soft limit, not a hard spending reservation.
  - Usage attribution differs across model paths.
  - Commercial outputs are often placeholders (e.g., fixed $5000 proposal, invented checkout URL).

  These foundational correctness defects must be resolved first. Implementing new usage-based pricing features is blocked by the lack of trustworthy metered billing, tenant isolation in cost summaries, and reliable usage attribution.

  **Title:** Evaluate usage-based billing features
  **Problem Statement:** The current pricing models are based on a suspended $99 subscription hypothesis. We need to evaluate usage-based billing (compute/API usage) and BYOK capabilities.
  **Research Report:** As detailed in the audit, current billing telemetry is flawed. There's a risk of infinite loops in usage reporting, organization cost API leaks global totals, budget limits are soft, and attribution is inconsistent.
  **Design Doc:** ""
  **Implementation Prompt:** ""
  **Priority:** ""
  **Estimated Scope:** ""
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees:
