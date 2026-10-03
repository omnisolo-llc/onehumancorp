outcome: blocked
issue_title: "Native Rust Omnichannel Chat: Inbox, Conversations, and Messaging Pipeline"
issue_description: "The issue #35801 requires building the Native Rust Omnichannel Chat domain structs, Postgres schema migrations, and SeaORM/sqlx entities. After reviewing the current repository implementation, these components are already implemented. \n\n1. The Postgres schema migrations (e.g., `233_chat_omnichannel.sql` and `1024_chat_message_content_type.sql`) exist and include strict tenant_id multi-tenancy columns and RLS.\n2. The Rust models are already implemented in `src/server/services/chat/models.rs`.\n3. The repository and service layer are already implemented using `sqlx` in `src/server/services/chat/service.rs`.\n4. Endpoints for the mobile app to fetch conversations and messages are already built in `src/server/api/widget/chat.rs`.\n\nSince the specified target is already complete in the codebase, no further implementation is needed. Proceeding with a blocked/no_work report."
issue_priority: "P0"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
