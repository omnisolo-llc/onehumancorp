outcome: blocked
issue_title: "Omnichannel Tap-to-Pay Terminal SDK & Agentic POS Architecture"
issue_description: |
  The task is blocked because the required POS engines and hardware integrations (like Stripe Terminal) represent new verticals and integrations. According to the `RESEARCH.md` contract:
  > New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md.

  And further:
  > The user's instruction to migrate from Bazel to native Rust/Cargo, Tauri and Node.js, then address every finding in the detailed audit.

  And specifically regarding the audit findings:
  > F14: No measured representative serving costs or owner outcomes -> Blocked

  Since the required evidence (e.g. owner willingness to pay, pilot data, expansion decision in `RESEARCH.md`) for POS engine integration is missing, this feature request cannot be fulfilled within the current authorized scope.
