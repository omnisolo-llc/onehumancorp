issue_title: "[blocked no-work finding: Native Rust Omnichannel Chat System already implemented]"
issue_description: |
  I have investigated the codebase for the implementation of the Native Rust Omnichannel Chat System, as requested in issue #36754.

  **Finding:** The core native Rust omnichannel chat models, database schema, and REST API are **already fully implemented**.

  **Evidence:**
  - The database migrations for the native omnichannel chat schema exist (`src/server/db/migrations/1009_native_omnichannel_chat.sql`, `src/server/db/migrations/233_chat_omnichannel.sql`) and include all required models (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`). It implements row-level security as well.
  - The domain models and implementation are present under `src/server/domain/chat/mod.rs`, matching the specification (`Inbox`, `Contact`, `Conversation`, `Message`).
  - The database service endpoints are present under `src/server/services/chat/service.rs` (`create_inbox`, `create_channel`, `create_contact`, `start_conversation`, `send_message`).
  - The API handlers exist under `src/server/api/widget/chat.rs`.
  - Thorough testing is included inside these modules.

  Therefore, no new implementation is needed. This constitutes a blocked/no-work outcome, as implementing it again or making dummy changes would be redundant.

outcome: no_work
issue_priority: P0
issue_category: research
issue_type: no_work
issue_label: agent-report
assignees: []
