outcome: no_work
issue_title: "Implement Custom Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  The requested feature (implementing a custom Rust omnichannel chat system to replace Chatwoot, including inbox, contact, conversation, and message models) is already implemented in the codebase.

  Evidence:
  1. The core data models (`chat_inboxes`, `chat_conversations`, `chat_messages`, `chat_contacts`) and their relationships are already implemented in `src/server/integrations/omnichannel/src/models.rs`.
  2. The database migrations for these tables exist, e.g., `src/server/db/migrations/20260701_omnichannel_tables.sql` and `src/server/migrations/233_chat_omnichannel.sql`.
  3. The requested AI proactive drafting, Gateway webhook processing, etc., are also represented by the files in `src/server/integrations/omnichannel/src/`, such as `web_widget_adapter.rs`, `whatsapp_adapter.rs`, `router.rs`, `state_machine.rs`, and the webhook handlers in `src/server/api/omnichannel_webhook.rs`.
  4. End-to-end testing stubs and contracts are available in `src/e2e/playwright/omnichannel-intake.mock-contract.ts` and others.

  Therefore, I am returning a `no_work` finding as the requested functionality is already present and matches the acceptance criteria.
