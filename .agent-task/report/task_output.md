issue_title: "🎨 Canvas: [blocked no-work finding: WhatsApp Integration]"
issue_description: |
  **Observation:**
  The task asks to implement a Native Rust Omnichannel Chat & WhatsApp Integration (WhatsAppCloudClient, WhatsApp webhook handlers, PostgreSQL schema changes, Redis locks, etc.) based on a research report describing an AI Ambassador agent integration.

  However, per the final scope and evidence check (revision 2026-09-18-usage-audit), this new feature development requires explicit business evidence.

  **Verification Limitations:**
  The `RESEARCH.md` states: "Evaluate managed API, customer API-key/cloud-account billing, provider-permitted native-client subscription access and local inference separately... Evaluation of managed API vs API-key/cloud-account billing is currently blocked pending real usage data and owner interviews. Evaluation of provider-permitted native-client subscription vs local inference is blocked due to missing specific provider access prerequisites."

  The instruction strictly states: "Verify current code and tests. If the issue is already complete or requires unavailable authorization/evidence, return an explicit no_work or blocked outcome. Do not invent follow-up features or dummy changes." The WhatsApp omnichannel integration relies on provider APIs, authentication rules, and AI inference billing configurations that are explicitly blocked by missing commercial evidence and owner decisions in the business audit.

  **Executed test commands:**
  - `make test` (Failed due to timeout/missing dependencies)
  - `cargo test --locked --workspace --exclude app` (Timed out)

  **Outcome:**
  Therefore, this is a blocked no-work finding. I am generating the report and terminating the task without making dummy code changes.

issue_priority: "P0"
issue_category: "backend"
issue_type: "blocked"
issue_label: "blocked"
assignees: []
outcome: blocked
