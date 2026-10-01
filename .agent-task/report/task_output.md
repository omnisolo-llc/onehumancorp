outcome: blocked
issue_title: "Feature Mission: Native Omnichannel Agentic Support"
issue_description: |
  The requested feature (Native Omnichannel Agentic Support) adds new channels (Instagram, WhatsApp) which conflicts with the expansion gate in RESEARCH.md. Specifically, RESEARCH.md states: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md." There is no retained customer's observed need or willingness-to-pay evidence provided to support this expansion decision.

  Furthermore, a search for Chatwoot dependencies across the repository yielded no results, meaning there are no existing Chatwoot dependencies to remove.

  Therefore, this is a blocked no-work finding.

  Verification Checks and Outcomes:
  - Checked `RESEARCH.md` for strategy constraints regarding new channels.
  - Searched for Chatwoot dependencies and found none.
  - Ran `make test && make lint`; `make test` failed due to missing `next` command (`Error: Next build failed: 127`).

  Superpowers skills loaded:
  - `using-superpowers` (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - `brainstorming` (Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d)
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
