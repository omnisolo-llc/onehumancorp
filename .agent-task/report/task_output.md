outcome: blocked
issue_title: "Native Rust Omnichannel Chat System & Universal Inbox"
issue_description: |
  Implementation of the Native Rust Omnichannel Chat System (Legacy Omni-channel Replacement) is blocked.

  Prerequisite or verification evidence:
  1. According to the OHC operating contract, new channels (like Instagram, WhatsApp, SMS) require explicit evidence and an expansion gate in RESEARCH.md. An audit of RESEARCH.md shows no such explicit strategy approval or expansion gate for this omnichannel implementation. Therefore, implementation is blocked.
  2. The issue requests the implementation of "Flutter mobile UI components". However, the project's frontend architecture has explicitly migrated away from the old Slint/Flutter UI to a Next.js UI inside a Tauri shell or Node web deployment. Writing or executing Flutter code is explicitly forbidden.

  Target ID: #35934
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
