issue_title: "Native Rust Omnichannel Chat: Inbox & Channel Data Models (Phase 1)"
issue_description: |
  **Problem Statement:**
  Currently, OHC lacks a native omnichannel chat system. The mandate requires retiring any external dependencies on Chatwoot and implementing a 100% native Rust, multi-tenant omnichannel chat engine. Without these data models, owners like Maya (Home Baker) and Carlos (Field Service) cannot aggregate customer inquiries from Instagram, WhatsApp, and Web Chat into a single, unified view natively within OHC.

  **Research Report:**
  Based on an exhaustive evaluation of the Chatwoot source code (`app/models/inbox.rb`, `app/models/channel/`, `app/models/conversation.rb`), Chatwoot uses a robust pattern where an `Inbox` belongs to an `Account` (tenant) and delegates its underlying channel configuration to a polymorphic `Channel` (e.g., `Channel::Email`, `Channel::Whatsapp`, `Channel::WebWidget`). This allows for unified conversation routing while keeping channel-specific credentials and configurations isolated. For OHC, this needs to be adapted to our multi-tenant PostgreSQL architecture using row-level security (`tenant_id`).

  Competitor analysis shows that platforms like Shopify Inbox and Zendesk use similar unified data structures, but OHC's version must emphasize simplicity for the non-technical owner, hiding the complex channel configurations behind seamless connection flows.

  **Design Doc:**
  - **New Models (`src/server/services/chat/models.rs`)**:
    - `Inbox`: Represents a unified queue for messages. Needs fields like `id`, `tenant_id`, `name`, `channel_type`, `greeting_enabled`, `greeting_message`, `working_hours_enabled`, etc.
    - `ChannelConfig`: An enum or JSONB column (or separate tables) to store channel-specific data (e.g., WhatsApp phone number, WebWidget token).
    - `Conversation`: Represents a thread of messages within an Inbox.
    - `Message`: Represents an individual message in a conversation.
  - **Integration**: These models will be managed by `src/server/services/chat/service.rs` and exposed via REST/gRPC endpoints in `src/server/api/chat.rs`.

  **Implementation Prompt:**
  Implement the core database schema (Diesel/SQLx migrations) and Rust data structures (`structs`) for `Inbox`, `Channel`, `Conversation`, and `Message` in `src/server/services/chat/models.rs`. Ensure all models enforce `tenant_id` isolation. Create a basic Rust service layer (`src/server/services/chat/service.rs`) with functions to create, read, update, and delete inboxes and conversations. Verify functionality with 100% unit test coverage. The owner should ultimately be able to create an "Inbox" and see a unified stream of messages.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
