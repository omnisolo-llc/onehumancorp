outcome: "no_work"
issue_title: "[Research] OHC Native Rust Omnichannel Inbox & Chat Engine (Replacing Chatwoot)"
issue_description: |
  Target: GitHub Issue #36908
  Customer/Workflow: Maya the baker / omnichannel customer support

  Evidence/Justification for no-work:
  The issue requests building a "native Rust implementation of an omnichannel chat engine" to replace an existing third-party dependency (Chatwoot), requiring extensive new schemas, WebSocket infrastructure, and event buses.
  However, `RESEARCH.md` and `docs/research/business_capability_and_usage_economics_audit.md` explicitly mandate: "Retain existing business modules; do not build another generic assistant, duplicate subsystem or broad ERP on the basis of this research." Additionally, "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  There is no explicit evidence of an accepted issue or willingness-to-pay that justifies a complete platform rewrite of the omnichannel inbox. The task is therefore blocked at the expansion gate.

  Superpowers Verification:
  Loaded skill path: skills/using-superpowers/SKILL.md, skills/brainstorming/SKILL.md, skills/systematic-debugging/SKILL.md, skills/executing-plans/SKILL.md
  Upstream revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  Checks and outcomes:
  - `make test && make lint` failed initially due to missing frontend dependencies (`next`, `pg`, `@types/pg`). After installing packages and system libraries (`libglib2.0-dev`, `libgtk-3-dev`, `libsoup2.4-dev`, `libwebkit2gtk-4.1-dev`, `libayatana-appindicator3-dev`, `librsvg2-dev`), running `make test-rust` timed out.
  - No code changes are required as this is a no-work finding blocked by the expansion gate.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
