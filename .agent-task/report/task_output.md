issue_title: "Native Rust Omnichannel Customer Support & Chat Engine (Track 2 Deep Dive)"
issue_description: |
  # Native Rust Omnichannel Customer Support & Chat Engine (Track 2 Deep Dive)

  ## Problem Statement (Non-Technical Persona: Priya, Boutique Operator)
  Priya currently receives customer inquiries across Instagram DMs (asking about dress sizes), WhatsApp (customers checking order status), and email (vendor communications). Right now, she has to switch between three different apps on her phone while running her physical store. If she misses a message, she loses a sale or a loyal customer. She needs ONE simple "Inbox" in her OHC app where all messages arrive, where her AI assistant can draft replies based on her inventory, and where she can just tap "Approve and Send" to reply to the customer on whatever platform they used, without thinking about it.

  ## Research Report
  - **Market Context**: Platforms like Shopify Inbox, Zendesk, and Intercom offer multi-channel support, but they are often too complex, requiring separate apps or significant configuration.
  - **OHC Differentiation**: We need a natively built, high-performance omnichannel inbox that feels invisible to the owner. The AI assistant should triage and pre-draft responses.
  - **Technical Gap**: Currently, our integration landscape has separate modules (whatsapp, meta, sendgrid) but lacks a cohesive, low-latency, real-time message bus and unified data model for an Omnichannel Inbox built fully in Rust, leveraging WebSockets/SSE for mobile-first delivery.

  ## Design Doc (Architecture & UX)

  ### High-Level Architecture
  The system will introduce a unified `OmnichannelInbox` domain in Rust.

  **Mermaid ER Diagram:**
  ```mermaid
  erDiagram
      Tenant ||--o{ Conversation : has
      Conversation ||--o{ Message : contains
      Conversation }|--|| Customer : involves
      Conversation {
          uuid id
          uuid tenant_id
          string status "open, closed, snoozed"
          string primary_channel "whatsapp, ig_dm, email"
      }
      Message {
          uuid id
          uuid conversation_id
          uuid sender_id "null if customer, ai_agent_id, or owner_id"
          string content
          string direction "inbound, outbound"
          timestamp created_at
      }
      ChannelAdapter ||--o{ Message : translates
  ```

  ### Mobile UX Flow (375px First)
  1. **Bottom Tab**: A new "Inbox" tab with a badge for unread messages.
  2. **List View**: A unified, translucent-glass styled list of conversations. Badges indicate the source (WhatsApp, IG, Email) but the visual hierarchy prioritizes the customer name and the urgency/AI-draft status.
  3. **Detail View**: A familiar chat interface. If the AI has drafted a reply, it appears as a distinct, colored bubble with a prominent "Review & Send" button.
  4. **Action**: Priya taps "Send", and the Rust backend routes it through the correct `ChannelAdapter` transparently.

  ### AI Agent Integration
  - **Trigger**: New inbound message event on the unified message bus.
  - **Action**: The Customer Service AI department receives the context (conversation history, customer tags), queries inventory if needed, and inserts a `draft` message into the conversation stream.

  ## Implementation Prompt
  Implement the core `OmnichannelInbox` native Rust engine.
  1. Define the multi-tenant data structures (Conversation, Message) with strict `tenant_id` isolation.
  2. Implement the unified WebSocket/SSE real-time event broadcaster so the mobile UI updates instantly when a message arrives.
  3. Create the generic `ChannelAdapter` trait that existing integrations (WhatsApp, Meta) will implement to translate platform-specific webhooks into unified `Message` structs and route outbound replies.
  Ensure zero mock data is used; the system must handle real payloads and persist to the database. All UI (if implemented in this slice) must use macOS translucent glass tokens.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
