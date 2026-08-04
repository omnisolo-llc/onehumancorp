issue_title: "Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  One Human Corp (OHC) has been relying on Chatwoot as an external dependency for omnichannel messaging. However, per the architectural mandate, Chatwoot as a 3rd-party service is being 100% retired. OHC requires a high-performance, multi-tenant omnichannel customer support and chat engine built natively in Rust inside the `onehumancorp/mono` repository. Non-technical owner/operators like Maya, Carlos, and Fatima need a unified inbox that brings together Instagram DMs, WhatsApp, Email, and SMS seamlessly on a 375px mobile screen. This system must be backed by real-time WebSockets and deeply integrate with AI agents for drafting responses, capturing context, and classifying intent.

  ## Research Report
  - **Chatwoot Architecture Audit**: Reviewed Chatwoot's source code (`https://github.com/chatwoot/chatwoot`) to understand its omnichannel data models. Core concepts include Inboxes, Channels (Web Widget, API, WhatsApp, SMS, Email, etc.), Contacts, Conversations, Messages, and Agents. Chatwoot relies on ActionCable for real-time WebSocket messaging and polymorphic associations for channels.
  - **OHC Native Alignment**: Instead of Ruby/ActionCable, OHC will utilize its Rust async stack (Axum) combined with PostgreSQL (via SQLX) for data persistence and Valkey (Redis) for high-throughput pub/sub WebSocket messaging across stateless API pods.
  - **Competitive Differentiation**: Unlike standalone support tools, our native chat system will tightly integrate with OHC's autonomous workflows. For example, the chat interface will seamlessly connect with the Booking Engine, Ledger, and AutoDream pipeline.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      INBOX ||--o{ CHANNEL_ADAPTER : has
      INBOX ||--o{ CONVERSATION : manages
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o| ATTACHMENT : includes

      INBOX {
          uuid id PK
          uuid tenant_id FK
          string name
          boolean is_active
      }
      CHANNEL_ADAPTER {
          uuid id PK
          uuid inbox_id FK
          string channel_type "whatsapp, ig, email, web"
          jsonb config
      }
      CONTACT {
          uuid id PK
          uuid tenant_id FK
          string name
          string identifier
      }
      CONVERSATION {
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status "open, resolved, snoozed"
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          string sender_type "contact, agent, ai"
          text content
          datetime created_at
      }
  ```

  ### UI Wireframes & 375px Mobile UX Flow
  - **Unified Feed (List View)**: A 375px-optimized vertical scrolling list of active conversations. Uses macOS-style Translucent Glass headers. Badges indicate unread messages or AI-drafted replies awaiting owner approval.
  - **Conversation Thread (Detail View)**: Apple iMessage-style chat bubbles. A sticky bottom input bar natively integrates with the mobile keyboard and includes a quick-action "+" button to invoke AI tools (e.g., generate a quote, send a booking link).
  - **Contact Context Sheet**: A swipeable bottom sheet surfacing customer lifetime value, previous orders, and AI-summarized sentiment.

  ### Mobile UX Flow
  1. **Triage**: The owner opens the OHC app and lands on the Unified Feed.
  2. **Review**: Taps a conversation tagged with an "AI Draft" badge.
  3. **Approve**: The thread opens to display the AI's suggested reply in a pending state. The owner reviews the draft and taps "Approve" (or "Edit").
  4. **Execute**: The message is sent asynchronously via the `CHANNEL_ADAPTER` (e.g., WhatsApp API) while the UI instantly reflects the updated state.

  ### AI Agent Integration Points
  - **Triage Agent**: Subscribes to `conversation.created` and `message.created` pub/sub events. Analyzes intent (e.g., Support, Sales, Booking) and automatically routes or prioritizes the thread.
  - **Drafting Agent**: Auto-generates proposed replies based on the tenant's embedded Knowledge Base and conversation history. These are persisted as `MESSAGE` rows with `sender_type = "ai"` and `status = "draft"`.

  ### Key Design Decisions
  - **Polymorphic Channel Adapters**: The `CHANNEL_ADAPTER` entity uses a `jsonb` column for storing integration-specific configurations (tokens, API keys). This schema allows the platform to dynamically add new external channels (Meta, Twilio) without subsequent database migrations.
  - **Stateless WebSocket Cluster**: WebSocket clients connect to a stateless Axum API pod. The pod subscribes to tenant-specific Valkey/Redis topics (e.g. `ohc:tenant:{id}:inbox:{id}`). When an external webhook or AI agent inserts a message, an event is published to Valkey, enabling the correct pod to instantly push the event to the connected mobile client.

  ## Implementation Prompt
  Implement the Native Rust Omnichannel Chat backend and database schemas to successfully retire Chatwoot.
  1. Create SQLX migrations for the `inboxes`, `channel_adapters`, `contacts`, `conversations`, and `messages` tables. Ensure row-level security (`tenant_id`) is strictly enforced across all entities.
  2. Develop the Rust Axum REST API endpoints (under `src/server/services/chat/`) for listing inboxes, retrieving conversation histories, and sending messages.
  3. Implement the WebSocket route `/api/v1/chat/ws`. The handler must authenticate the user via OIDC/JWT, subscribe to the appropriate Valkey/Redis pub/sub channels, and stream real-time `MessageCreated` JSON events to the client.
  4. Write comprehensive E2E Playwright tests simulating the Critical User Journey (CUJ): login, view the inbox list, select a conversation, and send a message. All tests must pass locally via `bazel test //src/e2e:playwright` using local test adapters (zero external network calls).
  5. The implementation must include appropriate Unit Tests for the chat domain logic with 100% coverage expectations.

  Focus exclusively on the backend API, WebSocket infrastructure, and automated E2E tests. The deep UI/UX mobile implementation will be handled in a fast-follow task.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
