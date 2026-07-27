issue_title: "Retire Chatwoot: Build Native Rust Omnichannel Chat System"
issue_description: |
  ## Title: Retire Chatwoot: Build Native Rust Omnichannel Chat System

  ## Problem Statement
  Currently, managing customer communications across different channels (Instagram DMs, WhatsApp, Website Chat, Emails) can feel disjointed or require relying on external third-party systems like Chatwoot, which breaks our unified "One Human Corp" experience. For an owner like Maya the Baker or Carlos the Handyman, jumping between different apps or waiting for third-party syncs means lost leads, missed follow-ups, and an incomplete view of customer history. They need a single, instant, and reliable unified inbox directly within their OHC mobile and desktop app. The system must feel native, be instantly responsive, and automatically coordinate with AI agents to draft replies and manage operational tasks.

  ## Research Report
  - **Chatwoot Source Code Audit**: An audit of Chatwoot's repository (`https://github.com/chatwoot/chatwoot`) reveals a robust MVC architecture in Ruby on Rails. The core domain models that enable its omnichannel capabilities include:
    - `Inbox`: Represents a specific channel connection (e.g., a specific Facebook page, a web widget, or a WhatsApp number).
    - `Conversation`: A unified thread of messages between a contact and the business within a specific inbox.
    - `Message`: The individual chat bubbles or emails within a conversation.
    - `Contact`: The unified customer profile that spans multiple conversations and inboxes.
    - `Channel::*`: Adapters for specific platforms (Facebook, Twitter, Web Widget, API, Email).
  - **Competitive Analysis**:
    - *Shopify Inbox*: Excellent native integration with store products and orders, allowing merchants to send product links and discount codes directly in chat. But limited to store-centric conversations.
    - *Wix Inbox*: Good unified view of forms and chats, but can be sluggish on mobile and lacks deep AI operational coordination.
    - *OHC Differentiation*: By bringing the chat system natively into Rust, we can integrate it directly with our AI Job Queue, Distributed Locks, and Postgres multi-tenancy. Agents won't just draft replies; they can instantly transition a conversation into an order, a booking, or a task without crossing network boundaries to third-party tools.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--o{ CHANNEL_ADAPTER : configured_via

      TENANT {
          uuid id
          string name
      }
      INBOX {
          uuid id
          string name
          uuid channel_id
      }
      CONTACT {
          uuid id
          string name
          string email
          string phone
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
          string content
          string sender_type
      }
  ```

  ### UI Wireframes & Screen Flow (375px First)
  1. **Unified Inbox List (Home)**: Clean, unread-first list of conversations. Avatars show the channel icon (WhatsApp, Web, IG) as a small badge. Translucent glass header.
  2. **Conversation View**: Familiar chat bubble layout. 44x44px touch targets for attachments and AI agent quick-actions (e.g., "Draft Reply", "Create Booking").
  3. **Contact Sidebar (Drawer on mobile)**: Swiping left from the right edge reveals the customer's history, past orders, and agent-generated summary notes.

  ### Mobile UX Flow
  - Maya receives an Instagram DM.
  - Push notification appears on her 375px screen.
  - Tapping opens the specific `Conversation` instantly.
  - The AI Assistant has already read the message and proposed a draft reply in a subdued glass-morphic bubble at the bottom.
  - Maya taps "Send Draft" or edits it using the native mobile keyboard.

  ### AI Agent Integration Points
  - **Work Triage Agent**: Subscribes to new `Message` events (via Postgres or internal pub/sub) to evaluate urgency and intent.
  - **Customer & Relationship Agent**: Automatically generates a draft `Message` for unread inquiries.
  - **Operations Agent**: Can attach structured UI cards (e.g., "Booking Quote") directly into the message feed.

  ### Key Design Decisions
  - **Native Rust Implementation**: Complete retirement of Chatwoot as an external service. All endpoints, WebSocket real-time updates, and data models (`Inbox`, `Conversation`, `Message`, `Contact`) will be built natively in `onehumancorp/mono` (specifically in `src/server/ohc/domain/`).
  - **Strict Multi-Tenancy**: Every table must include `tenant_id` and utilize PostgreSQL Row Level Security (RLS) to guarantee complete data isolation between owners.
  - **Performance**: Natively integrated WebSockets and Rust Axum endpoints will ensure sub-100ms latency for message delivery, crucial for live chat widget parity.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Task:** Build the core backend data models and service layer for the native Rust Omnichannel Chat System in OHC, replacing Chatwoot.
  **CUJ:** A business owner needs to view a unified list of incoming messages from multiple sources (simulated by different `Inbox` records) and send a reply to a specific customer `Contact` in a `Conversation`.
  **Acceptance Criteria:**
  1. Define Rust structs, SQLx database migrations, and repository functions for `Inbox`, `Conversation`, `Message`, and `Contact` within `src/server/ohc/domain/`.
  2. Implement strict multi-tenant isolation using `tenant_id`.
  3. Expose Axum REST endpoints for fetching conversations and creating messages.
  4. Write Playwright E2E tests verifying a user can open the inbox, view a seeded conversation, and send a message. ZERO mock data in the UI; all data must flow through the new endpoints.
  5. Ensure 100% Rust unit test coverage for the new domain modules.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
