issue_title: "[Native Rust Implementation] Omnichannel Unified Inbox Architecture & Missing Core Capabilities"
issue_description: |
  # Problem Statement
  Small business owners like Carlos (handyman) or Maya (baker) receive customer inquiries across fragmented channels (Instagram DMs, WhatsApp, SMS, Web Chat). Currently, OHC lacks a high-performance, native Rust unified inbox to aggregate these messages. The previous dependency on the external Chatwoot service has been entirely RETIRED per engineering standards. Without a native OHC omnichannel architecture, owners miss messages, lose sales context, and cannot trigger our AI Ambassador for autonomous replies. We need to implement a native Rust replacement for Chatwoot's core inbox functionality directly within `onehumancorp/mono`.

  # Research Report
  Based on a deep audit of the retired Chatwoot repository and current OHC codebase:
  - **Chatwoot's Architecture (Ruby on Rails):** Handled webhooks from FB/WhatsApp/Email, unified them into `Conversation` and `Message` models, and dispatched them via WebSockets (ActionCable). It used a multi-tenant `Account` structure with `Inbox` and `Channel` abstractions.
  - **The OHC Native Rust Solution:** We must replicate this model natively. Using Rust, we can achieve significantly higher concurrency for webhook processing and WebSocket connections than Rails.
  - **Missing Rust Crates/Modules:** OHC currently lacks:
    1. A `Conversation` and `Message` schema that supports rich multi-channel attachments and threading.
    2. An `Inbox` and `ChannelAdapter` trait system to normalize payloads from WhatsApp Cloud API, Instagram Graph API, and our own web widget.
    3. A WebSocket server (e.g., using `axum` and `tokio-tungstenite`) to push real-time updates to the mobile/web frontend without polling.
  - **AI Synergy:** By bringing the data model native, our AI Ambassador can directly query the tenant's PostgreSQL schema using RAG, without making expensive API calls to an external Chatwoot instance.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: IG/WA/Email] -->|HTTP POST| B(Axum Webhook Gateway)
      B --> C{Channel Adapter Normalization}
      C -->|Insert| D[PostgreSQL: Messages & Conversations]
      D --> E[Redis / Event Bus]
      E --> F(Tokio WebSocket Server)
      F -->|Real-time Push| G[OHC Flutter App 375px]
      E --> H(The Ambassador AI Agent)
      H -->|Drafts Reply| D
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Mobile Inbox List:** A clean, UniFi-style card list showing active conversations. Each card shows an avatar (channel icon), customer name, snippet of the last message, and a status token (e.g., "AI Drafted", "Unread").
  - **Conversation View:** Standard chat interface. Messages from the owner/AI are on the right (with an AI sparkle icon if auto-drafted), customer on the left.
  - **Translucent Glass UI:** The header and message input area use iOS-style translucent blur over the conversation history. Native mobile keyboard behavior is critical.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the `message.created` event bus. When a customer sends a message, The Ambassador reads the conversation history and customer profile, drafts a response, and saves it with `status: draft`. The frontend displays this for the owner to 1-tap approve.

  ### Key Design Decisions
  - **Native Rust Axum/Tokio:** Replaces Ruby/ActionCable for massive scalability and lower memory footprint per tenant.
  - **Strict Multi-Tenancy:** Row-level security (RLS) on `messages` and `conversations` using `tenant_id` is mandatory to ensure cross-tenant data isolation.
  - **Adapter Pattern:** A generic Rust trait `ChannelAdapter` will standardize the diverse JSON payloads from WhatsApp, Instagram, etc., into a unified `Message` struct before saving.

  # Implementation Prompt
  **User-Facing Outcome:** The owner opens the OHC mobile app and sees all messages from Instagram, WhatsApp, and Web Chat in a single, lightning-fast native inbox. AI drafts are instantly visible, waiting for a single tap to send.
  **CUJ & Acceptance Criteria:**
  1. Define the Rust data structs and Diesel/SQLx migrations for `Inbox`, `Conversation`, `Contact`, and `Message` tables, including `tenant_id` for RLS.
  2. Implement an Axum-based WebSocket server that authenticates via OHC's existing SPIFFE/JWT tokens and allows clients to subscribe to conversation updates.
  3. Create a dummy webhook ingestion endpoint that accepts a normalized JSON payload, saves it to the database, and broadcasts it over the WebSocket to connected clients.
  4. Build the Flutter UI (mobile-first 375px) that connects to this new native WebSocket, displaying messages in a unified list with Translucent Glass styling.
  5. **Verification:** Playwright E2E tests MUST prove that simulating a webhook POST causes the message to immediately appear in the Playwright-driven Flutter web UI without a page refresh.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, rust-omnichannel]
assignees: []
