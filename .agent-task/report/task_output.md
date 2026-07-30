issue_title: "Native Rust Omnichannel Chat System - Web Widget Channels & Unified Inbox"
issue_description: |
  # Native Rust Omnichannel Chat System: Web Widget & Unified Inbox Architecture

  ## Problem Statement
  OHC requires a completely native, high-performance omnichannel communication platform to replace third-party dependencies (like Chatwoot). The current implementation lacks the architecture required for a unified inbox that supports multiple channels (WhatsApp, Web Widget, Email) securely isolated by `tenant_id`. Business owners like Maya (baker) and Carlos (handyman) need to respond to all their inquiries seamlessly from the OHC mobile or web app, without managing multiple separate communication silos.

  ## Research Report
  Based on an audit of leading customer engagement platforms (such as Shopify Inbox, Intercom, and Chatwoot), an effective omnichannel chat system relies on:
  1.  **Unified Data Model:** Centralized models for `Inbox`, `Conversation`, `Message`, and `Contact`.
  2.  **Channel Adapters:** Interfaces bridging distinct external channels (e.g., WhatsApp, Web Widget WebSockets) to the unified data model.
  3.  **Real-Time Subscriptions:** WebSocket connections handling real-time push events back to the client interface (agent UI) for instantaneous communication.
  4.  **Multi-Tenancy:** Hard data segregation by `tenant_id` at every layer (database RLS, API boundary).

  A source code review of Chatwoot reveals the need for distinct controllers for managing conversations, building channel adapters, maintaining websocket channels for real-time delivery, and applying automated SLA policies and macros.

  ## Design Doc
  ### Architectural Overview
  The native Rust system will consist of core microservices inside `onehumancorp/mono`.

  **Key Components:**
  - **Omnichannel Service (Rust):** The central service orchestrating channels, routing, and message persistence.
  - **API Layer:** Exposes REST and gRPC endpoints for managing inboxes, contacts, and retrieving conversation history.
  - **Channel Adapters (Rust Crates):**
      - `channel-whatsapp`: Handles Meta Graph API integration.
      - `channel-web-widget`: Manages WebSocket connections for website visitors.
  - **Real-Time Event Bus:** Redis Pub/Sub for broadcasting events (e.g., `message.created`, `conversation.updated`) across service instances and to connected WebSocket clients.

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : manages
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : tracks
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          uuid inbox_id
          string type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string phone_or_email
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          text content
          timestamp created_at
      }
  ```

  ### Data Model & Invariants
  All tables MUST implement Row Level Security (RLS) via `tenant_id`.
  - `Inbox`: Configuration for a specific channel (e.g., Maya's Instagram DM inbox).
  - `Channel`: Specific configuration for the adapter (e.g., WhatsApp credentials, Web Widget domain allowlist).
  - `Contact`: The end-user (e.g., the customer inquiring about a cake).
  - `Conversation`: A continuous thread between a `Contact` and an `Inbox`.
  - `Message`: Individual messages within a `Conversation`.

  ### AI Agent Coordination
  - **Work Triage Agent:** Hooks into the `message.created` event via PostgreSQL `SKIP LOCKED` job queue. It analyzes intent, categorizes priority, and drafts suggested replies.
  - **Operations Agent:** If the intent is booking-related, it pre-fetches calendar availability to include in the draft.

  ### Mobile UX Flow (375px First)
  - **Unified Inbox View:** A clean, scrollable list of active conversations. Each row shows the avatar, last message preview, channel icon (e.g., WhatsApp logo), and an unread badge.
  - **Conversation View:** Standard chat interface. Messages grouped by date. Input field supports text, image upload (optimized via WebP), and a quick-select for AI-generated suggested replies.
  - **Interaction:** Swiping right on a conversation archives it. Tapping a button reveals quick actions (Create Quote, Book Appointment).

  ## Implementation Prompt
  **Goal:** Implement the core data models, API endpoints, and a basic Web Widget Channel Adapter for the Native Rust Omnichannel Chat System.
  **CUJ:** A business owner (e.g., Maya) opens her OHC app and sees a unified list of active conversations. She clicks into a conversation originating from her website widget and sends a reply. The reply is delivered in real-time to the website visitor via WebSockets.

  **Acceptance Criteria:**
  1. Define Rust data structs and database schema (with RLS) for `Inbox`, `Channel`, `Contact`, `Conversation`, and `Message`.
  2. Implement REST API endpoints for listing conversations and sending messages.
  3. Build the foundational Web Widget channel adapter with WebSocket support for real-time bidirectional communication.
  4. Ensure 100% test coverage for the implemented logic.
  5. The UI (simulated or real) must demonstrate receiving and sending a message through the widget channel.

  **Estimated Scope:** Large
  **Priority:** P0

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
