outcome: blocked
issue_title: "🔨 Forge: [blocked no-work finding: Stripe payment implementation]"
issue_description: |
  The requested Stripe payment link automation (Issue #38887) is currently blocked due to multiple project constraints and unresolved dependencies documented in the project ledgers.

  The issue asks for a secure connection method for Stripe, backend logic to generate Stripe payment links, and a webhook listener. While a secure connection implementation currently exists and is verified under finding F06 ("usable connection flow") in the remediation ledger (docs/research/native_migration_and_remediation.md), the backend logic and webhook acceptance criteria remain unmet and cannot be safely implemented at this time.

  1. The remediation ledger states that for finding F08 ("fabricated checkout links"), "Full provider sandbox replay/payment event reconciliation, persisted provider receipts across every workflow and all business transitions remain outstanding." This means the foundational support for reliable webhook reconciliation is incomplete, and we cannot simply add a one-off webhook endpoint without these larger prerequisites.
  2. For finding F05 ("telemetry not invoice-grade"), the ledger explicitly records that "Customer payment collection remains disabled on the new usage API" and that provider-invoice reconciliation and payment collection are not completed by this ledger. Creating fully integrated real-world money collection workflows requires these billing fundamentals to be solid.
  3. The issue also asks for an "invisible" Stripe checkout integration without "setting up merchant accounts" based on the hypothesis of a solo service provider segment. But `RESEARCH.md` states: "The earlier $99 subscription, $299 setup, 300-step allowance, fixed cohort and margin targets are SUSPENDED HYPOTHESES... Do not implement them as billing or segment constraints." It also states, "Research evidence alone does not certify these gates or a shipped business outcome." and "A missing SDK, provider sandbox, signing credential or owner interview is a specific outstanding verification dependency, not permission to report success."

  Since we do not have the verified prerequisites (reliable durable ledger for reconciliation, sandbox environment setup for external tests) to properly implement safe payment links, and since the remediation ledger documents this as a blocked/in-progress concern across F05/F08, a "blocked no-work finding" is the required outcome according to the project's strict guidelines.
issue_priority: P1
issue_category: "backend"
issue_type: "feature"
issue_label: "blocked"
assignees: []
