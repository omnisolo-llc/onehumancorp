outcome: blocked
issue_title: "GitHub Issue #36868: Native Rust Omnichannel Chat: Core Data Models & Schema"
issue_description: |
  Superpowers skill provenance: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)

  The instruction is to implement a SeaORM schema for an Omnichannel Chat engine (Contacts, Inboxes, Conversations, Messages).
  However, based on the `RESEARCH.md` constraint under the "What to defer" section, building out generic custom channel/messenger infrastructure is likely blocked unless it addresses an accepted issue for a selected business outcome that isn't overridden by preserving existing tools. The prompt states:
  "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."

  Crucially, there is no `src/server/ohc/src` or similar dedicated application domain folder containing SeaORM entities in the current repository that match the targeted architecture requested in the issue (checked `src/server/ohc/src`, it doesn't exist). The codebase has some integration directories, but not a `src/server/ohc/src` folder containing the application entities.
  The prompt itself dictates: "Verify current code and tests. If the issue is already complete or requires unavailable authorization/evidence, return an explicit no_work or blocked outcome. Do not invent follow-up features or dummy changes."
  Given the missing domain package in the repository and the lack of authorization/evidence for adding this completely new omnichannel capability that was explicitly categorized as blocked without new evidence, a blocked no-work finding is required.
