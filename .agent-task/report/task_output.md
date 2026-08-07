issue_title: "Implement Custom Rust Omnichannel Chat System based on Chatwoot Architecture"
issue_description: |
  # Research Report: OHC Custom Rust Omnichannel Chat System

  ## Core Mandate
  As per the `AGENTS.md` and project requirements, "Chatwoot as an external third-party service, dependency, or integration is 100% RETIRED. OHC implements its own high-performance, multi-tenant omnichannel customer support & chat engine natively in Rust inside `onehumancorp/mono`."

  We need to replace the retired external Chatwoot dependency with a native Rust implementation that serves the OHC owner/operator personas (Maya, Carlos, Priya, Leo, Fatima, Nora, Jun). This system must handle multi-tenant conversations, real-time messaging, and omnichannel integrations (web widget, email, SMS, Instagram, WhatsApp) within our existing ecosystem.

  ## Problem Statement
  OHC owners need a unified inbox to manage all customer interactions seamlessly. The previous reliance on an external Chatwoot service created architectural fragmentation, data privacy concerns (lack of strict multi-tenant row-level security within OHC's control), and integration overhead. We need a unified, performant, native Rust chat system tightly integrated with OHC's core data models, AI agent workflows, and the mobile-first UX.

  ## Market & Competitor Analysis
  - **Chatwoot (Open Source Baseline):** Excellent model for omnichannel routing, conversation modeling, and agent assignment. However, it's Ruby on Rails based, which doesn't align with our high-performance Rust backend requirements.
  - **Shopify Inbox:** highly integrated with e-commerce, offering order context within the chat. OHC needs this level of integration with its own commerce features.
  - **Intercom / Zendesk:** Enterprise-grade, but often too complex for small business owners. OHC's solution must follow the "Radical Simplicity" value.

  ## Design Doc

  ### Architecture
  The system will be built as a set of Rust crates within the OHC monorepo, interacting with the existing PostgreSQL database (enforcing RLS) and Redis (for pub/sub and caching).

  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          text content
          string message_type
      }
  ```

  ### Core Components
  1.  **`ohc-chat-core` (Rust Crate):** Defines the domain models (Conversation, Message, Inbox, Contact), business logic, and database access (using SQLx or similar, strictly enforcing `tenant_id` RLS).
  2.  **`ohc-chat-api` (Rust Crate):** gRPC and REST endpoints for the frontend and external webhooks.
  3.  **`ohc-chat-realtime` (Rust Crate):** WebSocket server for real-time bi-directional communication with the OHC frontend and web widgets.
  4.  **Channel Adapters:** Modular implementations for Web Widget, Email, SMS (Twilio/Plivo), and Social (Meta Graph API).

  ### AI Agent Integration
  - **Work Triage Agent:** Hooks into the `ConversationCreated` and `MessageCreated` events to categorize and prioritize incoming chats.
  - **Customer Assistant Agent:** Drafts replies based on historical conversation context and owner preferences. It can be triggered manually by the owner or automatically for specific inbox rules.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View:** A clean, scrollable list of conversations. Unread messages have a clear visual indicator (e.g., a subtle blue dot, consistent with OHC design tokens).
  - **Conversation View:** Standard chat interface. Messages from the contact are on the left (gray bubble), owner/agent replies on the right (primary brand color bubble).
  - **AI Actions:** A prominent, yet unobtrusive "AI Draft" button near the input field.
  - **Context Panel:** A collapsible top or side panel (depending on orientation) showing the contact's previous orders or bookings.

  ## Implementation Prompt
  **Goal:** Implement the foundational data models and API for the native Rust OHC Chat System, replicating the core conversational structure of Chatwoot but adapted for our multi-tenant Rust environment.

  **Critical User Journey (CUJ):**
  1. An owner (e.g., Maya) receives a new message from a customer via the web widget.
  2. The system creates a `Contact`, an `Inbox` (if not existing), a `Conversation`, and a `Message`.
  3. The owner opens the OHC mobile app and sees the new conversation in their unified inbox list.
  4. The owner opens the conversation and sees the message text.

  **Acceptance Criteria:**
  - Database migrations for `inboxes`, `contacts`, `conversations`, and `messages` are created with strict Row-Level Security (`tenant_id`).
  - Rust models (structs) and repository layer (SQLx or equivalent) are implemented for these entities.
  - A REST API endpoint (or gRPC equivalent) is implemented to create a new message (which handles creating the conversation/contact if necessary).
  - A REST API endpoint is implemented to list conversations for a tenant.
  - Unit tests achieve 100% coverage for the new Rust code.
  - Provide instructions for testing the API endpoints locally.

  ## Priority & Scope
  - **Priority:** P0 (Blocks all other communication features)
  - **Estimated Scope:** Large (Foundation for the entire chat system)

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
