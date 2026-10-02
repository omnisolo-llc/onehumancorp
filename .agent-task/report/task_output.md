outcome: no_work
issue_title: "[Research] Architect Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  The request to build a native replacement for an external service (Chatwoot) requires explicit authorization, evidence, and an expansion gate.
  In accordance with the operating contract and memory guidelines:
  - "Requests to build native replacements for external services (e.g., Chatwoot) require explicit authorization, evidence, and an expansion gate; otherwise, return a no_work finding."
  - "Integrate existing tools instead of rebuilding them."

  Test validation results: The tests failed with a build error when run against the unmodified codebase (`sh: 1: next: not found`), a known environmental limitation when only creating a `.agent-task` report. Outstanding blocker: environment failed to build Next standalone Node assets.

  Superpowers workflow provenance: loaded skills/using-superpowers, skills/brainstorming from revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d.
