issue_title: "[Omni-Channel Inbox] Architect and Implement Unified Inbox Data Model and Webhook Ingestion"
issue_description: |
  # Omni-Channel AI Inbox Architecture & Implementation

  ## Problem Statement
  Small business owners (e.g., Maya the Home Baker) receive customer inquiries across multiple platforms—Instagram DMs, WhatsApp, SMS, Email, and Facebook Messenger. Keeping track of these messages, responding promptly, and ensuring consistency across channels is overwhelming. The current disconnected communication silos prevent AI agents from providing unified, context-aware support. A customer asking "Do you do vegan cakes?" on Instagram needs a seamless AI reply without the business owner having to manually switch contexts.

  ## Research Report
  The current market (Shopify, Wix) either lacks a unified inbox entirely or provides limited, chat-only integrations (like Shopify Inbox) that don't deeply integrate with a full suite of business operations (inventory, bookings). Tools like Zendesk or Intercom are too complex and expensive for micro-businesses.
  Integrating Meta Graph API (Instagram/Messenger), WhatsApp Business API, Twilio (SMS), and Resend/IMAP (Email) into a single Event Mesh within OHC will allow our AI agents to consume, analyze, and respond to all messages centrally.

  *Competitor Comparison:*
  - **Shopify Inbox:** Good for web chat and basic Instagram, but not deeply autonomous.
  - **Wix Inbox:** Basic consolidation, limited AI.
  - **OHC:** True autonomous agentic inbox where AI drafts, approves, or auto-sends replies based on business context (inventory, calendar, policies) across *all* channels.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph External Channels
          IG[Instagram DM]
          WA[WhatsApp]
          FB[FB Messenger]
          SMS[SMS / Twilio]
          Email[Email / Resend]
      end

      subgraph OHC Integration Layer
          MetaWebhook[Meta Webhook Handler]
          TwilioWebhook[Twilio Webhook Handler]
          EmailWebhook[Email Webhook Handler]
      end

      subgraph Core
          MessageBus[NATS / Redis PubSub]
          UnifiedInboxDB[(Postgres Unified Inbox)]
          AgentQueue[AI Job Queue]
      end

      subgraph AI Agents
          CSAgent[Customer Success Agent]
          OpsAgent[Operations Agent]
      end

      IG --> MetaWebhook
      WA --> MetaWebhook
      FB --> MetaWebhook
      SMS --> TwilioWebhook
      Email --> EmailWebhook

      MetaWebhook --> MessageBus
      TwilioWebhook --> MessageBus
      EmailWebhook --> MessageBus

      MessageBus --> UnifiedInboxDB
      MessageBus --> AgentQueue

      AgentQueue --> CSAgent
      AgentQueue --> OpsAgent

      CSAgent -.-> |Drafts/Sends Reply| UnifiedInboxDB
      OpsAgent -.-> |Takes Action e.g. Books| UnifiedInboxDB
  ```

  ### UI Wireframes & Mobile UX Flow
  - **Home Screen (375px):** An "Inbox" tab with a notification badge.
  - **Inbox View:** A consolidated list of conversations. Each item shows the customer's name, the latest message snippet, and an icon indicating the channel (Instagram, WhatsApp, etc.).
  - **Conversation View:** A chat interface (Translucent Glassmorphism style).
      - Messages from the customer appear on the left.
      - AI-drafted replies appear at the bottom with a prominent "Approve & Send" button or a "Edit" button.
      - If the AI auto-replied (based on confidence threshold), it shows "Sent by AI" below the message bubble.
  - **Context Panel (Drawer/Swipe):** Swiping left reveals customer context: previous orders, upcoming bookings, and notes.

  ### AI Agent Integration Points
  - **Customer Success Agent:** Listens to incoming messages. If the intent is a general inquiry, it queries the RAG knowledge base and drafts a reply.
  - **Operations Agent:** If the message intent is transactional (e.g., booking change), this agent handles the state change and drafts the confirmation reply.

  ### Key Design Decisions
  - **Event-Driven:** Webhooks normalize incoming payloads into a standard `OmniMessage` format before hitting the message bus.
  - **Asynchronous AI Processing:** AI generation must happen in the background queue so webhooks can return 200 OK immediately.
  - **Human-in-the-Loop by Default:** Initially, the AI *drafts* replies. The business owner must approve them.

  ## Implementation Prompt
  Implement the foundational data models, Protobuf definitions, and webhook handlers for the Omni-Channel AI Inbox.
  1. Define `OmniMessage` and `Conversation` models in Protobuf (e.g. `src/proto/inbox.proto`) and PostgreSQL schema, ensuring strict `tenant_id` isolation.
  2. Implement a unified webhook API service in Go (e.g., `src/server/api/inbox.go`) that can accept incoming messages (start with Meta Graph API structure for Instagram/WhatsApp as the MVP).
  3. Normalize the incoming Meta payload into the `OmniMessage` struct and persist it to the database.
  4. Publish an event to the internal message bus (e.g. using the existing `hub` infrastructure) so the AI agents can process it asynchronously.
  5. Create a basic API endpoint to fetch conversations and messages for a specific tenant, to be used by the mobile frontend.
  6. Implement comprehensive unit tests and Playwright E2E tests verifying a mock webhook ingestion, data persistence, and API retrieval.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
