issue_title: "Architecture Design: Native Rust Omnichannel Chat System & Chatwoot Retirement"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) currently lacks a native, high-performance, multi-tenant omnichannel chat system, and historically relied on or considered Chatwoot as an external service/dependency. Chatwoot integration is now 100% RETIRED. The core gap is that owner/operator personas (like Maya, Carlos, Priya) need a unified inbox to triage messages across Instagram DMs, WhatsApp, SMS, Web Chat, and Email. Without a native solution, OHC cannot provide the tight AI-agent coordination (Work Triage, Customer Assistant) or row-level tenant isolation required by our platform architecture.

  ## Research Report
  - **Competitor Analysis:**
    - Shopify Inbox: Integrates directly with store data but is highly ecommerce focused.
    - HubSpot: Comprehensive but too complex for small-business operators (jargon-heavy).
    - Chatwoot: Excellent open-source omnichannel model (channels, inboxes, conversations, messages, contacts) but relies on Ruby on Rails, PostgreSQL without our specific RLS multi-tenancy model, and Redis.
  - **Source Code Audit (Chatwoot):**
    - Explored `chatwoot/app/models`. Key entities: `Account`, `User`, `Inbox`, `Channel`, `Contact`, `Conversation`, `Message`, `AgentBot`.
    - Chatwoot uses polymorphic associations for `Channel` (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Email`).
    - WebSocket real-time messaging is handled via ActionCable.
  - **OHC Architecture Fit:**
    - OHC uses Go+Bazel backend but the mandate specifically requires a "Native Rust Implementation" for the omnichannel chat engine inside `onehumancorp/mono`.
    - We must port the core Chatwoot data models into our PostgreSQL schema with strict `tenant_id` RLS.
    - Real-time updates should use WebSockets/gRPC streams powered by Rust, coordinating through Redis (Redlock/PubSub).

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CHANNEL_ADAPTER : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE }o--|| AI_AGENT_DRAFT : "can have"

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL_ADAPTER {
          uuid id
          uuid inbox_id
          string type "WhatsApp, Web, IG"
          jsonb credentials
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

  ### Mobile UX Flow (375px First)
  1. **Work Command Center (Home):** User opens the app. A unified "Urgent Messages" card shows unread DMs from Instagram/WhatsApp.
  2. **Unified Inbox List:** Tapping the card opens a full-screen, clean list of conversations across all channels. Badges indicate the source (e.g., green for WhatsApp).
  3. **Conversation View:** Standard chat interface. Native mobile keyboard.
  4. **AI Assistant Integration:** Above the keyboard, an "AI Draft" translucent glass button appears. Tapping it shows a suggested reply based on customer history and business context. The owner can tap "Approve & Send" or edit.

  ### AI Agent Integration Points
  - **Work Triage:** Analyzes incoming `MESSAGE` webhooks/events to categorize urgency and update the owner's prioritized feed.
  - **Customer & Relationship Assistant:** Listens to new `CONVERSATION` and `MESSAGE` creation. Automatically drafts responses and stores them in `AI_AGENT_DRAFT` pending owner approval, avoiding immediate send unless auto-reply is enabled.

  ### Key Design Decisions
  - **Rust Microservice:** Use Rust (e.g., Tokio, Axum/Tonic) for the chat engine to ensure high concurrency and low latency for WebSockets.
  - **Data Isolation:** All database tables (`inboxes`, `conversations`, `messages`) MUST include `tenant_id` and have Row Level Security (RLS) enabled.
  - **Polymorphic Channels:** Implement a trait-based channel adapter pattern in Rust to easily add new channels (WhatsApp, IG, SMS) without changing core conversation logic.

  ## Implementation Prompt
  Implement the Native Rust Omnichannel Chat System for OneHumanCorp.
  - **Target Outcome:** A business owner can connect a channel (e.g., Web Widget or mock WhatsApp), receive messages in a unified inbox, view them on a 375px mobile-responsive UI, and see AI-drafted suggested replies.
  - **CUJ:**
    1. Owner logs in and navigates to "Inbox".
    2. Owner connects a "Web Chat" channel.
    3. A customer sends a message via the web widget.
    4. The owner sees the message appear in real-time on their mobile dashboard.
    5. The owner approves an AI-drafted reply, which is sent back to the customer.
  - **Acceptance Criteria:**
    - Rust service compiles and runs via Bazel.
    - PostgreSQL tables include `tenant_id` and enforce RLS.
    - Real-time message delivery works via WebSockets.
    - AI Draft generation triggers on new incoming messages.
    - UI is built using OHC Premium Token library with translucent materials, fully usable at 375px width.
    - 100% Unit test coverage and Playwright E2E tests validating the complete CUJ.
    - NO Chatwoot external dependencies.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
