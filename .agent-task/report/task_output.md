outcome: blocked
issue_title: "Architecture Design: Native Rust Omnichannel Chat Engine"
issue_description: |
  Implementation of the Native Rust Omnichannel Chat Engine is blocked due to the following reasons:

  1. The task requests the implementation of new channels (Instagram DMs, WhatsApp, SMS) and an omnichannel chat engine. However, according to the OHC operating contract: "New verticals, channels, agent marketplaces, and harness adapters require explicit evidence and an expansion gate in RESEARCH.md. Without it, implementation is blocked." I have verified that RESEARCH.md contains no such expansion gate or mention of omnichannel chat, WhatsApp, SMS, or Instagram DMs.

  2. The issue explicitly requests building "Flutter frontend ConversationList and ConversationThread widgets". However, memory instructions explicitly mandate: "The project's frontend architecture has explicitly migrated away from the old Slint/Flutter UI; the maintained UI is Next.js inside a Tauri shell or a Node web deployment. Do not attempt to write or execute Flutter/Dart code."
issue_priority: P0
issue_category: interop
issue_type: feature
issue_label: interop
assignees: []
