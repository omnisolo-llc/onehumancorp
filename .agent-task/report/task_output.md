issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OneHumanCorp (OHC) historically relied on Chatwoot as an external dependency for omnichannel customer messaging. This external dependency introduces complexity, reduces reliability (external API limits and downtime), fractures our multi-tenant Zero Trust data isolation, and degrades performance. We need a native Rust omnichannel chat system within `onehumancorp/mono` that operates invisibly as part of our AI work assistant framework to seamlessly integrate chat channels (Web, Instagram, WhatsApp, Email) into a unified inbox for our personas (Maya, Carlos, Priya, Leo, Fatima) with true row-level tenant isolation, while hiding this technical complexity behind a unified, beautifully designed interface.

  ## Research Report
  - **Chatwoot Architecture Audit**: Reviewed `chatwoot/chatwoot` source code, focusing on `app/models`, controllers, channels, and WebSockets.
    - **Key Entities**: Account (Tenant), User, Inbox, Channel (WebWidget, API, Email, WhatsApp, Facebook, Line, SMS), Contact, ContactInbox, Conversation, Message, AgentBot.
    - **Flow**: Webhooks/API calls come in -> Channels parse -> Find/Create Contact & Conversation -> Create Message -> Dispatch WebSocket events to online users -> Trigger Automations/AgentBots.
    - **WebSockets**: Uses ActionCable. Real-time updates for `conversation.created`, `message.created`, `contact.updated`.
  - **Competitive Analysis**:
    - **Shopify Inbox**: Extremely tight integration with product catalog and order data. Our system must similarly integrate tightly with OHC's Operations and Sales assistants.
    - **Stripe / Square**: Unified customer view is essential.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CHANNEL ||--o{ INBOX : routes_to
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--o{ AGENT_BOT : assisted_by
  ```

  ### Mobile UX Flow (375px First)
  - **Unified Work Feed**: The primary view is a unified feed where messages from all channels (Instagram DM, Web chat, Email) appear exactly like tasks or system alerts.
  - **Conversation View**: Tap a conversation -> See full history. Translucent glass sticky header with customer name and "AI Draft" suggestion pill.
  - **Action Sheet**: Tap "+" to attach quotes, deposit links, or calendar bookings directly into the chat.
  - **No Technical Configuration**: Connecting a channel (e.g., Instagram) is a 1-tap OAuth flow. Webhooks and API keys are completely hidden.

  ### AI Agent Integration Points
  - **Customer Assistant AgentBot**: Listens to `message.created` events on the PubSub/Redis queue. Automatically generates an AI draft response for the owner to approve, stored in the `MESSAGE` table with `status = drafted`.
  - **Work Triage**: Analyzes incoming messages to categorize them (e.g., "Inquiry", "Support", "Urgent") and adjusts the conversation priority.
  - **Operations Assistant**: Extracts dates and service requests from chat to suggest calendar events or tasks.

  ### Key Design Decisions
  - **Language & Framework**: Rust natively within `onehumancorp/mono` for maximum performance, minimal memory footprint, and tight integration with OHC's gRPC/REST APIs.
  - **Multi-Tenancy**: Strict Row-Level Security (RLS) in PostgreSQL with `tenant_id` on every table (Inbox, Conversation, Message, etc.).
  - **Real-Time Messaging**: Tokio + Tungstenite (or similar Axum WebSockets) for WebSocket connections, utilizing Redis PubSub for horizontally scaled event distribution across instances.
  - **Data Isolation**: Follows OHC's Zero Trust & SPIFFE/SPIRE identity models.

  ## Implementation Prompt
  **Goal**: Implement the core data model, gRPC/REST API, and WebSocket infrastructure for OHC's native Rust Omnichannel Chat System, replacing external Chatwoot functionality.
  **CUJ**: Maya receives an Instagram DM. It routes through the native Rust webhook endpoint, creates a Contact and Conversation in her Tenant's unified Inbox, and broadcasts a WebSocket event to her mobile app, rendering the new message in a clean, 375px-optimized UI. She sees an AI-drafted reply and taps "Send".
  **Acceptance Criteria**:
  1. Define Rust structs and PostgreSQL schemas with RLS (`tenant_id`) for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`.
  2. Implement the API endpoints to create and list conversations and messages.
  3. Implement the WebSocket server in Rust to broadcast `message.created` events to authenticated clients.
  4. Build a Playwright E2E test verifying a message sent via API appears in the UI via WebSocket without page refresh.
  5. UI must follow OHC Premium Token library (Translucent Glass materials, 375px mobile-first layout).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
