issue_title: "Architectural Design: Native Rust Omnichannel Chat System"
issue_description: |
  ## Problem Statement
  Currently, OneHumanCorp (OHC) lacks a high-performance, fully integrated native omnichannel chat system. Relying on external third-party services like the legacy open-source chat platform introduces multi-tenancy fragmentation, breaks zero-trust identity flow, and creates friction for our core personas (like Maya responding to Instagram DMs or Carlos fielding service requests via SMS). To achieve the OHC promise of an "Assistant-First Shell" where the AI unifies messages and drafts replies, we must build a native, Rust-based omnichannel engine inside the OHC monolith. This will provide seamless tenant isolation, offline-tolerant mobile access, and direct integration with our AI agent orchestrators.

  ## Research Report
  **the legacy open-source chat platform Source Code Audit**:
  We audited the open-source the legacy open-source chat platform codebase (`https://github.com/the-legacy-open-source-chat-platform/the-legacy-open-source-chat-platform`) to understand industry-standard omnichannel patterns:
  - **Data Models**: the legacy open-source chat platform centralizes around `Account` (Tenant), `Inbox`, `Conversation`, `Message`, and `Contact`. Channel-specific logic is abstracted into `Channel::*` models (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::Whatsapp`).
  - **Messaging Flow**: Webhooks hit controllers, which route payloads to channel-specific processors. These processors normalize the data into standard `Message` records associated with a `Conversation`.
  - **Real-time Engine**: WebSockets broadcast updates to subscribers using Redis Pub/Sub, ensuring clients (web/mobile) receive instant updates.
  - **AI Integration**: the legacy open-source chat platform relies on basic agent bots. OHC needs a deeper integration where AI is a first-class participant capable of drafting, approving, and sending replies automatically.

  **Competitor Analysis**:
  - Shopify Inbox: highly integrated with storefront and orders but limited omnichannel reach.
  - WeCom/DingTalk: strong unified messaging but complex enterprise setup.
  - **OHC Gap**: OHC requires the simplicity of Shopify Inbox combined with the reach of the legacy open-source chat platform, governed seamlessly by AI assistants that understand business context (sales, operations, scheduling).

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : tracks
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION ||--|| CONTACT : associated_with
      TENANT ||--o{ CONTACT : manages

      MESSAGE {
          uuid id
          uuid conversation_id
          uuid sender_id
          enum sender_type
          text content
          jsonb metadata
          timestamp created_at
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          enum status
          timestamp last_activity_at
      }
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant Channel (IG/WhatsApp)
      participant Rust Webhook Ingress
      participant Channel Adapter
      participant Message Router
      participant AI Assistant
      participant Mobile Client (Flutter)

      Customer->>Channel: Sends message
      Channel->>Rust Webhook Ingress: POST webhook payload
      Rust Webhook Ingress->>Channel Adapter: Parse & Normalize
      Channel Adapter->>Message Router: Dispatch standard Message
      Message Router->>AI Assistant: Trigger Work Triage
      Message Router-->>Mobile Client: WebSocket Broadcast
      AI Assistant->>Message Router: Draft suggested reply
      Message Router-->>Mobile Client: Broadcast Agent Draft
  ```

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed**: The user opens the app and sees a unified list of active conversations across all channels (Instagram, SMS, Web).
  - **Conversation View**: Tapping a thread opens a clean chat UI. The AI Assistant's drafted reply is prominently visible above the keyboard input, labeled with a translucent "Agent Draft" badge.
  - **One-Tap Action**: The owner (e.g., Maya) can tap "Approve & Send" to dispatch the AI draft or manually edit the response using the native mobile keyboard.
  - **Offline Tolerance**: Messages sent while offline are queued in a local SQLite database (via Flutter) and display a "pending" status token until the network is restored and the sync completes.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Evaluates incoming `Message` payloads, categorizes intent (e.g., "sales inquiry", "support complaint"), and assigns priority.
  - **Customer & Relationship Assistant**: Listens to the `Conversation` stream and automatically generates `AgentDraft` messages based on context (previous interactions, inventory, scheduling availability).

  ### Key Design Decisions
  - **Rust over Go for Chat Engine**: To achieve ultra-low latency WebSocket broadcasting and predictable memory usage under high concurrency, the core real-time message router will be built in Rust.
  - **Normalized Storage**: All channel-specific quirks are stripped at the edge (`ChannelAdapter`); internally, everything is a standard `Message` with a typed `sender`.
  - **Row-Level Security (RLS)**: PostgreSQL RLS will be enforced on all chat tables using `tenant_id` to guarantee zero-trust isolation.

  ## Implementation Prompt
  **User Facing Outcome**: The owner receives a unified feed of customer messages from all channels (Web, IG, SMS) directly in their OHC mobile app. The AI automatically drafts highly contextual replies, allowing the owner to simply review and tap "Approve & Send".

  **CUJ**: Maya (Baker) receives a new Instagram DM asking about a vegan cake. The message appears in her OHC app. The AI drafts a reply confirming availability and linking to a deposit checkout. Maya reviews the draft on her 375px mobile screen, taps "Approve", and the message is dispatched to Instagram.

  **Acceptance Criteria**:
  1. Implement the database schema for Tenants, Inboxes, Conversations, Messages, and Contacts using PostgreSQL with RLS.
  2. Build a native Rust web service exposing a webhook ingress for at least one channel (e.g., a mock Web Widget or simulated Instagram hook) and standardizes incoming payloads.
  3. Implement a real-time WebSocket broadcasting mechanism in Rust to push updates to connected clients.
  4. Develop the Flutter mobile UI (375px responsive) to display the unified inbox, chat thread, and AI draft approval flow using the OHC Premium Token translucent design system.
  5. Include full E2E Playwright/Flutter testing simulating a message receipt and AI draft approval.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
