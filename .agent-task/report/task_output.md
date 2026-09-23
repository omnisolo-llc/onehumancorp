issue_title: "Blocked: Missing billing and usage prerequisites for growth features"
issue_description: |
  **Observation:** As Principal Growth Engineer & Nova, I investigated potential growth loops (e.g., paywalls, upgrade pathways, referral systems). However, according to `docs/research/business_capability_and_usage_economics_audit.md`, several critical billing and usage accounting prerequisites remain unverified or missing (Findings F04, F05, F14). Specifically, there is no invoice-grade telemetry meter (F05), model paths disagree on usage (F04), and there are no measured representative serving costs or owner outcomes (F14).

  **Justification:** The instructions strictly state: "Before rates, prove stable usage identity, payer/model/rate attribution, tenant-specific reads, replay-safe settlement, hard budget reservations and invoice reconciliation" and "If prerequisites for growth loops are missing or unverified, report a justified no-work/blocked outcome via `.agent-task/report/task_output.md` rather than fabricating an unsupported feature." Since these prerequisites are missing, any conversion or monetization loop built on top of them would be unsupported fabrication.

  **Outcome:** This task is blocked. A no-work outcome is reported. The foundational billing and telemetry gaps (F04, F05, F14) must be resolved before proceeding with growth optimizations.

  **Loaded skills/revision:**
  - superpowers/brainstorming
  - superpowers/using-superpowers
  - Revision: f8e9d8dd5c099f417df0c32f6131e9b465e5fb20
issue_priority: "P0"
issue_category: "research"
issue_type: "report"
issue_label: "ohc:lane:growth"
assignees: []
