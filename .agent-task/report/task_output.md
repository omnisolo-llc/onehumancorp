issue_title: "[Native Chat] Custom Rust Omnichannel Chat System Replication"
issue_description: |
  # Problem Statement
  OneHumanCorp (OHC) is officially retiring Chatwoot as a third-party omnichannel solution, as it is complex and hard to natively integrate into OHC's "owner work assistant" philosophy. SMB operators like Maya, Carlos, and Fatima receive inquiries across multiple platforms (WhatsApp, IG DMs, email, SMS, etc.) and need them unified in one place. Relying on an external Chatwoot deployment adds maintenance burden, complicates multi-tenant data isolation, and slows down AI coordination (e.g., proactive drafting of replies). We need a Native Rust Omnichannel Chat Engine that replicates core Chatwoot features, built directly into the OHC monolithic Rust stack, natively leveraging our existing agent event mesh and unified customer graphs.

  # Research Report
  **Chatwoot Source Code Audit Findings (https://github.com/chatwoot/chatwoot):**
  - **Data Models:** Chatwoot uses extensive models like `account` (tenant), `inbox`, `conversation`, `message`, `contact`, `channel`, `agent_bot`, `macro`, and `canned_response`.
  - **Channels & Adapters:** They abstract platforms via `Channel` models (`Channel::Whatsapp`, `Channel::Email`, `Channel::Api`, `Channel::WebWidget`).
  - **Real-Time Comm:** It relies heavily on WebSockets (ActionCable) to broadcast events (`conversation.created`, `message.created`) to the frontend and connected agents.
  - **Automation & Routing:** Inbox routing assigns conversations to users, teams, or bots. Automation rules trigger webhooks or internal state changes based on message intent.

  **OHC Gap Analysis:**
  - OHC currently lacks a built-in messaging gateway to ingest, store, and stream these multi-channel messages natively.
  - We need to translate Chatwoot's Rails-based ActiveRecords and ActionCable WebSockets into OHC's Rust multi-tenant data models, Redis event mesh, and gRPC/REST APIs.
  - Unlike Chatwoot (which is agent-to-customer), OHC is **Assistant-to-Owner**. The AI (The Ambassador) must sit in the middle, intercepting the webhook, looking up the `Unified Customer Graph`, and proactively drafting a response for the owner's mobile feed, instead of just dumping messages into a unified inbox for the owner to manually read and reply.

  # Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhooks: IG, WhatsApp, Email] -->|Ingest| B(Rust: Omnichannel Gateway Service)
      B --> C{Rust: Message Normalizer & Validator}
      C -->|Store| D[(PostgreSQL: Multi-tenant Chat Schema)]
      D --> E[Conversations, Messages, Inboxes, Contacts]
      C -->|Publish| F(Redis Pub/Sub Event Mesh)
      F --> G[Rust: Ambassador AI Agent Hook]
      G -->|Query Context| H[(PostgreSQL: Customer Graph)]
      G -->|Propose Draft| D
      F --> I(Rust: WebSocket Broadcaster)
      I --> J[OHC Flutter Frontend / Web]
      J -->|Approve/Edit Draft| B
  ```

  ### Core Data Entities & Multi-Tenancy (Rust Models)
  - All tables MUST strictly enforce row-level tenant isolation using `tenant_id`.
  - **`chat_inboxes`**: Represents a channel integration (e.g., WhatsApp Business Account, IG Account).
  - **`chat_contacts`**: Represents the external user, mapped to OHC's global `Customer` identity graph.
  - **`chat_conversations`**: A thread between a `contact` and an `inbox`. Tracks status (Open, Snoozed, Resolved).
  - **`chat_messages`**: Individual messages. Includes `sender_type` (Contact, Owner, Agent), `content`, `attachments`, and crucially, `is_draft` (boolean) to support AI proactive drafting.

  ### Mobile UX Flow (375px First)
  - **Work Triage Feed:** The owner (e.g., Maya) opens the app. A unified glassmorphic card appears: "Action Required: Approve WhatsApp Reply to Sarah".
  - **Interaction:** Tapping expands the thread. Top half: context (Sarah's last order). Bottom half: AI-drafted reply.
  - **Actions:** Prominent "Send" (Primary) and "Edit" (Secondary).
  - **WebSocket:** Upon sending, the message is instantly marked sent via WebSocket update, and the card fades out, maintaining a zero-clutter inbox.

  ### AI Agent Integration Points
  - **The Ambassador:** Subscribes to the `message.created` event via Redis Pub/Sub. When a contact sends a message, the Ambassador queries the Customer Graph and Product Catalog, inserts a new `chat_messages` row with `is_draft=true`, and publishes a `draft.created` event to update the owner's UI.

  ### Security & Zero Trust
  - Multi-tenant isolation at the DB layer via `ENABLE ROW LEVEL SECURITY`.
  - Webhooks from external providers (WhatsApp/IG) must be strictly validated via signature hashes in the Rust Gateway before processing.

  # Implementation Prompt

  **User-Facing Outcome:**
  As an owner (e.g., Maya), I do not need a separate complex Chatwoot dashboard. When a customer messages my WhatsApp, the message is natively ingested into OHC. My OHC Assistant (The Ambassador) reads the message, checks the customer's history, and drafts a reply. I simply open my OHC app, see the draft in my triage feed, and tap "Approve".

  **CUJ & Acceptance Criteria:**
  1. Implement the Core Rust Data Models (`Inbox`, `Contact`, `Conversation`, `Message`) in the OHC native PostgreSQL schema, ensuring strict `tenant_id` isolation.
  2. Implement a generic API endpoint (Omnichannel Gateway) in Rust to ingest mock webhooks, parse them into a standardized format, and store them in the DB.
  3. Implement the Redis Pub/Sub event publisher that fires a `chat.message.created` event upon ingestion.
  4. Implement a mock Rust Ambassador agent listener that catches `chat.message.created` and inserts a drafted reply (`is_draft=true`) back into the DB.
  5. Expose REST/gRPC endpoints for the frontend to fetch conversations and approve/send drafted messages.
  6. **UI/E2E Verification (Playwright):** Create a Playwright E2E test where: A test simulates an incoming webhook. The user logs into the OHC frontend. The unified triage feed displays the AI-drafted reply. The user taps "Approve". The message status updates to sent (simulated dispatch).

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
