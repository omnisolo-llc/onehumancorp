issue_title: "Architecture Design: Native Rust Omnichannel Chat System"
issue_description: |
  ### Title
  Architecture Design: Native Rust Omnichannel Chat Engine

  ### Problem Statement
  OneHumanCorp (OHC) currently lacks a native omnichannel customer support and chat system, having recently deprecated the external dependency on Chatwoot. Without a deeply integrated inbox, owners like Carlos the handyman or Maya the baker cannot effortlessly turn DMs and inquiries into scheduled tasks, booked services, and deposits without leaving their central workspace. An embedded, multi-tenant Rust-based chat architecture is required to unify disparate customer communication streams (Instagram DMs, WhatsApp, Email, Web Chat) into a single, actionable OHC assistant feed.

  ### Research Report
  Based on an audit of the `chatwoot/chatwoot` repository, their system relies on the following core components that we must adapt to our multi-tenant Rust architecture:
  - **Data Models**: The core entities involve `Accounts` (Tenants in OHC), `Inboxes`, `Conversations`, `Messages`, `Contacts`, and `Channel Adapters` (e.g., Email, WhatsApp, Web Widget).
  - **Controllers & Channels**: A unified Inbox view handles message routing across channels using a publish-subscribe mechanism. WebSockets power real-time updates to connected clients via ActionCable in Chatwoot; OHC will utilize native Tokio-based async WebSockets with Redis Pub/Sub for distributed scaling.
  - **Agent Automation**: Chatwoot relies on basic rules, SLA policies, and macros. OHC will elevate this by deeply integrating our AI Assistant (Gemini Pro/GPT-4o) directly into the message pipeline to draft replies, detect intents (e.g., booking requests), and propose operational actions to the owner.

  ### Design Doc

  #### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      Tenant ||--o{ Inbox : owns
      Inbox ||--o{ Conversation : contains
      Inbox ||--o{ Channel : has
      Conversation ||--o{ Message : has
      Conversation }o--|| Contact : belongs_to
      Message ||--o{ Attachment : includes
      Tenant ||--o{ Contact : manages
  ```
  ```mermaid
  sequenceDiagram
      participant Customer
      participant Channel_Webhook
      participant Rust_Message_Bus
      participant AI_Assistant
      participant OHC_UI (Owner)

      Customer->>Channel_Webhook: Sends DM (e.g. IG/Web)
      Channel_Webhook->>Rust_Message_Bus: Ingest Message
      Rust_Message_Bus->>AI_Assistant: Request context / Draft reply
      AI_Assistant-->>Rust_Message_Bus: Return intent & draft
      Rust_Message_Bus->>OHC_UI (Owner): Real-time WebSocket update
      OHC_UI (Owner)->>Rust_Message_Bus: Approve draft & Send
      Rust_Message_Bus->>Channel_Webhook: Dispatch to network
  ```

  #### UI Wireframes & Screen Flow (375px first)
  1. **Work Feed / Inbox View**: A unified, vertically scrolling list of active conversations. Each row shows the customer avatar, channel icon (Instagram, Web), latest message snippet, and an AI-generated intent tag (e.g., `New Lead`, `Deposit Pending`).
  2. **Conversation View**:
     - Sticky header: Customer name, status, and quick action buttons (e.g., 'Book', 'Request Payment').
     - Scrollable message thread with native chat bubbles.
     - **AI Draft Panel**: Prominently displayed above the input field. The AI Assistant pre-fills a suggested response based on business context (inventory, calendar availability) that the owner can tap to send or edit.
     - Native mobile keyboard integration.

  #### Mobile UX Flow
  - The owner opens OHC on their 375px device.
  - The default "Work Triage" screen aggregates new chat messages alongside system alerts.
  - Tapping a new message from a contact smoothly slides in the Conversation View.
  - The AI has already analyzed the message, drafted a reply, and surfaced contextual business actions (e.g., "Create Quote").
  - The owner hits "Send Draft," and the Rust backend instantly syncs the update over WebSockets and dispatches it via the respective channel API.

  #### AI Agent Integration Points
  - **Triage Hook**: Every incoming message triggers an AI classification task to update conversation priority and tag intent.
  - **Drafting Hook**: AI agents subscribe to the `ConversationUpdated` event, generate localized, context-aware responses (considering owner policies, past chats, and active products), and store them as proposed `MessageDraft` entities.
  - **Operational Hook**: If the AI detects actionable intent (e.g., "I'd like to book tomorrow"), it automatically stages an internal system action (e.g., pre-fills a scheduling card) embedded in the chat timeline for the owner to approve.

  #### Key Design Decisions
  - **Rust + Tokio**: Chosen for high-throughput, low-latency concurrent message handling.
  - **Redis Pub/Sub**: Essential for cross-node WebSocket broadcasting to ensure the 375px mobile UI is immediately consistent with external webhooks.
  - **Multi-Tenant Isolation**: Enforced via row-level security (`tenant_id`) in PostgreSQL and strictly prefixed keys in Redis (`ohc:lock:{tenant_id}:...`).
  - **AI-First Inbox**: Unlike standard inboxes, OHC’s chat architecture treats the AI Assistant as an asynchronous active participant, drafting replies before the owner even opens the thread.

  ### Implementation Prompt
  **Goal:** Implement the core multi-tenant native Rust Chat system backend (Data layer + API) and the 375px-optimized Flutter UI to replace Chatwoot.
  **CUJ:**
  1. As an owner (Maya), I open the OHC mobile app and see a new Instagram DM in my Work Triage feed.
  2. I tap the conversation and see a pre-generated AI response addressing the customer's question about custom cake availability.
  3. I tap 'Approve and Send', the message appears in the chat thread instantly, and the state is resolved.
  **Acceptance Criteria:**
  - `Conversation`, `Message`, and `Inbox` Rust entities are implemented with strict `tenant_id` isolation.
  - WebSocket infrastructure is established for real-time bidirectional syncing.
  - AI Assistant hook is scaffolded to automatically generate a draft message upon receiving a new customer message.
  - Flutter UI matches the 375px design, utilizing translucent glass styling, native keyboards, and frictionless AI draft approval.
  - 100% unit test coverage and at least 5 Playwright/Flutter UI tests covering the conversation flow.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
