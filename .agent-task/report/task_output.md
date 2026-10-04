outcome: blocked
issue_title: "Implement AI-Native Missed Call & Inquiry Recovery Flow for Operators"
issue_description: |
  The codebase already contains the bounded prototype slice including the `unified_triage_actions` schema, `omni_inbox_triage.spec.ts` tests, and the UI flow with a simulated backend (`src/server/api/work_triage.rs`). Moving beyond the simulated state into actual AI provider execution requires passing the expansion gate and providing explicit test/sandbox authorization. According to the OneHumanCorp operating contract and RESEARCH.md, real model evaluations and external automation are blocked pending explicit owner usage/billing evidence, tenant budget caps, and authorized provider credentials (see F05, F12, F13, F14 in the remediation ledger).
