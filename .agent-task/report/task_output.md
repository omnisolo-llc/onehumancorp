issue_title: "Evaluate compute/API charging and BYOK"
issue_description: |
  This report evaluates compute/API charging and BYOK usage boundaries as instructed.
  - The F01 (repeated usage accounting) finding has already been verified and fixed in the code (verified `record_event` usage in `src/server/hub.rs` and `src/server/services/billing/auditor.rs`).
  - The F02 (global totals in tenant summary) finding is also fixed (verified `tenant_agent_snapshot` in `src/server/services/billing/auditor.rs` filters by `tenant_id`).
  - BYOK and provider native-client subscriptions must be distinguished. Claude Code terms allow native-client hosting only under specific conditions and prohibit API token intermediation.
  - A new cost model must measure OHC-funded model/tool usage, allocate CPU, database, storage and bandwidth, and separate these from customer-paid BYOK.

  Loaded Superpowers revision: using-superpowers (latest local copy), brainstorming (latest local copy).
issue_priority: "High"
issue_category: "billing"
issue_type: "research"
issue_label: "ohc:lane:finance"
assignees: ["jules"]
