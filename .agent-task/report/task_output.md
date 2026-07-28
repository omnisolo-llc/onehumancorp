issue_title: "Native Rust Omnichannel Chat Engine (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  Chatwoot as an external service/dependency is 100% RETIRED. OHC requires a high-performance, multi-tenant omnichannel customer support & chat engine natively built in Rust inside `onehumancorp/mono`. We need this native chat system to seamlessly aggregate customer conversations across Instagram DMs, WhatsApp, SMS, and Email into a single Inbox that the owner/operator (like Maya or Carlos) can manage effortlessly from a 375px mobile screen. It must handle real-time messaging, agent routing, auto-assignment, and AI integration for auto-drafting replies without relying on external vendors.

  ## Research Report
  - **Source Code Audit (Chatwoot)**: Analyzed Chatwoot's Postgres schema (`schema.rb`), which centralizes around `accounts`, `inboxes`, `channels`, `conversations`, `messages`, and `contacts`. Multi-tenancy is achieved via `account_id` on all core models. Real-time updates use ActionCable WebSockets.
  - **OHC Native Parity**: Our OHC models in `src/server/services/chat/models.rs` outline the basic foundation (`ChatInbox`, `ChatChannel`, `ChatContact`, `ChatConversation`, `ChatMessage`). However, they lack robust real-time channel abstractions, WebSocket integration, AI-agent auto-reply states, and SLA policies.
  - **Competitor Insights**: Systems like Front and Zendesk use distinct "Channel Adapters" mapping external platform webhooks to internal unified message models.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      Tenant ||--o{ ChatInbox : owns
      ChatInbox ||--o{ ChatChannel : configured_by
      Tenant ||--o{ ChatContact : has
      ChatContact ||--o{ ChatConversation : participates_in
      ChatInbox ||--o{ ChatConversation : contains
      ChatConversation ||--o{ ChatMessage : holds
      ChatMessage ||--o{ AgentDraft : auto_generates

      ChatChannel {
          UUID id
          UUID tenant_id
          String channel_type "email, sms, instagram"
      }
      ChatConversation {
          UUID id
          UUID inbox_id
          UUID contact_id
          String status "open, resolved, snoozed"
      }
      ChatMessage {
          UUID id
          UUID conversation_id
          String sender_type "human, agent, customer"
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Triage Feed Screen**:
    - Unified list of conversations sorted by `last_activity_at`.
    - Unread indicator and AI-draft icon (sparkles) if an AI response is ready.
    - Large 44x44px touch targets for selecting a conversation.
  - **Conversation Screen**:
    - Sticky header with Customer Name and source icon (e.g., IG logo).
    - Scrollable message history.
    - Floating action button (FAB) for "Approve AI Draft" if the Customer Service Agent generated a reply.
    - Native mobile keyboard input area.

  ### AI Agent Integration Points
  - **Customer Service Agent**: Hooks into the `ChatMessage` creation event (via internal gRPC or pub/sub). When a customer sends a message, the Agent analyzes the `ChatConversation` history and creates an `AgentDraft` for the owner to approve with one tap.
  - **Operations Agent**: Parses messages to detect booking requests, generating an internal `WorkItem` invisible to the customer but actionable by the owner.

  ### Key Design Decisions
  - **Zero Trust & Multi-Tenancy**: Every database table will strictly enforce `tenant_id` Row Level Security (RLS) via `app.current_tenant_id` to guarantee cross-tenant isolation.
  - **WebSocket Event Bus**: A native Rust WebSocket service will stream `conversation.updated` and `message.created` events to the mobile/web client for real-time interaction.
  - **Channel Adapters Strategy**: Abstracted traits in Rust for `ChannelAdapter` will allow uniform processing of incoming webhooks from Twilio, Meta, and Email providers.

  ## Implementation Prompt
  **User-Facing Outcome:** Maya (the baker) receives an Instagram DM about a custom cake. She opens the OHC mobile app, sees the message in her Triage Feed, and taps it. The AI has already drafted a polite reply asking about delivery dates. She taps "Send", and the message is instantly routed back to Instagram.

  **Critical User Journey (CUJ):**
  1. Owner logs into OHC on mobile (375px).
  2. Owner navigates to the unified Inbox.
  3. Owner views an open conversation with an AI-generated draft.
  4. Owner modifies or approves the draft, pressing "Send".
  5. The UI updates optimistically, and the backend processes the webhook dispatch.

  **Acceptance Criteria:**
  - Database schema includes RLS-enforced `inboxes`, `channels`, `conversations`, and `messages`.
  - Rust API provides endpoints for fetching conversations and sending messages.
  - WebSocket connection broadcasts new messages to connected clients.
  - AI Agent automatically creates a draft reply upon receiving a new customer message.
  - 100% unit test coverage for the service layer and at least 5 Playwright E2E tests verifying the mobile CUJ.
  - ZERO external dependency on Chatwoot; all logic is native.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
