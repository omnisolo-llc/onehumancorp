outcome: blocked
issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  Missing required external verification evidence.

  The `docs/superpowers/specs/2026-07-13-native-omnichannel-chat-design.md` specifies that "Every enabled core cutover connector passes its credentialed end-to-end sandbox gate; a connector without evidence remains disabled and explicitly blocked." and "Missing external credentials therefore block that connector's readiness rather than becoming a skipped success."

  Since we do not have external sandbox provider credentials (WhatsApp, Resend, SendGrid, etc.) available in the environment to perform the credentialed end-to-end sandbox verification, we cannot implement and verify this feature. It is explicitly blocked by the design document.

  Loaded Superpowers skills: `using-superpowers`, `brainstorming`
  Skill revision: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  Verification check: `deploy/tests/no_chatwoot_residue_test.sh` passed.
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
