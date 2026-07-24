issue_title: "Implement Native Rust Omnichannel Chat Inbox Foundation to Replace Chatwoot"
issue_description: |
  ## Problem Statement
  We have retired Chatwoot as an external 3rd-party service to ensure strict multi-tenant data isolation, reduce operational overhead, and unify our platform. However, our core personas (like Maya the Baker handling Instagram DMs, or Carlos the Handyman handling service inquiries) desperately need a unified inbox. They need a native omnichannel work assistant that pulls together web widget chat, email, Instagram, WhatsApp, and SMS into one centralized feed, with AI agent capabilities natively embedded. The gap is the missing OHC-native, Rust-backend unified messaging foundation that mirrors Chatwoot's omnichannel flexibility but enforces OHC's strict Zero-Trust tenant isolation and mobile-first 375px design.

  ## Research Report & Feature Benchmarking
  I have cloned and audited the open-source `chatwoot/chatwoot` repository to benchmark its core capabilities.
  Key findings from Chatwoot's Ruby on Rails architecture:
  - **Conversations & Messages**: `Conversation` maps to `account_id` and `inbox_id`, tying together multiple `Message` records. Messages support rich attachments, AI formatting (`LlmFormattable`), and complex filtering.
  - **Inboxes & Channels**: The `Inbox` model is polymorphic over channels (`Channel::WebWidget`, `Channel::Email`, `Channel::Whatsapp`, etc.), allowing seamless routing.
  - **Contacts**: `Contact` unifies user identity across multiple sessions and channels via `ContactInbox`.

  **OHC Adaptation**: OHC's Rust implementation will map these into strict tenant-isolated entities. Every table (`inboxes`, `channel_connections`, `contacts`, `conversations`, `messages`) will have a mandatory `tenant_id` and PostgreSQL Row-Level Security (RLS). We will consolidate legacy `inbox_messages` into this new canonical omnichannel domain model. We will implement PowerSync for the mobile client to provide offline-tolerant chat capabilities.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      TENANT ||--o{ CONVERSATION : owns
      TENANT ||--o{ MESSAGE : owns
      INBOX ||--o{ CHANNEL_CONNECTION : configures
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONTACT_IDENTITY : authenticates
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--o{ PARTICIPANT : involves

      INBOX {
          uuid id
          uuid tenant_id
          string name
          boolean is_active
      }
      CHANNEL_CONNECTION {
          uuid id
          uuid inbox_id
          string provider_type
          jsonb capabilities
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
          int priority
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string sender_type
          timestamp delivered_at
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Triage Feed (Home)**: The owner opens the app. The bottom tab "Inbox" shows an unread badge.
  2. **Inbox List**: Tapping "Inbox" reveals a translucent glass-styled list of active conversations. Each row shows the customer avatar, channel icon (e.g., IG, Web), a snippet of the latest message, and a timestamp. Layout targets 375px; touch targets are minimum 44x44px.
  3. **Conversation Thread**: Tapping a conversation opens the chat view. Messages are rendered in iOS-style bubbles. Sticky header shows customer name and AI-summarized context ("Requested vegan cake quote").
  4. **AI Reply Action**: At the bottom, alongside the standard native keyboard input, an "AI Suggest" button is prominently displayed. Tapping it generates a contextual draft in the text field based on OHC's Knowledge Assistant.

  ### AI Agent Integration
  - **Work Triage Agent**: Hooks into the unified webhook ingress, analyzes incoming message intent, sets the conversation priority, and assigns preliminary tags.
  - **Customer Assistant**: When the owner views a conversation, the agent proactively drafts a suggested reply based on past interactions, tenant knowledge base, and open orders.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Your objective is to implement the foundational Rust backend and database schema for the native OHC Omnichannel Inbox, replacing Chatwoot.
  1. Create the PostgreSQL migration establishing the canonical schema for `inboxes`, `channel_connections`, `contacts`, `conversations`, and `messages`. You MUST enforce `tenant_id` on all tables and apply `ENABLE ROW LEVEL SECURITY`.
  2. Implement the Rust backend domain models and Axum REST endpoints (or gRPC handlers) to perform CRUD operations on these entities. Ensure authentication relies exclusively on SPIFFE/SPIRE and extracts the `tenant_id` securely.
  3. Build a basic Flutter/Next.js frontend view for the 375px-wide Inbox list and Conversation thread using our OHC Premium Token translucent glass design system.
  4. Ensure 100% unit test coverage for the new Rust module and add a Playwright E2E test verifying the flow from creating a conversation to sending a message. All `bazel test //...` runs MUST pass.

  **Acceptance Criteria**:
  - The canonical database schema accurately reflects the ER diagram with strict RLS.
  - Rust handlers correctly isolate data by `tenant_id`.
  - The UI correctly displays the conversation list and thread without horizontal scrolling on mobile.
  - No Chatwoot external API calls are made.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
