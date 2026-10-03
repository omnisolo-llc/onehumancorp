outcome: no_work
issue_title: "[Platform] Native Rust Omnichannel Chat (Chatwoot Replacement)"
issue_description: |
  **Outcome: No-Work (Blocked by Missing Expansion Gate)**

  **Issue Addressed**: #35169 [Platform] Native Rust Omnichannel Chat (Chatwoot Replacement)

  **Evidence & Justification**:
  According to the `RESEARCH.md` contract (Revision: 2026-09-18-usage-audit) and the business capability audit:

  1.  **Strict Scope Gate**: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md."
  2.  **No New Architectures**: "Reuse existing architecture and preserve existing standalone behavior."
  3.  **Chatwoot specifically**: Expanding into an omnichannel Chatwoot-replacement infrastructure is an expansion into new channels and requires an explicit expansion gate with owner evidence in `RESEARCH.md`. The current documentation only allows focusing on correctness, bugfixes, and the defined specific core workflows.
  4.  **No Explicit Authorization**: There is no documented proof or authorized owner evidence in `RESEARCH.md` demonstrating that a new native omnichannel inbox database schema, REST API, and WebSocket publisher is approved for development.

  As instructed in `RESEARCH.md`: "A no-work/blocked result with evidence is valid... Do not interpret public anecdotes as customer interviews. OHC owns persistent business/customer/job context... Integrate existing tools instead of rebuilding them... If the issue's requested implementation already exists in the codebase and satisfies the acceptance criteria, you must return an explicit no_work finding... New epics need an explicit evidence-backed decision; assigned concrete defect work may continue."

  Since there is no expansion gate in `RESEARCH.md` to authorize building a new omnichannel database architecture and websocket pub/sub, this task is explicitly blocked.

  **Superpowers Skill Provenance**:
  Loaded skills/revision: using-superpowers (HEAD of https://github.com/obra/superpowers/)
