outcome: no_work
issue_title: "[no-work finding] Design OHC Native Rust Omnichannel Chat: Data Model & Repository Layer"
issue_description: |
  The task requested the design and implementation of a native Rust omnichannel chat system, specifically the data model and repository layer, tracking Conversations, Messages, Contacts, and Channels with strict multi-tenant isolation.

  However, this feature expands the product's scope into new channels and agent integrations that are explicitly blocked by the repository guidelines. According to the current instructions in `RESEARCH.md` and the job description:
  - "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  - "Missing or materially conflicting strategy is a blocker to new scope, not a reason to improvise another product direction."
  - "No deferred feature is commissioned merely because an old prompt says mandatory or a generic TODO exists."

  There is no explicit expansion gate or evidence in `RESEARCH.md` authorizing the implementation of a native Rust omnichannel chat engine. The `RESEARCH.md` document explicitly states: "New epics need an explicit evidence-backed decision; assigned concrete defect work may continue."

  Therefore, following the strict repository guidelines, this requested implementation is rejected as it requires an expansion gate that is not present. This constitutes a `no_work` finding.
