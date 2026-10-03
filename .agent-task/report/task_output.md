outcome: no_work
issue_title: "Native Rust Omnichannel Chat System & Universal Inbox"
issue_description: |
  # Prerequisites not met / Already implemented

  The codebase has already migrated away from Flutter to Next.js in a Tauri shell, as stated in the `README.md` ("The old Slint/Flutter UI is also removed; the maintained UI is Next.js inside the Tauri shell or a Node web deployment").

  The issue requests implementing a Flutter UI: "Objective: Implement the core Native Rust Omnichannel Chat Service backend and corresponding Flutter mobile UI components... Build the 375px-optimized Flutter UI for the Unified Inbox list and Conversation view."

  Furthermore, the codebase already has a native Rust omnichannel implementation underway or completed, located at `src/server/integrations/omnichannel` and `src/server/services/chat/service.rs`, which matches the requested database models (`chat_inboxes`, `chat_conversations`, etc.) and multi-tenant RLS policies. The prompt states: "If the issue is already complete or requires unavailable authorization/evidence, return an explicit no_work or blocked outcome. Do not invent follow-up features or dummy changes."

  Because the requested Flutter UI is obsolete and the backend components appear to be substantially present already (or superseded by the Next.js UI path), no further implementation is warranted.
