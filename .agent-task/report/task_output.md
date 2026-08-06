issue_title: "Implement Custom Rust Omnichannel Chat System to Replace ChatPlatform"
issue_description: |
  **Problem Statement**:
  OHC currently relies on external ChatPlatform services for its omnichannel chat capabilities, or has incomplete native implementations. The `AGENTS.md` and project requirements explicitly state that "ChatPlatform as an external third-party service, dependency, or integration is 100% RETIRED." We need a native Rust implementation of a high-performance, multi-tenant omnichannel customer support & chat engine built directly inside `onehumancorp/mono`.

  **Research Report**:
  - The ChatPlatform source code repository reveals a complex architecture with models for Accounts, Users, Channels, Contacts, Conversations, and Messages.
  - ChatPlatform handles multi-tenancy, various channel types (API, Email, Facebook, Instagram, Line, SMS, Telegram, TikTok, Twilio, Twitter, Web Widget, WhatsApp).
  - OHC currently has fragmented native implementations in `src/server/services/chat`, `src/server/services/inbox`, and `src/server/services/omnichannel_service.rs` that attempt to replicate some of this functionality but lack a unified, comprehensive architecture mirroring ChatPlatform's capabilities.
  - The goal is to design a unified Rust-based native chat engine that replaces these disparate attempts and fully deprecates any ChatPlatform integrations.

  **Design Doc**:
  - **Architecture Diagram (Mermaid)**:
    ```mermaid
    erDiagram
      Tenant ||--o{ Conversation : has
      Tenant ||--o{ Contact : has
      Tenant ||--o{ ChannelAdapter : configures
      Conversation ||--o{ Message : contains
      Conversation }|--|| Contact : "is with"
      Message ||--o| AiDraft : "has draft"
      Message {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string sender_type
          string content
          json metadata
      }
      Conversation {
          uuid id
          uuid tenant_id
          uuid contact_id
          uuid assignee_id
          string channel_type
          string status
      }
      Contact {
          uuid id
          uuid tenant_id
          string name
          string phone
          string email
          json custom_attributes
      }
      ChannelAdapter {
          uuid id
          uuid tenant_id
          string channel_type
          json credentials
          boolean active
      }
    ```
  - **Mobile UX Flow (375px first)**:
    - Unified Inbox view showing active conversations across all channels.
    - Conversation view with real-time message updates.
    - AI draft suggestions visible as actionable buttons above the compose area.
    - Seamless transition between channels (e.g., SMS to Web Chat) within a contact's context.
  - **AI Agent Integration**:
    - The `Operations` and `Customer Service` AI departments will hook into the message creation lifecycle.
    - When a customer sends a message, an `AiDraft` is generated asynchronously in the background.
    - The draft is presented to the owner for approval before sending.
  - **Key Design Decisions**:
    - Build on top of the existing PostgreSQL structure with strict Row-Level Security for `tenant_id`.
    - Unify `ChatInbox`, `UnifiedThread`, and `OmniChannelRepo` structures into a single, canonical `OmnichannelChat` module.
    - Implement a WebSocket-based real-time event system for immediate UI updates.
    - Ensure zero-trust access using SPIFFE/SPIRE for inter-service communication.

  **Implementation Prompt**:
  - Implement a complete native Rust omnichannel chat system in `src/server/services/omnichannel_chat/`.
  - Create database schemas for `chat_contacts`, `chat_channels`, `chat_conversations`, `chat_messages`, and `chat_ai_drafts` with strict multi-tenant isolation.
  - Develop gRPC and REST APIs for the mobile/web frontend to interact with the inbox and messages.
  - Ensure real-time WebSocket support for message delivery.
  - Add comprehensive unit tests (100% coverage) and Playwright E2E tests covering the complete flow from message ingestion to AI draft generation and owner approval.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
