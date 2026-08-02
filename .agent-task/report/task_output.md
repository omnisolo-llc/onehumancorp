issue_title: "[Research] Architect OHC Native Omnichannel Chat & Web Widget (legacy chat platform Replacement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing its dependency on legacy chat platform as an external third-party service and migrating to a completely native Rust-based omnichannel chat engine inside `onehumancorp/mono`. Currently, the foundation models (`chat_inboxes`, `chat_channels`, `chat_contacts`, `chat_conversations`, `chat_messages`) exist in `src/server/migrations/1009_native_omnichannel_chat.sql` and `src/server/services/chat/models.rs`, but the broader architecture to handle multiple channels (Web Widget, Email, Instagram, WhatsApp, SMS, API), websocket real-time messaging, and agent assignment is missing. OHC needs a robust, scalable system to replace legacy chat platform's functionality natively to serve non-technical owners like Maya and Carlos, seamlessly capturing leads and customer context without complex configuration.

  ## Research Report
  - **legacy chat platform Source Code Audit**: Investigated the schema and channel models from `https://github.com/legacy-chat-platform/legacy-chat-platform`. legacy chat platform uses polymorphic channels (e.g., `Channel::WebWidget`, `Channel::Email`, `Channel::FacebookPage`, `Channel::Whatsapp`, `Channel::Sms`) linked to an `Inbox`. Messages belong to `Conversations`, and `Conversations` tie an `Inbox` to a `Contact`.
  - **Current OHC State**: We have `src/server/services/chat/service.rs` providing basic CRUD, but we lack the specific channel data models (especially the web widget config), the websocket real-time syncing layer, and the AI agent department integration. The database schema in `1009_native_omnichannel_chat.sql` defines `chat_channels.channel_type` and a flexible `config JSONB` column which allows polymorphic channel properties similar to legacy chat platform, but we need concrete channel adapters.
  - **Competitor Insights (Stripe, Wix, Shopify Sidekick)**: Real-time, localized, low-latency messaging with offline capabilities is essential. The web widget needs to be lightweight, easy to embed (like an iframe or custom element), and natively interact with our WebSocket endpoints.

  ## Design Doc
  ### Architecture Blueprint
  - **Data Model Extensions**:
    - Extend the `chat_channels` conceptual usage. For the web widget, the `config` JSONB will store `website_url`, `widget_color`, `welcome_title`, `welcome_tagline`.
    - Introduce a WebSocket Event Gateway (`src/server/api/chat/ws.rs`) to handle real-time bi-directional streaming for `chat_messages`.
    - Create a Channel Adapter Pattern (`src/server/services/chat/channels/`) to parse incoming webhooks (e.g., from Twilio/Meta) and route them to `ChatService::send_message`.
  - **Web Widget (Frontend)**:
    - Create a lightweight embeddable React/Preact or Vanilla JS widget in `src/ui/widget/`.
    - The widget uses the `/api/v1/chat/widget/config?inbox_id={id}` to load settings.
    - Connects via WebSocket to `ws://api.onehumancorp.com/v1/chat/ws`.
  - **AI Agent Integration**:
    - AI agents (Customer Service Department) hook into the `ChatService::send_message` event stream via the message bus (`src/server/msgbus.rs`). When a message arrives in a conversation assigned to the bot, it triggers an AI response.

  ### ER Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      CHAT_INBOXES ||--o{ CHAT_CHANNELS : has
      CHAT_INBOXES ||--o{ CHAT_CONVERSATIONS : hosts
      CHAT_CONTACTS ||--o{ CHAT_CONVERSATIONS : participates_in
      CHAT_CONVERSATIONS ||--o{ CHAT_MESSAGES : contains

      CHAT_INBOXES {
          uuid id PK
          uuid tenant_id
          string name
      }
      CHAT_CHANNELS {
          uuid id PK
          uuid tenant_id
          uuid inbox_id FK
          string channel_type
          jsonb config
      }
      CHAT_CONTACTS {
          uuid id PK
          uuid tenant_id
          string name
          string email
          string phone
      }
      CHAT_CONVERSATIONS {
          uuid id PK
          uuid tenant_id
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id
          string status
      }
      CHAT_MESSAGES {
          uuid id PK
          uuid tenant_id
          uuid conversation_id FK
          string sender_type
          uuid sender_id
          string content
      }
  ```

  ### Sequence Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      actor Customer
      participant Widget as Web Widget
      participant WSServer as WebSocket Gateway
      participant ChatService
      participant MessageBus
      participant AIAgent as Customer Service Agent

      Customer->>Widget: Types message & sends
      Widget->>WSServer: WS publish message
      WSServer->>ChatService: send_message(conversation_id, content)
      ChatService->>Database: INSERT chat_messages
      ChatService->>MessageBus: Publish event (message_created)
      WSServer->>MessageBus: Subscribe to conversation updates
      MessageBus-->>WSServer: Emit update to connected clients (Owner/Other devices)
      MessageBus-->>AIAgent: Trigger AI processing if bot assigned
      AIAgent->>ChatService: Generate and send AI reply
      ChatService->>Database: INSERT chat_messages (AI reply)
      ChatService->>MessageBus: Publish event (message_created)
      MessageBus-->>WSServer: Emit update
      WSServer-->>Widget: WS receive message (AI reply)
      Widget-->>Customer: Displays AI response
  ```

  ### Mobile UX Flow (375px)
  - The Owner sees an "Inbox" tab. New conversations bubble to the top.
  - Selecting a conversation opens a standard chat view (macOS Translucent Glass style).
  - Tapping "Reply" shows options for AI Draft, Canned Response, or Manual Entry.
  - The Web Widget on the customer's phone appears as a floating fab, expanding into a full-screen drawer for easy tapping (44x44px minimum touch targets).

  ### Technical Decisions
  - **Single `chat_channels` table with JSONB**: Avoids schema bloat (unlike legacy chat platform's 14 separate channel tables) while maintaining flexibility.
  - **WebSocket over REST polling**: Required for real-time customer support.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your task is to build out the next layer of the native Rust Omnichannel Chat system to achieve feature parity with legacy chat platform's Web Widget channel.
  1. **API Layer**: Create REST endpoints for the Web Widget to fetch its config, initialize a contact session, and fetch conversation history.
  2. **WebSocket Layer**: Implement a WebSocket handler in Rust (using `axum` or the existing web framework) that allows the Web Widget to connect, subscribe to a specific conversation, and send/receive real-time messages.
  3. **Service Layer**: Enhance `ChatService` to emit events to a pub/sub mechanism (e.g., Redis or in-memory broadcast channel) when `send_message` is called, so WebSocket subscribers get updates instantly.
  4. **Widget UI (Optional / Scope Permitting)**: Lay the groundwork for the embeddable Web Widget UI.
  Ensure all new endpoints and websocket handlers are fully unit-tested, and write an E2E Playwright test simulating a customer chatting via the widget and an owner receiving it. Ensure strict tenant isolation.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
