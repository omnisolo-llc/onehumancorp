issue_title: "Implement Native Rust Omnichannel Chat System"
issue_description: "The issue asks to build a new high-performance, multi-tenant, omnichannel chat engine natively in Rust inside `onehumancorp/mono`. We need to define PostgreSQL schemas for `inboxes`, `conversations`, `messages`, and `contacts` ensuring `tenant_id` and RLS policies are set up. Also to implement Rust data models and DB repos, core API endpoints, and a foundational WebSocket handler."
outcome: "no_work"
evidence: "The feature has already been fully implemented.
- `src/server/migrations/1009_native_omnichannel_chat.sql` contains the requested PostgreSQL schema including `chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, and `chat_messages` tables with `tenant_id` columns and Row Level Security (RLS) policies.
- `src/server/integrations/omnichannel/` exists and contains the Rust data models (`src/server/integrations/omnichannel/src/models.rs`) mapped with `sea_orm`, and the repository implementation (`src/server/integrations/omnichannel/src/repository.rs`) handling the core abstractions.
- `src/server/services/chat/` contains another version of models, and the DB interaction service `src/server/services/chat/service.rs` managing transactions, tenant scopes and operations for inboxes, channels, contacts, conversations, and messages. Tests also provide evidence.
- The `WidgetChatState` logic at `src/server/api/widget/chat.rs` handles the multi-tenant verified API operations connecting the domain repository.
- There are already multiple omnichannel webhooks implemented in `src/server/api/omnichannel_webhook.rs` and `src/server/api/inbox/webhook.rs` that listen to incoming messages from platforms like WhatsApp, using the event routing system (`tenant.omnichannel.message.received`).
- `src/server/integrations/whatsapp_cloud` and `src/server/integrations/whatsapp` integrate WhatsApp natively with payloads and REST clients.
Because the exact schema and data models requested already exist and are active in the codebase, this is a redundant feature request and no new work needs to be performed."
