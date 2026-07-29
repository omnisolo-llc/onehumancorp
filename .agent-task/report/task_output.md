issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Mission Queue Protocol: Omnichannel Chat

  ### Problem Statement
  Owners and operators like Maya (baker) and Carlos (field service) receive customer demand scattered across Instagram, WhatsApp, SMS, and their website. Managing these disconnected inboxes leads to missed leads and slow response times. Previously, OHC relied on an external third-party Chatwoot integration for this capability. However, maintaining a separate Chatwoot cluster breaks our unified multi-tenant architecture and complicates agent orchestration. We need a native, integrated omnichannel chat engine built directly into OHC.

  ### Research Report
  Based on a deep-dive evaluation of the open-source Chatwoot repository (`https://github.com/chatwoot/chatwoot`), the platform models customer communication using the following core domain concepts:
  - **Inboxes**: The central aggregation point. Each inbox maps 1:1 to a specific channel (e.g., a WhatsApp number, a Web Widget).
  - **Channels**: The underlying connector implementations.
    - `Channel::Whatsapp`: Connects via WhatsApp Cloud API or providers like Twilio. Uses phone numbers and provider credentials.
    - `Channel::WebWidget`: A live chat widget embedded on websites. Uses `website_token`, `widget_color`, and `welcome_tagline`.
    - `Channel::TwilioSms`: Handles SMS messages via Twilio (`account_sid`, `auth_token`, `phone_number`).
  - **Conversations**: Groupings of messages between a Contact and the Inbox.
  - **Webhooks**: Chatwoot relies heavily on webhooks (`app/models/webhook.rb`) to dispatch events (`message_created`, `conversation_updated`) to external systems (or AI bots).

  By implementing these capabilities natively in Rust, OHC can provide a zero-configuration experience for owners. When Maya connects her Instagram, she doesn't need to configure a third-party CRM; OHC's native Chat engine will ingest the messages directly into her daily work feed.

  ### Design Doc
  - **Data Model**: Introduce native Rust models mirroring the core Chatwoot domain but heavily optimized for OHC's multi-tenant PostgreSQL schema (row-level security via `tenant_id`).
    - `Inbox`: Configuration for a communication channel.
    - `Channel`: Trait/Interface for various connectors (WhatsApp, SMS, Web Chat).
    - `Conversation` and `Message`: Core chat payload storage.
  - **Integration Point**: The Chat engine will run as a native service within `onehumancorp/mono`. Webhook endpoints will be exposed via the gRPC/REST API layer to receive incoming messages from providers like Twilio or Meta.
  - **AI Handoff**: Instead of external webhooks, OHC's native AI agents will subscribe to internal event buses (e.g., via Redis or in-memory queues) to instantly draft replies when a new `Message` arrives.
  - **User Experience**: The owner sees a unified "Work Triage" feed. They click a message, see the AI-drafted reply, and hit "Send"—the system handles routing it back to the original channel (WhatsApp, Instagram, etc.).

  ### Implementation Prompt
  Implement a native Rust multi-tenant Omnichannel Chat module inside OHC's backend. Start by defining the core database schema (PostgreSQL) and Rust structs for `Inbox`, `Conversation`, and `Message`. Then, implement two initial channel connectors: a Web Widget (for OHC hosted portals) and a Twilio SMS connector. Ensure all tables enforce `tenant_id` row-level security. The UI must present a unified conversation view that seamlessly handles messages regardless of the underlying channel, and the backend must emit events that OHC's AI agents can consume to draft responses. Do not use external Chatwoot services.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
