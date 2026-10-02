outcome: no_work
issue_title: "Migrate away from Chatwoot to Custom Rust Omnichannel Chat System"
issue_description: |
  The core strategic directive in `RESEARCH.md` states: "Integrate existing tools instead of rebuilding them." Requests to build native replacements for external services (such as Chatwoot) without explicit authorization, evidence, and an expansion gate must be rejected with a no_work finding.

  There is no explicit authorization in `RESEARCH.md` to build a native Chatwoot replacement. Therefore, this task is rejected.

  Superpowers workflow provenance:
  Skill loaded: `using-superpowers`
  Revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d

  Background validation processes (`make test && make lint`) failed due to environment-related issues:
  `make test` failed during `make build-web` with `sh: 1: next: not found`.
  This environmental error is an explicit outstanding blocker.
