issue_title: "Design & Implement Native Rust Omnichannel Inbox (Chatwoot Replacement)"
issue_description: |
  ### Title
  Design & Implement Native Rust Omnichannel Inbox (Chatwoot Replacement)

  ### Problem Statement
  Owners like Maya (custom cakes) and Carlos (handyman) receive messages from Instagram DMs, WhatsApp, SMS, and website chat. Managing these across separate apps causes missed inquiries, slow responses, and lost revenue. Previously, we integrated with Chatwoot, but as an external service, it creates data silos, increases latency, and makes it impossible to deeply embed our autonomous AI agents (The Ambassador) directly into the message routing and drafting flow. Owners need a single, unified command center where their AI assistant can see all messages, draft contextual replies, and even handle routine inquiries automatically, natively built into the OHC platform.

  ### Research Report
  **Findings & Competitive Analysis:**
  - **Chatwoot Source Audit:** We audited Chatwoot's architecture (`app/models`). It uses a robust structure: `Inbox` (representing a channel integration), `Conversation` (a thread with a `Contact`), and `Message` (individual texts, attachments, or system activities). Channels are polymorphic (`Channel::Whatsapp`, `Channel::WebWidget`, etc.).
  - **Shopify Inbox:** Aggregates chat, but lacks deep AI drafting based on full multi-channel history.
  - **Wix Inbox:** Good aggregation, but AI is limited to tone adjustment.
  - **OHC Native Advantage:** By building this natively in Rust, we can link `Contact` records directly to our OHC multi-tenant identity graph. This allows our AI agents to access past orders, bookings, and payments instantaneously when drafting a response.

  ### Design Doc
  **Architecture Diagram**
  ```mermaid
  graph TD
      A[WhatsApp / Instagram / Web] -->|Webhook/WS| B(Channel Gateway - Rust)
      B --> C{Message Router}
      C --> D[(OHC PostgreSQL DB)]
      C --> E[Event Mesh - NATS]
      E --> F[The Ambassador Agent]
      F -->|Queries Context| G[Unified Identity Graph]
      F -->|Drafts Reply| D
      E --> H[WebSocket Server]
      H -->|Realtime Update| I[OHC Mobile/Web App]
  ```

  **UI Wireframes & Mobile UX Flow (375px First)**
  - **Unified Inbox List (Mobile):** A clean, unread-first feed of conversations. Each row shows the contact's avatar, channel icon (e.g., small WhatsApp logo), snippet of the last message, and an AI badge if a drafted reply is waiting.
  - **Conversation View:** Standard chat bubble layout. Top bar shows contact name and channel.
  - **AI Drafting Flow:** If a customer asks a question, the AI (The Ambassador) pre-drafts a response shown in a translucent glass card at the bottom above the native keyboard, with a "Send Draft" or "Edit" button.
  - **Visual Design:** Apple/Ubiquiti-style hierarchy. Use translucent materials for floating AI suggestions.

  **AI Agent Integration Points**
  - **The Ambassador (Customer Success Agent):** Listens to `message.created` events on NATS. If the message is from a customer, it retrieves the `Conversation`, `Contact`, and their recent `Orders`/`Bookings`. It then generates a draft reply and inserts a `Message` with `status = draft` and `private = true`.

  **Key Design Decisions**
  - **Retire Chatwoot:** 100% replacement. The core entities (`Inbox`, `Channel`, `Contact`, `Conversation`, `Message`) must be built in OHC's Rust backend using SeaORM, enforcing row-level security per tenant.
  - **WebSockets for Real-time:** The frontend must receive immediate updates for new messages and AI drafts without polling.
  - **Draft-First AI:** Instead of auto-replying, the AI drafts messages for the owner to approve, ensuring quality control while saving time.

  ### Implementation Prompt
  **User-Facing Outcome:**
  As an owner, when a customer sends a WhatsApp message, I receive a push notification. I open the OHC app, see the message in my unified inbox, and see a perfect AI-drafted reply based on the customer's past orders. I tap "Approve" and the message is sent.

  **CUJ & Acceptance Criteria:**
  1. An API endpoint exists to receive a simulated incoming message from an external channel.
  2. The backend creates or updates a `Contact`, creates a `Conversation` if none exists, and saves the `Message`.
  3. The system broadcasts the new message via WebSocket to the connected owner UI.
  4. The Ambassador Agent detects the new message, drafts a reply based on mock context, and saves it as a draft message.
  5. The draft message is pushed via WebSocket and displayed in the UI as an actionable card.
  6. Provide Playwright E2E tests: A user logs in, opens the inbox, receives a simulated message, sees the AI draft, and taps "Approve" to send it.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
