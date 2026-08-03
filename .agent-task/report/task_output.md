issue_title: "Architecture: Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Mission Queue Protocol: Architecture Brief

  ### 1. Problem Statement
  OneHumanCorp (OHC) is replacing its external Chatwoot dependency with a native Rust omnichannel chat system. Small business owners (like Maya the Baker or Carlos the Handyman) need a unified inbox to handle Instagram DMs, SMS, WhatsApp, and Web Widget chats. Relying on an external monolithic service breaks our strict multi-tenant Zero Trust boundaries and prevents deep, invisible AI coordination (e.g., our Customer & Relationship Assistant auto-drafting replies). We need a high-performance, strictly isolated Rust architecture that scales edge-caching and real-time WebSockets effortlessly.

  ### 2. Research Report
  Based on an audit of `chatwoot/chatwoot` source code:
  - **Core Models**: Chatwoot centers around `inboxes` (channels), `conversations` (threads), `messages`, and `contacts`.
  - **Tenancy**: Chatwoot uses `account_id` uniformly across models. OHC will enforce `tenant_id` via PostgreSQL Row Level Security (RLS) on all entities.
  - **Channels**: Chatwoot relies on polymorphic channel adapters. OHC will use gRPC internal services or NATS streams for channel ingestion (e.g., Twilio, Meta, WebWidget).
  - **Real-time**: Chatwoot uses ActionCable (`RoomChannel`). OHC will use `axum` WebSockets backed by Redis PubSub/NATS to distribute events across pods to provide similar `presence.update` and message broadcasting capabilities.

  ### 3. Design Doc (Architecture Design)
  **Architecture Diagram:**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : hosts
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
          jsonb config
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
          uuid assignee_id
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string sender_type
          string message_type
      }
  ```

  **Mobile UX Flow (375px First):**
  - **Unified Inbox View**: A simple, unified list of active conversations categorized by 'Requires Action', 'Pending', and 'Resolved'. Uses clean UniFi-style spacing with Translucent Glass headers.
  - **Conversation View**: Tap a thread to view full chat history. The screen features a sticky bottom native-keyboard-friendly input area. AI-suggested draft replies float above the input box as translucent chips.
  - **Interactivity**: Touch targets > 44px. The UI handles flaky networks by showing an optimistic "sending" state with a clear visual fallback if the WebSocket disconnects.

  **AI Agent Integration Points:**
  - **Work Triage**: On `ohc_message` insert, a PostgreSQL `SKIP LOCKED` job queues the message for AI triage to categorize intent and update conversation urgency.
  - **Drafting**: Customer Assistant subscribes to NATS `conversation.updated` events, fetching previous context to generate draft responses.

  ### 4. Implementation Prompt
  **To the Implementer:**
  Implement the core native Rust Chat Engine for OHC.
  1. Define SeaORM entities for `Inbox`, `Conversation`, `Message`, and `Contact` applying `tenant_id` Row Level Security strictly.
  2. Implement an `axum` WebSocket handler that authenticates via OIDC tokens, subscribes to a NATS subject for real-time events, and streams incoming messages to the client.
  3. Provide a REST/gRPC API for appending new messages and listing conversation history.
  4. Ensure ZERO mock data is used. Empty states must be truthful.
  5. **Verification**: Write full unit tests (100% coverage) for the SeaORM entities and a Playwright E2E test that drives a real browser session to open the inbox, create a mock incoming request (via API), and visually verify the new message appears without page reload.

  ### 5. Priority & Scope
  **Priority**: P0 (Critical - Blocks unified communication capability)
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
