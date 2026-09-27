issue_title: |
  💰 Miser: [blocked no-work finding: evaluate compute/API charging and BYOK]
issue_description: |
  # Task: Evaluate compute/API charging and BYOK modes

  **Problem Statement:**
  We needed to evaluate charging for compute and AI API usage, customer BYOK, and provider-permitted native subscription access. However, after reading `docs/research/business_capability_and_usage_economics_audit.md`, I discovered that the system lacks the prerequisites for trustworthy metered billing. There are severe defects in usage accounting, cost isolation, and provider attribution that block us from implementing billing at this time.

  **Research Report:**
  1. Usage accounting has an unbounded feedback loop (`src/server/hub.rs` and `services/billing/auditor.rs`), meaning the same event could be counted multiple times.
  2. The organization cost summary (`services/billing/service.rs`) reads global totals rather than tenant-specific totals, leaking cost data across organizations.
  3. The budget monitor (`pricing/budget.rs`) only implements a soft limit, not the hard spending reservation required for customer-funded usage.
  4. Usage attribution is inconsistent across models, and commercial outputs (like proposals and invoices) are currently hardcoded placeholders.

  **Design Doc:** ""
  **Implementation Prompt:** ""
  **Priority:** ""
  **Estimated Scope:** ""

  **Evidence:**
  - Loaded skills: `skills/using-superpowers/SKILL.md`, `skills/brainstorming/SKILL.md`
  - Exact git revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Executed test commands: `make test-node`, `cargo check --locked --workspace --exclude app --all-targets`
  - Checks: Codebase inspection of `hub.rs`, `billing/auditor.rs`, `billing/service.rs`, and `pricing/budget.rs` based on the audit document.
  - Outcomes: Blocked due to foundational defects in the usage accounting and cost attribution systems. These issues must be resolved before proceeding with compute/API charging or BYOK implementation. (checks, and outcomes as final evidence)
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
