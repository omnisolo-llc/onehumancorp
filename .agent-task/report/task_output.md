issue_title: "Architect Native Rust Omnichannel Chat to Replace Chatwoot"
issue_description: |
  **Title:** Architect Native Rust Omnichannel Chat to Replace Chatwoot

  **Problem Statement:**
  The platform relies on Chatwoot as an external service for omnichannel messaging (Instagram, WhatsApp, Email, SMS). Chatwoot is 100% retired. OHC must implement its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust to achieve 100% feature parity. The absence of this native layer forces fragmented context, breaking the core "unified assistant" promise for users like Maya and Carlos who run businesses primarily from Instagram DMs and WhatsApp.

  **Research Report:**
  - **Competitor Systems Audit:** Analyzed Chatwoot source code (`app/models/conversation.rb`, `app/models/message.rb`, `app/models/channel/`). Chatwoot abstracts platforms via polymophic `Channel` adapters, storing universal `Conversation` and `Message` entities mapped to a central `Inbox`. Leading platforms like Shopify Inbox handle this natively via edge-cached WebSockets, reducing dependency overhead and improving SLA for LLM processing.
  - **Codebase Audit:** OHC currently lacks a native `Conversation`, `Message`, and `Inbox` domain with unified channel egress/ingress. The new system must isolate tenants via PostgreSQL RLS and provide direct event hooks for OHC's AI agents.

  **Design Doc:**
  - **Architecture Diagram (Mermaid.js):**
    ```mermaid
    erDiagram
        TENANT ||--o{ INBOX : manages
        INBOX ||--o{ CONVERSATION : contains
        CONVERSATION ||--o{ MESSAGE : contains
        CONVERSATION }o--|| CONTACT : belongs_to
        INBOX }o--|| CHANNEL_ADAPTER : uses

        MESSAGE {
            uuid id
            uuid conversation_id
            uuid sender_id
            string content
            enum message_type
            jsonb content_attributes
        }
        CONVERSATION {
            uuid id
            uuid inbox_id
            uuid contact_id
            enum status
            jsonb custom_attributes
        }
    ```
  - **Mobile UX Flow (375px first):**
    - The bottom navigation defaults to the "Inbox" tab with an unread badge.
    - **List View:** Clean, translucent cards showing contact name, last message preview, channel icon (e.g., IG, WA), and an "Agent Handled" status dot.
    - **Detail View:** Full-height chat interface. Native mobile keyboard support. Quick action chips float above the input (e.g., "Draft Quote", "Send Payment Link").
  - **AI Agent Integration Points:**
    - AI agents listen to `MessageCreated` events asynchronously via the AI Job Queue (PostgreSQL `SKIP LOCKED`).
    - Agents act as `assignee` on new conversations, drafting replies (Message status: `draft`) for owner review, or auto-replying based on tenant config.
  - **Key Design Decisions:**
    - Build polymorphic Channel adapters in Rust (`Channel::Instagram`, `Channel::WhatsApp`, `Channel::Email`).
    - Use Server-Sent Events (SSE) or WebSockets for real-time mobile sync.
    - Apply PostgreSQL Row Level Security (`tenant_id`) on all entities for Zero Trust isolation.
    - Adhere to the PowerSync strategy for offline mobile support.

  **Implementation Prompt:**
  Implement the Core Domain and Database Schema for the Native Omnichannel Chat system in Rust (`src/server/domain/chat`).
  - Create SeaORM entities and migrations for `Inbox`, `Conversation`, `Message`, `Contact`, and `ChannelAdapter`. Ensure every table has a `tenant_id` and PostgreSQL RLS policies are enforced.
  - Implement the core service layer (e.g., `ChatService`) with methods to create messages, start conversations, and assign agents.
  - Create a unified ingress webhook handler that normalizes incoming payloads into standard `Message` structs, ready for AI consumption.
  - Write unit tests mocking external channel payloads and ensuring the database accurately maps them to the correct tenant inbox.

  **Priority:** P0
  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
