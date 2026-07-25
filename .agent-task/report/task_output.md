issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  Currently, the OneHumanCorp (OHC) platform lacks a unified, highly scalable omnichannel chat and customer support engine natively built into the platform. With the complete retirement of Chatwoot as an external service dependency, owners like Maya (the baker selling via Instagram DMs) and Carlos (handyman handling inquiries on his Android) need a seamless, integrated, and reliable inbox that bridges multiple channels (Instagram, SMS, Email, Web Widget, WhatsApp). The gap is that the platform cannot handle multi-tenant messaging reliably without an embedded Rust-based communication layer that understands OHC's unique Agent AI context, Zero Trust security model, and mobile-first latency constraints.

  ## Research Report
  **Chatwoot Codebase Audit:**
  Based on an in-depth audit of the `chatwoot/chatwoot` GitHub repository, their architecture relies heavily on:
  - `Inbox` mapped to `Account` (Tenant) and specific `Channel` (e.g., API, Web Widget, SMS, Email, Facebook, Instagram, Twilio).
  - `Conversation` mapped to an `Inbox`, `Contact`, and `Account`, tracking states such as `open`, `snoozed`, and `resolved`.
  - `Message` tracking content, `message_type` (incoming/outgoing), and `content_type` (text, rich media).
  - `Contact` mapping to `Account` and tracking customer properties (name, email, phone).

  Chatwoot's model is comprehensive but relies on Ruby on Rails patterns. For OHC, replacing this with a Rust native implementation allows for higher concurrency, robust multi-tenancy at the data access level (row-level security in PostgreSQL), and integrated AI worker capabilities.

  **Industry Comparison:**
  - **Shopify Ping / Inbox:** Unifies chat, social media, and email for merchants in a mobile-optimized view.
  - **Stripe:** Provides real-time synchronization with high availability using edge-caching and efficient polling/websockets.
  - **OHC Native Rust Implementation:** By bringing this in-house in Rust (`onehumancorp/mono`), OHC can deeply integrate AI agents to auto-draft replies, contextually link invoices or bookings directly into the conversation, and achieve strict tenant isolation using SPIFFE/SPIRE.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT {
          uuid id PK
          string name
      }
      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          string channel_type
      }
      CHANNEL_ADAPTER {
          uuid id PK
          uuid inbox_id FK
          jsonb credentials
          string type
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string identifier
          string name
          string phone_number
      }
      CONVERSATION {
          uuid id PK
          uuid tenant_id FK
          uuid inbox_id FK
          uuid contact_id FK
          string status
          datetime last_activity_at
      }
      MESSAGE {
          uuid id PK
          uuid tenant_id FK
          uuid conversation_id FK
          string sender_type
          string content
          string message_type
      }

      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_by
      TENANT ||--o{ CONTACT : manages
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
  ```

  ### UI Wireframes / Mobile UX Flow (375px first)
  1. **Unified Inbox View (375px):**
     - Apple-style translucent glass header with segmented controls: "Unread", "Mine", "All".
     - List of conversation cards. Each card displays: Contact Avatar (or initial), Contact Name, Channel Icon (e.g., Instagram, SMS), snippet of the last message, and a timestamp. Unread messages have a bold typography token and a primary color dot indicator.
  2. **Conversation Thread View (375px):**
     - Sticky header displaying Contact Name and Channel.
     - Scrollable message thread (iMessage/WhatsApp style layout).
     - **AI Draft Panel:** A translucent card directly above the input bar suggesting an AI-drafted reply based on the context (e.g., "Yes, we do vegan cakes! [Send] [Edit]").
     - Native mobile keyboard integration. Bottom input area includes an attachment button and a 'Send' button.
  3. **Contact Context Sheet (Bottom Sheet):**
     - Swiping left or tapping the header opens a bottom sheet showing the contact's recent orders, LTV, and upcoming bookings.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant:** Listens to `MessageCreated` events on the Rust event bus. When an incoming message arrives, it retrieves the contact context, analyzes intent, and pushes a draft reply to the conversation.
  - **Work Triage:** Flags conversations with negative sentiment or high urgency for immediate owner attention.
  - **Operations Assistant:** When a message mentions scheduling, it surfaces booking action buttons natively inside the chat interface.

  ### Key Design Decisions
  - **Native Rust & WebSockets:** Use a native Rust WebSocket server for real-time `Message` delivery to the Flutter frontend, achieving sub-50ms latency.
  - **Multi-Tenant Strict Isolation:** Every table (`INBOX`, `CONVERSATION`, `MESSAGE`) includes `tenant_id` combined with PostgreSQL RLS (Row Level Security).
  - **Channel Adapters (Strategy Pattern):** Rust implementations of adapters for Instagram, SMS, WhatsApp, Web Widget.
  - **Event-Driven Architecture:** Decouple inbound message parsing from AI processing by publishing events (`conversation.created`, `message.received`) to a background job queue (PostgreSQL `SKIP LOCKED` or Redis).

  ## Implementation Prompt
  **Goal:** Implement the foundational Native Rust Omnichannel Chat backend and the accompanying mobile-first Flutter UI for the unified inbox.
  **Persona:** Maya (Baker) who receives custom cake inquiries on Instagram and Web Widget.
  **CUJ:** Maya opens the OHC app on her iPhone (375px screen), navigates to the Inbox, sees a new conversation from an Instagram DM, reviews an AI-drafted reply, edits it slightly, and hits send. The reply goes out through the channel adapter.
  **Acceptance Criteria:**
  - Create Rust core models and database schemas for `Inbox`, `Conversation`, `Message`, and `Contact` with strict RLS (tenant isolation).
  - Implement basic REST/gRPC APIs for fetching the unified inbox list and message threads.
  - Implement the Flutter UI components: Inbox List, Conversation Thread view with Translucent Glass materials.
  - Ensure zero mock data in the UI (must fetch from the Rust backend).
  - 100% unit test coverage for new Rust modules.
  - Add at least 5 Playwright E2E tests verifying the inbox rendering, conversation selection, and message sending loop (using test-mode channel adapters).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []