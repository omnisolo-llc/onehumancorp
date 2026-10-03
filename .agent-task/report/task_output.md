outcome: blocked
issue_title: "Architecture: Native Rust Omnichannel Chat Engine"
issue_description: |
  The requested omnichannel chat engine requires new channels (Instagram DMs, WhatsApp, SMS, Web Chat, Line) and a native Flutter UI frontend. However, according to the final OHC scope check, "new verticals, channels, agent marketplaces, and harness adapters require explicit evidence and an expansion gate in RESEARCH.md." An audit of RESEARCH.md confirms no such expansion gate exists for these channels, thus the implementation of new channels is blocked. Furthermore, the issue specifies implementing the UI in "the Flutter frontend," but the project's frontend architecture has explicitly migrated away from the old Slint/Flutter UI to Next.js inside a Tauri shell. Therefore, writing Flutter/Dart code is prohibited.
issue_priority: P0
issue_category: architecture
issue_type: feature
issue_label: blocked
assignees: []
