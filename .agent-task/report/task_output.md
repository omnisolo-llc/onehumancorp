issue_title: "Architect & Build Native Rust Omnichannel Chat (Legacy External Dependency Replacement)"
issue_description: |
  **Title**: Architect & Build Native Rust Omnichannel Chat (Legacy External Dependency Replacement)

  **Problem Statement**:
  Currently, Maya the home baker and Carlos the handyman struggle to manage their customer inquiries coming from multiple channels (WhatsApp, Web Widgets, Instagram DMs, etc.) because they either have to switch between apps or rely on third-party integrations that are disjointed from their core business workflows (inventory, bookings, payments). They need a unified inbox embedded directly within OHC where their AI assistant automatically triages messages, drafts replies, and contextually connects conversations to bookings and payments. Relying on an external legacy external dependency deployment introduces latency, disjointed data models, and breaches our tenant-isolation architecture.

  **Research Report**:
  - **Codebase & Docs Audit**: OHC currently lacks a unified inbox. The `src/server/integrations/chat/` directory only contains a README. External legacy external dependency usage has been completely retired by mandate.
  - **Legacy External Dependency Architecture Benchmarking**: Inspection of legacy external dependency (`app/models/conversation.rb`, `app/models/message.rb`, `app/models/inbox.rb`, etc.) reveals core entities:
    - **Inboxes**: Channels configured per tenant (e.g., WhatsApp, Website Widget, Facebook Page).
    - **Conversations**: Threads tied to a specific `contact_id`, `inbox_id`, and `account_id` (tenant). Maintains SLA, status (`open`, `resolved`), priority, and assignee.
    - **Messages**: Individual pieces of communication within a conversation. Can be inbound/outbound, private (internal notes), text/attachments.
    - **Contacts**: Omnichannel identity resolution (linking WhatsApp phone number with Web Widget session).
  - **Competitor Systems Audit**: Shopify Inbox and Wix Chat natively embed the messaging experience into the merchant's workflow. This allows the chat UI to inject product cards, payment links, and order statuses directly into the conversation. OHC must achieve the same tight coupling but powered by AI triage and Rust's performance.

  **Design Doc**:

  *Architecture Diagram (Mermaid.js)*:
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      TENANT ||--o{ CONVERSATION : has
      INBOX ||--o{ CONVERSATION : routes
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o| ATTACHMENT : includes

      INBOX {
          uuid id
          uuid tenant_id
          string channel_type "whatsapp, web_widget, ig_dm"
          jsonb credentials
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status "open, snoozed, resolved"
          timestamp last_activity_at
      }
      MESSAGE {
          uuid id
          uuid conversation_id
          uuid tenant_id
          string content
          string message_type "incoming, outgoing, template"
          boolean is_private_note
          string sender_type "contact, agent, bot"
      }
  ```

  *UI Wireframes & Mobile UX Flow (375px)*:
  - **Unified Inbox Feed (Home)**: A clean, UniFi-style list of active conversations. Each row shows the contact avatar, channel icon (e.g., WhatsApp), a snippet of the latest message, and a timestamp. Unread messages have a bold translucent indicator.
  - **Conversation View**: Full-height chat interface. Top app bar shows Contact Name and Channel. Sticky bottom input area with native keyboard support.
  - **Action Menu**: A "+" button next to the chat input allows the chat user to inject quick actions: "Send Payment Link", "Send Product/Service Catalog", "Request Booking".
  - **AI Triage Glass Card**: At the top of an unresolved conversation, a translucent glass card shows the AI's suggested action (e.g., "Drafted a reply confirming vegan cake availability. [Approve & Send]").

  *AI Agent Integration Points*:
  - **Triage Agent (On `message_created` event)**: Automatically assesses incoming messages, tags intent, updates conversation priority, and invokes the Customer Assistant to draft a reply.
  - **Customer Assistant (Background)**: Has access to the tenant's memory (past orders, FAQs, inventory). Drafts contextual replies as "private notes" or proposed drafts for the user to approve.

  *Key Design Decisions & Why*:
  - **Unified Rust Core**: Implement the messaging engine entirely in Rust (`ohc-mono`) for low-latency WebSocket broadcasting to the UI and fast webhook ingestion from Meta/Twilio.
  - **Strict Tenant Isolation**: Every chat entity (Inbox, Conversation, Message, Contact) MUST have a `tenant_id` enforced by PostgreSQL Row Level Security (RLS).
  - **AI-First Abstraction**: Instead of routing to human agents like classic legacy external dependency, conversations default to routing to the "AI Assistant". Human users only intervene when the AI escalates or proposes a high-stakes draft.

  **Implementation Prompt**:
  *Objective*: Implement the core data models and service layer for the native Rust Omnichannel Chat system.
  *CUJ*: The user (e.g., Maya) opens her app and sees a Unified Inbox showing a new WhatsApp message from a customer. She clicks it, sees an AI-drafted reply, and clicks "Approve & Send", which sends the message back out through the channel adapter.
  *Acceptance Criteria*:
  1. Define Protobuf schemas for Inbox, Contact, Conversation, and Message entities.
  2. Implement the PostgreSQL persistence layer (with RLS) for these entities in Rust.
  3. Create the gRPC/REST APIs for the frontend to list conversations, fetch messages, and send replies.
  4. Implement a dummy/local Web Widget channel adapter for E2E testing without external dependencies.
  5. Provide at least 5 Playwright E2E tests covering the creation of a conversation, viewing the inbox, and sending a reply.
  6. Ensure 100% unit test coverage for the new Rust services.
  7. Mobile-first UI: The inbox list and conversation view must be fully usable on a 375px width.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, chat]
assignees: []
