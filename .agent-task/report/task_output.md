outcome: no_work
issue_title: "Native Rust Omnichannel Chat System Implementation to Retire Chatwoot"
issue_description: |
  Superpowers skill provenance: skills/using-superpowers/SKILL.md (revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d).

  The issue requests the implementation of a native Rust Omnichannel Chat API to replace Chatwoot, and to implement corresponding Flutter UI components (which is obsolete, as the repository uses Next.js/React).

  Upon exploration, this system is already implemented and verified in the codebase. The native Rust backend resides in `src/server/integrations/omnichannel`, and the corresponding frontend resides in `src/ui/next/src/app/api/v1/ui/omni_inbox` and `src/ui/next/src/app/inbox`.

  Executed test commands:
  cargo test -p server_integrations_omnichannel
  (Passed)

  Since the requested feature is already implemented and verified, no new implementation is needed.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
