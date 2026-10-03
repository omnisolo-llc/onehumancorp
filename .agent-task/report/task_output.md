outcome: blocked
issue_title: Native Rust Omnichannel Chat (Chatwoot Replacement)
issue_description: |
  **Verified Target**: Native Rust Omnichannel Chat System: Architecting The Ambassador (GitHub Issue #35389).

  **Finding**: The task is blocked. The prompt requests the implementation of a native Rust Omnichannel Chat system (replacing Chatwoot, with an Axum/Tonic API, PostgreSQL DB, and mobile UI) and mentions it is a P0 requirement. However, according to the OneHumanCorp operating contract in `RESEARCH.md` (Revision 2026-09-18-usage-audit), new channels and agent integrations require an explicit expansion gate and evidence. The document states:

  > "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md. Do not interpret a successful PR as a completed customer business outcome."

  > "If fewer than five partners pay, retained use is weak, owner correction erases time savings, or serving cost remains too high, narrow the workflow, revise price/packaging or reassess the segment. Do not respond by adding ten more departments."

  The current capability map and current evidence in `RESEARCH.md` do not authorize a complete rewrite/replacement of the omnichannel chat system into Rust as a new integration/channel expansion without prior established gates. Consequently, I am recording a blocked outcome and keeping the repository clean of unauthorized rewrites.
issue_priority: P0
issue_category: integrations
issue_type: feature
issue_label: ohc:lane:revenue
assignees: []
