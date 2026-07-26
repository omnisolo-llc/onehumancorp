issue_title: "Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  **Title**: Architect and Implement Native Rust Omnichannel Chat Engine

  **Problem Statement**:
  Currently, the system's reliance on external services like Chatwoot introduces unnecessary complexity, limits native multi-tenant data isolation, and restricts deep integration with OHC's core AI assistant capabilities. Non-technical owners (like Maya the baker and Carlos the handyman) need a seamless, invisible system that unifies DMs, WhatsApp, SMS, and web chat into one inbox that they, and their AI agents, can easily manage. The external Chatwoot dependency is now 100% retired, requiring a high-performance, native Rust omnichannel chat system within OHC's monorepo that provides parity with Chatwoot while adhering to strict multi-tenant isolation and mobile-first UX.

  **Research Report**:
  Based on an audit of the Chatwoot source code and current industry standards:
  - **Data Model Parity**: Chatwoot uses entities like `Account` (Tenant), `Inbox`, `Conversation`, `Contact`, `Message`, and `ChannelAdapter`.
  - **Omnichannel Architecture**: It abstracts external channels (WebWidget, API, WhatsApp, Instagram) into Inbox channels.
  - **Real-Time Delivery**: Chatwoot relies on ActionCable (WebSockets) for real-time delivery to agents.
  - **Competitor Analysis**: Platforms like Shopify Inbox, Zendesk, and Front have native integrated inboxes. Shopify Inbox integrates directly with its store data, enabling rich cards (e.g., product recommendations, orders) within the chat stream. OHC must follow this approach, integrating deeply with our Catalog, CRM, and AI agent layers.

  **Design Doc**:

  *Architecture Diagram*:
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : has
      CHANNEL_ADAPTER }|--|| EXTERNAL_PROVIDER : bridges
      MESSAGE }o--o| AI_AGENT : drafted_by
  ```

  *System Architecture*:
  - **Rust Backend**: Implementation in `src/server/integrations/chat` (or similar).
  - **Data Store**: PostgreSQL with strict Row Level Security (RLS) on `tenant_id` for `inboxes`, `conversations`, `messages`, and `contacts`.
  - **Real-time**: Axum WebSockets integrated with Redis/NATS PubSub to push events (new messages, typing indicators) to connected clients.
  - **Channel Adapters**: Rust traits defining how to send/receive messages from channels (Email, SMS via Twilio, Web Widget, WhatsApp).

  *UI Wireframes & Mobile UX Flow (375px first)*:
  - **Unified Inbox Screen**: A scrollable list of conversations. Each list item shows contact name, avatar, snippet of the last message, channel icon (e.g., Instagram, Web), and unread indicator.
  - **Conversation Screen**:
    - Header: Contact name and quick action buttons (Call, View Profile).
    - Message Stream: Bubbles for incoming and outgoing messages. System messages (e.g., "AI drafted a response") appear inline with translucent glass styling.
    - Composer: Native mobile keyboard area with attachment support and an "AI Assist" button to generate replies based on context.
  - **Mobile Parity**: Ensure swipe-to-go-back, 44x44px touch targets for all buttons, and a clean interface without horizontal scroll.

  *AI Agent Integration Points*:
  - **Work Triage**: On incoming messages, an AI triage agent categorizes the message intent (e.g., "Quote Request", "Support") and sets priority.
  - **Drafting Responses**: The Customer & Relationship Assistant agent can automatically draft proposed replies to new messages. These are saved in the database with a specific `status = drafted_by_ai` and shown in the UI for the owner to approve or edit before sending.
  - **Contextual Memory**: AI agents can read the `Contact`'s historical conversations and tags to personalize drafts.

  *Key Design Decisions*:
  - **Deep Native Integration**: Building natively in Rust allows zero-latency interaction with our AI queue and CRM data, avoiding webhook lag associated with external systems like Chatwoot.
  - **Strict Multi-Tenancy**: Every database table will include a `tenant_id` column protected by Postgres RLS to ensure 100% data isolation between owners.

  **Implementation Prompt**:
  Implement the core native Rust omnichannel chat system to replace Chatwoot.
  1. Define the SQL schemas for `inboxes`, `channels`, `conversations`, `messages`, and `contacts` with `tenant_id` and PostgreSQL Row-Level Security.
  2. Implement the Rust service layer with Axum endpoints to create conversations, send messages, and fetch history.
  3. Implement the WebSockets (Axum + tokio-tungstenite) layer to broadcast real-time message events to connected 375px-first clients.
  4. Build the unified inbox UI using the OHC Premium Token library with translucent materials and UniFi-style layouts, ensuring all touch targets are at least 44x44px.
  5. Include tests proving RLS isolation, websocket broadcasting, and UI interactions (using Playwright).

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
