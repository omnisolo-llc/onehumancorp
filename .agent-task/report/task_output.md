outcome: blocked
issue_title: "[blocked no-work finding: Native Rust Omnichannel Chat System Replication]"
issue_description: |
  # Native Rust Omnichannel Chat System Replication

  This issue requests the implementation of a native Rust omnichannel messaging system to replace Chatwoot, including Flutter UI and robust real-time WebSocket broadcasting.

  **Verification Evidence:**
  The `docs/superpowers/plans/2026-07-13-chatwoot-removal.md` file shows that the removal of Chatwoot in favor of a native omnichannel architecture is already in progress/complete. The residue script `deploy/tests/no_chatwoot_residue_test.sh` is present, confirming this.

  The implementation scope also requests building a "Flutter Mobile UI (375px-first)". However, per memory instructions, "The project's frontend architecture has explicitly migrated away from the old Slint/Flutter UI; the maintained UI is Next.js inside a Tauri shell or a Node web deployment. Do not attempt to write or execute Flutter/Dart code."

  Because the requested Flutter implementation contradicts the current, explicit frontend architectural direction in the repository, and native chat replacement artifacts already exist (e.g. `src/server/migrations/233_chat_omnichannel.sql` and `src/server/domain/chat/mod.rs`), I am returning a blocked no-work finding. The issue requests implementing legacy Flutter tech and duplicates work already recorded in the native migration ledger and chat removal specs.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat, rust]
assignees: []
