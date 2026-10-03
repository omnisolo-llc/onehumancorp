outcome: no_work
issue_title: "Architect Native Rust Omnichannel Chat to Replace Chatwoot"
issue_description: |
  The requested Native Rust Omnichannel Chat to replace Chatwoot has already been fully implemented.
  Code exploration verified that:
  - Core domain structs are present in `src/server/domain/chat/mod.rs`.
  - SeaORM entities are available in `src/server/integrations/omnichannel/src/models.rs` (`inbox`, `conversation`, `message`, `contact`).
  - Database schema and PostgreSQL RLS tenant isolation are established in `src/server/migrations/233_chat_omnichannel.sql`.
  - The core service layer `ChatService` exists in `src/server/services/chat/service.rs` with necessary methods (e.g., `create_inbox`, `start_conversation`, `send_message`), including tenant isolation tests (`assert_open_conversation_scope`).
  - Unified ingress webhook handlers are implemented in `src/server/api/omnichannel_webhook.rs` and `src/server/api/unified_inbox_webhook.rs`, resolving identities across channels like `instagram_dm`, `sms`, and `email_inquiry`, and persisting messages as standard payloads.
  Therefore, the acceptance criteria are satisfied by the existing codebase.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
