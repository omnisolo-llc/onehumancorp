issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  ## Problem Statement
  The legacy Chatwoot external service integration is being 100% retired in favor of a native, high-performance omnichannel customer support and chat engine written in Rust. OHC operators like Maya, Carlos, and Fatima need a unified inbox that brings together WhatsApp, Instagram DMs, SMS, and web widgets into a single feed that AI agents can seamlessly triage and respond to. External dependencies introduce latency, complicate multi-tenant data isolation, and limit our ability to deeply integrate our AI department routing. We need an internal architecture matching Chatwoot's core capabilities (Inboxes, Conversations, Contacts, Channel Adapters) but built natively on our platform.

  ## Research Report
  An audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) reveals its core architecture is built around several key entities:
  - **Inboxes**: The entry point for messages, linked to a specific channel (e.g., `whatsapp`, `web_widget`, `email`, `sms`).
  - **Conversations**: Groupings of messages tied to a specific `contact` and `inbox`, supporting states like `open`, `resolved`, or `snoozed`, and assignments to agents (human or bot).
  - **Contacts**: The unified identity of the customer across different channels.
  - **Channel Adapters**: Handlers for distinct platforms (e.g., Meta webhooks for WhatsApp/Instagram, WebSocket handlers for web widgets).

  OHC's native implementation must replicate this structure using Rust, storing data in PostgreSQL with strict Row-Level Security (RLS) based on `tenant_id`. The new system will eliminate third-party API latency and allow seamless background AI processing (Operations, Sales, CS agents) via our existing job queues.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      CHANNEL_ADAPTER ||--o{ INBOX : bridges

      INBOX {
          uuid id
          uuid tenant_id
          string name
          string channel_type
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          text content
          string sender_type
      }
  ```

  ### Mobile UX Flow (375px)
  1. **Unified Inbox View**: The operator taps the "Messages" tab on their mobile dashboard. A clean, translucent glass-styled list displays recent conversations across all channels (WhatsApp, Web, IG) unified into one feed.
  2. **Conversation Thread**: Tapping a conversation opens a thread view. Touch targets are large (44x44px). The UI clearly shows if an AI agent is currently handling the conversation or drafting a response.
  3. **Agent Handoff**: If an operator wants to take over, they tap a prominent "Take Over" button, disabling the AI auto-responder for that thread.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Monitors new incoming `Conversation` events and routes them.
  - **Customer Assistant Agent**: Subscribes to `Message` creation events, reads the conversation history (context), and pushes drafted replies to the thread or sends them automatically based on tenant settings.
  - **Memory/Context**: Agents access the `Contact` records to remember past interactions and preferences (e.g., "Customer prefers vegan cakes").

  ### Key Design Decisions
  - **Native Rust**: Implementation in `src/server/integrations/chat/` using our existing Rust microservice framework for maximum performance and memory safety.
  - **Strict Multi-Tenancy**: Every database table (`inboxes`, `conversations`, `messages`, `contacts`) must include `tenant_id` and enforce RLS.
  - **WebSocket First**: For web widgets, real-time message delivery is prioritized using Rust WebSocket servers. Webhooks handle third-party channels.

  ## Implementation Prompt
  **To the Implementation Agent:**
  Your objective is to implement the foundational database schemas, Rust domain models, and core API endpoints for the new Native Omnichannel Chat System, replacing Chatwoot.

  1. **Data Models**: Create the SQL migrations and Rust structs for `Inbox`, `Conversation`, `Message`, and `Contact`, ensuring `tenant_id` RLS is strictly enforced.
  2. **Channel Adapters**: Implement the base trait/interface for Channel Adapters, and create the initial `WebWidget` and `WhatsApp` implementations.
  3. **API Endpoints**: Build the REST API endpoints to list inboxes, fetch conversation histories, and send messages.
  4. **AI Triage Hook**: Implement a hook or event publisher that triggers the OHC AI Agent queue when a new message arrives.
  5. **Verification**: You MUST write 100% coverage unit tests for the Rust logic and Playwright E2E tests for a Critical User Journey (CUJ) where a user sends a message via the Web Widget and it appears in the operator's Unified Inbox. Follow all Core Directives regarding real UI testing and no mock data.

  **Acceptance Criteria**:
  - The system compiles and passes all `bazel test //...` checks.
  - E2E tests confirm a message can be sent and received natively without external dependencies.
  - All new database tables have RLS enabled.

  ## Priority: P0
  ## Estimated Scope: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
