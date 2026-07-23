issue_title: "Native Rust Omnichannel Inbox & Chat System (Replacing Chat Platform)"
issue_description: |
  # Native Rust Omnichannel Inbox & Chat System

  **Priority:** P0 (Critical)
  **Estimated Scope:** Large

  ## Problem Statement
  Owner/operators (like Maya and Carlos) need a single, unified inbox to manage customer communications across Instagram DMs, SMS, WhatsApp, and Web Chat. Currently, these messages are scattered, and rely on external systems like Chat Platform, which creates disconnected experiences, delays in AI agent responses, and complexity in data sovereignty. We need a native, lightning-fast omnichannel chat system built into OHC so the AI work assistant can seamlessly triage messages, draft replies, and link conversations to bookings or orders without switching contexts.

  ## Research Report
  Based on an audit of the Chat Platform open-source repository (`app/models` and `app/controllers`), the core omnichannel architecture relies on a few key pillars:
  1. **Inbox & Channel Abstraction**: `Inbox` represents a queue of conversations, while `Channel` (e.g., `Channel::WebWidget`, `Channel::TwilioSms`, `Channel::FacebookPage`) handles platform-specific integrations and webhook ingestion.
  2. **Conversation & Message Flow**: A `Conversation` links a `Contact` (the customer) to an `Inbox`. `Message` records represent individual interactions, with polymorphic associations to attachments and external IDs (like WhatsApp message IDs) to ensure idempotency.
  3. **Real-time WebSockets**: Chat Platform relies on ActionCable for real-time bidirectional syncing of messages and typing indicators to the frontend widget and agent dashboard.
  4. **Agent & Automation Routing**: Auto-assignment policies and automation rules (macros/canned responses) dictate which human or bot handles a conversation.

  Compared to HubSpot Breeze or Shopify Inbox, Chat Platform's data model is robust for support but lacks deep native commerce/booking integration. By building this natively in Rust within OHC, we can tightly couple the `Conversation` model with OHC's existing `Customer`, `Order`, and `Booking` domains, allowing the AI assistant to instantly see a customer's purchase history while drafting a reply.

  ## Design Doc

  **Architecture Diagram**
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      INBOX ||--o{ CHANNEL_ADAPTER : configured_with
      CHANNEL_ADAPTER {
          string type "WebWidget, TwilioSMS, InstagramDM"
          json credentials
      }
      INBOX ||--o{ CONVERSATION : contains
      CONVERSATION ||--o{ MESSAGE : contains
      CONVERSATION }o--|| CONTACT : belongs_to
      MESSAGE {
          string content
          string external_source_id
          string sender_type "Customer, Agent, AI"
      }
      TENANT ||--o{ CONTACT : has
      MESSAGE ||--o{ ATTACHMENT : includes
  ```

  **Mobile UX Flow (375px first)**
  1. **Unified Feed**: The OHC mobile home screen presents an "Action Feed". Urgent unread messages bubble up as action cards (e.g., "Maya, 3 new Instagram DMs about custom cakes").
  2. **Conversation View**: Tapping a card opens a chat interface optimized for mobile (similar to iMessage, adopting the macOS-style Translucent Glass materials). The top header shows the customer's name and their most recent order/booking status.
  3. **AI Drafts**: Above the keyboard, the AI assistant presents a suggested draft response. The owner can tap "Send", "Edit", or type their own manual reply.
  4. **Context Switch**: A swipe-left drawer reveals the customer's full profile, past orders, and internal notes.

  **AI Agent Integration Points**
  - **Triage Agent**: Listens to the incoming webhook stream for new messages, categorizes intent (e.g., "Booking Inquiry", "Complaint"), and assigns a priority.
  - **Drafting Agent**: Automatically generates a proposed `Message` draft based on the context of the conversation and the business's knowledge base (RAG).
  - **Execution Agent**: Can interpret commands within the chat (e.g., if the owner types "/book Friday", the agent parses the intent and creates a calendar booking).

  **Key Design Decisions**
  - **Native Rust Implementation**: Replaces external Chat Platform dependency to ensure sub-millisecond latency, strict row-level multi-tenant data isolation, and deep integration with OHC's internal event bus.
  - **Unified Event Bus**: Incoming messages via Webhooks (Twilio, Meta) are normalized into a standard `Message` payload and published to OHC's message broker for real-time WebSocket delivery and AI processing.
  - **Abstract Channel Adapters**: The system uses a generic `ChannelAdapter` trait, making it trivial to add new platforms (e.g., WhatsApp, Line) without modifying the core conversation logic.

  ## Implementation Prompt
  Implement the foundational native Rust omnichannel chat engine and database schema to replace Chat Platform.
  **User-Facing Outcome:** The business owner sees a unified inbox in their OHC mobile app where they can receive and reply to web chat and SMS messages. The AI assistant can observe these messages and propose drafts.
  **Critical User Journey (CUJ):**
  1. An external webhook (e.g., Twilio SMS or Web Widget) hits the OHC API.
  2. The system normalizes the payload, creates or updates a Contact, and appends a Message to the appropriate Conversation in the Inbox.
  3. The real-time WebSocket server pushes the new Message to the active OHC mobile app session.
  4. The owner views the message in a 375px mobile-friendly chat view and types a reply.
  5. The system routes the reply back through the correct Channel Adapter to the customer.
  **Acceptance Criteria:**
  - Native Rust data models and DB migrations for Inbox, Channel, Conversation, Message, and Contact.
  - Abstract channel adapter interface with at least two implementations (e.g., Web Widget, Mock SMS).
  - Real-time WebSocket delivery of messages to a Flutter/Web client.
  - E2E Playwright test proving a message can be received via API, displayed in the UI, and replied to.
  - NO external Chat Platform dependencies. Use strict multi-tenant isolation with tenant_id row-level security.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
