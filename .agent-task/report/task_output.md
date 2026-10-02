outcome: blocked
issue_title: "GitHub Issue #36808: Build Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  The requested feature is blocked because it requires implementing a new native omnichannel chat system using Flutter and expanding channel capabilities, which violates explicit project constraints.

  1. The use of Flutter for the mobile UI is explicitly deprecated. Current project directives mandate that the maintained UI is Next.js inside the Tauri shell or a Node web deployment.
  2. The issue describes a new epic for feature expansion. However, the `RESEARCH.md` and `docs/research/native_migration_and_remediation.md` explicitly state that "The native OmniSolo omnichannel inbox remains in place; expanding its channel and inbox capabilities belongs to later native-chat projects rather than this removal" and require evidence-backed decisions for new channels and epics.
  3. No authorization or evidence is provided to bypass the existing boundaries. The chatwoot components have already been removed as part of an earlier migration/remediation without requiring an immediate, unverified rewrite of the omnichannel functionality.
