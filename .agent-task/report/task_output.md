outcome: blocked
issue_title: "Native Rust Omnichannel Chat: Core Data Models & Schema"
issue_description: |
  This issue asks to create SeaORM migration scripts and entities for a native Rust Omnichannel Chat engine (Contact, Inbox, Conversation, Message) to replace Chatwoot capabilities.
  Acceptance criteria satisfied: None.
  Acceptance criteria remaining unmet/unverified:
  - Create SeaORM migration scripts with tenant_id for RLS.
  - Generate SeaORM entity structs.
  - Register entities in Database setup.
  - Write unit tests for CRUD and tenant_id invariants.
  Why no safe implementation follows:
  Per the final OHC scope and evidence check (revision 2026-09-18-usage-audit), new epics, verticals, and channels require an explicit evidence-backed decision and the expansion gate in RESEARCH.md. The active business-capability map in RESEARCH.md does not include an Omnichannel Chat workflow, and explicit instructions forbid building deferred features without owner evidence and usage baseline validation. Thus, the implementation is blocked pending explicit business evidence and authorization.
  Loaded Superpowers revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d (skills/using-superpowers/SKILL.md)
