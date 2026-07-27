issue_title: "Implement Native Rust Omnichannel Chat System (Legacy App Replacement)"
issue_description: |
  ## Mission Queue Protocol Brief
  **Problem Statement**:
  Currently, OneHumanCorp (OHC) is missing a unified, native omnichannel messaging system for our core owner/operator personas (Maya, Carlos, Priya, Leo, Fatima). External dependencies like legacy omnichannel platform are 100% RETIRED to reduce operational complexity, guarantee strict multi-tenant data isolation, and deeply integrate AI agents directly into the workfeed. Owners currently have disjointed communication streams across Instagram DMs, SMS, WhatsApp, and Web, resulting in missed leads and inefficient follow-ups. We need a native, high-performance omnichannel inbox built in Rust to unify messages, apply AI automated responses, and feed directly into the OHC operations dashboard.

  ## Research Report
  Based on an audit of the legacy open source omnichannel app source code (`https://github.com/legacy omnichannel platform/legacy omnichannel platform`) and industry standards (Shopify Inbox, HubSpot):
  - **Data Models**: The legacy app relies on heavily decoupled models like `Account`, `Inbox`, `Conversation`, `Message`, and `Contact`. Specifically, they use polymorphic channel definitions (`Channel::Api`, `Channel::FacebookPage`, `Channel::Sms`, `Channel::Whatsapp`, etc.) linked to an `Inbox`.
  - **Real-Time Delivery**: The legacy app handles real-time messaging via WebSockets (ActionCable) broadcasting payloads. OHC requires a Rust-based WebSocket system (likely using `axum` or `actix-web` with `tokio-tungstenite`) backed by Redis pub/sub for scaling across nodes.
  - **Competitor Patterns**: Shopify Inbox seamlessly ties customer conversations directly to their cart and order history. Our native chat system must similarly tie conversations to OHC's ledger, bookings, and customer profiles contextually.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      CHANNEL ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }|--|| AGENT : created_by_or_processed_by

      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          string provider_type
          json credentials
      }
      CONVERSATION {
          uuid id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          text content
          string sender_type
      }
  ```

  ### Mobile UX Flow (375px first)
  1. **Work Triage Feed**: A combined inbox view showing the latest active conversations from all channels. Each card displays the customer's avatar, channel icon (e.g., IG, SMS), and the latest message snippet.
  2. **Conversation Thread**: Tapping a thread opens the chat view.
     - Sticky top header with customer name and context (e.g., "Maya's Bakery - Active Order #102").
     - Scrollable message list with clear visual distinction between customer messages, owner replies, and AI-drafted messages (marked with a translucent "AI Draft" badge).
     - Bottom sticky input area featuring a text field, an attachment button, and a prominent "AI Suggestion" button.
  3. **Context Pane**: A swipe-left drawer showing customer history, previous orders, and pending tasks without leaving the chat.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Automatically reads incoming messages, categorizes their intent (e.g., "Lead", "Support", "Complaint"), and assigns priority.
  - **Customer & Relationship Assistant**: Drafts replies in the background. Unread messages generate an AI draft that the owner can approve with a single tap.
  - **Operations Assistant**: Extracts structured data (e.g., booking dates, order items) from the natural language conversation and surfaces quick actions (e.g., "Create Quote", "Schedule Visit").

  ### Key Design Decisions
  - **Native Rust Services**: Ensures high performance, safe concurrency, and low memory footprint, scaling easily for thousands of concurrent WebSocket connections.
  - **Strict Multi-Tenancy**: All messages, conversations, and contacts will have mandatory `tenant_id` fields enforced via PostgreSQL Row Level Security (RLS) to guarantee Zero Trust isolation.
  - **Unified Channel Abstraction**: A single internal message format that adapter layers (IG, SMS, Email, Web Widget) parse into. This decouples core logic from third-party API changes.

  ## Implementation Prompt
  **User-Facing Outcome**: Provide the owner with a single, highly responsive "Unified Inbox" on their mobile app (Flutter PWA) where they can view, manage, and reply to customer inquiries from any channel (Instagram, SMS, Web Widget) without switching apps. AI agents automatically draft responses to common inquiries and extract actionable tasks directly from the chat.

  **Critical User Journey (CUJ)**:
  1. The owner opens the OHC app and navigates to the Inbox tab.
  2. The owner sees a new Instagram DM from a customer asking about booking a service.
  3. The conversation thread shows the customer's message alongside an AI-generated draft reply including a quote link.
  4. The owner taps "Approve & Send". The message is dispatched immediately, and a real-time WebSocket update confirms delivery.

  **Acceptance Criteria**:
  - The unified inbox UI correctly renders across 375px mobile screens.
  - Rust-based backend successfully handles real-time bidirectional messaging via WebSockets.
  - The system supports polymorphic channel ingestion (e.g., mock endpoints for SMS and Web).
  - AI Assistant seamlessly drafts contextual replies based on the conversation history.
  - E2E Playwright tests explicitly verify the conversation creation, message sending, and AI draft approval flows without any mock data in the UI.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
