issue_title: "Implement Native Rust Omnichannel Chat System (Chatwoot Replacement)"
issue_description: |
  # Title: Implement Native Rust Omnichannel Chat System

  ## Problem Statement
  Currently, owners and operators like Maya the baker and Carlos the handyman struggle to keep track of customer conversations scattered across Instagram DMs, WhatsApp, SMS, and website chat. While we previously relied on an external system (Chatwoot), this introduced complexity, latency, and data silos that made it harder to build seamless AI agent integrations and real-time mobile notifications. The person responsible for the business needs a single, unified "assistant-first" inbox on their phone (375px display) that magically routes, categorizes, and even drafts replies to all these messages without them ever needing to configure complex integrations or switch between multiple apps.

  ## Research Report
  Our audit of the retired external Chatwoot dependency reveals a robust set of core models required for an omnichannel inbox, primarily: `Account` (tenant), `Inbox` (channel source), `Conversation` (the thread), and `Message` (the individual interaction).
  - **Shopify Inbox**: Extremely streamlined for commerce, directly tying conversations to cart value and past orders. However, it lacks deep multi-platform flexibility for non-retail service businesses.
  - **Wix Inbox / GoDaddy Conversations**: Good mobile-first approaches but can feel clunky when escalating to AI agent drafts or handling advanced routing (e.g., SLA policies, automation rules).
  - **The Gap**: We need an ultra-fast, native Rust implementation embedded directly inside the `ohc-mono` repository that provides complete omnichannel capabilities (WhatsApp, Instagram, Web, SMS) but with 100% built-in AI agent awareness to draft responses before the owner even opens the message.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      ACCOUNT ||--o{ INBOX : has
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      ACCOUNT ||--o{ CONTACT : has
      CONTACT ||--o{ CONVERSATION : initiates
      INBOX }|--|| CHANNEL_ADAPTER : uses

      ACCOUNT {
          uuid id
          string name
          jsonb settings
      }
      INBOX {
          uuid id
          string name
          string channel_type
          uuid account_id
      }
      CONVERSATION {
          uuid id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          text content
          string message_type
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Unified Feed (Home)**: The owner opens the app. The primary view is a cleanly spaced, UniFi-style list of prioritized items. Unread customer messages (e.g., "Do you do vegan cakes?") appear at the top as translucent glass cards.
  - **Conversation View**: Tapping a card slides into the conversation view. The design mimics iMessage but with a distinct "Agent Draft" bubble at the bottom.
  - **Agent Interaction**: Instead of typing, the owner sees a pre-drafted reply (e.g., "Yes, we do vegan cakes! Would you like a quote?"). They can tap "Send" or tap the text field to edit using native mobile keyboards.
  - **Action Menu**: A prominent '+' icon allows turning a message directly into a quote, booking, or payment link without leaving the chat.

  ### AI Agent Integration Points
  - **Triage Agent (On Message Received)**: A background worker using the AI job queue evaluates incoming messages, tags them (e.g., "Lead", "Support"), and assigns a priority.
  - **Customer Assistant Agent (Drafting)**: Before the owner even sees the message, this agent reads the conversation history and business context to draft a suggested reply, storing it as an internal `message_type: draft`.

  ### Key Design Decisions
  - **Native Rust & PostgreSQL Row-Level Security**: Moving to a native Rust implementation allows us to enforce strict `tenant_id` based row-level security natively within our existing database architecture, avoiding synchronization issues with external services.
  - **Agent-First Data Model**: Unlike Chatwoot, our `Message` model will natively support `message_type: draft` and `agent_intent` fields, making AI collaboration a core part of the database schema rather than a bolted-on feature.
  - **Mobile-First Realtime**: We will use native WebSockets (integrated with our Rust backend) to ensure the 375px mobile client receives instant updates without battery-draining polling.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your goal is to build the native Rust backend and the Flutter mobile-first UI for the new Omnichannel Chat System, completely replacing the need for external Chatwoot.
  - **The User Journey**: A customer sends a message. The system captures it, an AI agent drafts a reply, and the owner sees it in their unified mobile feed. The owner reviews the draft and taps "Send".
  - **Acceptance Criteria**:
    1. The core domain models (Account, Inbox, Conversation, Message, Contact) are implemented natively in Rust within the `ohc-mono` backend with strict multi-tenant row-level security.
    2. A WebSocket or real-time event system pushes new messages to the frontend.
    3. The mobile UI (375px optimized) displays the unified inbox and conversation views using the translucent glass design system.
    4. AI agents can successfully attach drafted replies to conversations.
    5. At least five Playwright E2E tests verify the complete flow (receiving a message, viewing the draft, and sending) without any mocked UI data.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
