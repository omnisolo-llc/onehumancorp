issue_title: "Native Rust Omnichannel Chat Engine & Multi-Tenant Inbox Architecture"
issue_description: |
  ## Problem Statement
  OmniSolo (OHC) serves non-technical business owners (Maya, Carlos, Priya, Leo, Fatima) who need to handle customer inquiries across multiple channels (Instagram DMs, Facebook Messenger, SMS, Web Widget, Email) from a single assistant-led interface. Currently, relying on external third-party tools like Chatwoot fractures the owner experience, breaks the Zero-Trust identity model, and prevents native integration with OHC's AI work triage, cart recovery, and billing agents. Owners need a native, real-time omnichannel inbox built into the OHC Rust core that transparently centralizes multi-channel communications without requiring complex third-party setups.

  ## Research Report & Feature Benchmarking
  An exhaustive audit of the Chatwoot open-source repository (`https://github.com/chatwoot/chatwoot`) reveals the core architectural capabilities required for parity:
  - **Data Models**: The core entities are `Inbox`, `Conversation`, `Message`, and `Contact`. These are tied together via a multi-tenant `account_id` (mapping to OHC's `tenant_id`).
  - **Channels**: Chatwoot abstracts channels via specific models (`Channel::WebWidget`, `Channel::Sms`, `Channel::Email`, `Channel::Whatsapp`, `Channel::Instagram`, etc.) under an Inbox.
  - **Real-Time Mesh**: Chatwoot relies on WebSockets (ActionCable) and background job processing (Sidekiq) for message delivery and webhook handling.
  - **AI & Automation**: It includes `AgentBot`, `CannedResponse`, and `Macro` for automated interactions.

  **Market Comparison**: Shopify Inbox, Meta Business Suite, and Wix Inbox all provide integrated conversational commerce. OHC must differentiate by combining this native inbox with autonomous AI agents (the Teammate Mesh) that draft replies and trigger operational actions (e.g., booking a service for Carlos, sending a payment link for Maya).

  ## Design Doc: Native Rust Omnichannel Chat System

  ### 1. Data Model & Invariants (PostgreSQL + pgvector)
  The new Rust implementation in `src/server/` will introduce the following core entities, strictly enforcing `tenant_id` for row-level security:

  - `Contact`: Represents the customer/lead. Fields: `id`, `tenant_id`, `name`, `email`, `phone_number`, `avatar_url`, `custom_attributes`.
  - `Inbox`: The aggregation point for channels. Fields: `id`, `tenant_id`, `name`, `channel_type` (e.g., `web_widget`, `instagram_dm`, `twilio_sms`).
  - `Conversation`: The thread between a contact and an inbox. Fields: `id`, `tenant_id`, `inbox_id`, `contact_id`, `status` (open, resolved, snoozed), `assignee_id`.
  - `Message`: The individual message payload. Fields: `id`, `tenant_id`, `conversation_id`, `sender_type` (contact, user, ai_agent), `content_type` (text, image, template), `content`, `external_source_id`.
  - `ChannelAdapter`: Config schema storing credentials and state for external integrations (Meta Graph API, Twilio).

  *Mermaid ER Diagram:*
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : owns
      TENANT ||--o{ CONTACT : owns
      INBOX ||--o{ CONVERSATION : hosts
      CONTACT ||--o{ CONVERSATION : participates
      CONVERSATION ||--o{ MESSAGE : contains
      INBOX ||--o| CHANNEL_ADAPTER : configured_via
  ```

  ### 2. Real-Time Teammate Mesh Integration (Redis Pub/Sub)
  - **WebSockets**: The Tauri desktop/mobile app will connect to the Rust backend via a dedicated WebSocket endpoint.
  - **Pub/Sub**: We will leverage the existing KAIROS Teammate Mesh (`redis` Pub/Sub) to broadcast `message.created`, `conversation.updated`, and `contact.merged` events across all active pods to ensure the client UI updates instantly.

  ### 3. AI Department Coordination (The "Work Triage" Agent)
  - When a `Message` is ingested via a webhook handler (e.g., from Instagram), an asynchronous event is pushed to the Sub-Agent Queue.
  - The **Customer Assistant Agent** is woken up. It reads the `Conversation` history, fetches context from `AutoDream` (Memory), and drafts a reply.
  - If the inquiry involves sales, it pings the **Sales & Revenue Agent** to generate a quote or deposit link.
  - The drafted message is saved with `sender_type = ai_agent` and `status = pending_approval`, waiting for the owner (Maya/Carlos) to approve via a single tap on their mobile UI.

  ### 4. Mobile-First UX Flow (375px Base)
  - **The Feed**: The owner opens OHC and sees the "Work Triage" view. Unread conversations bubble to the top.
  - **Conversation View**: Tapping a conversation opens a standard chat UI. At the bottom, instead of just a keyboard, the AI's drafted response is presented as a glowing, translucent card (macOS glass styling).
  - **One-Tap Actions**: The owner can tap "Approve & Send", "Edit", or "Dismiss" the AI draft.
  - **Latency Target**: Messages must render optimistically on the client within 50ms, resolving backend confirmation in the background.

  ## Implementation Prompt (For Implementer Agent)
  **Objective**: Implement the core data models, database migrations, and basic CRUD API for the Native Rust Omnichannel Inbox in `src/server/`.

  **Tasks**:
  1. Create the PostgreSQL migration file to define the tables: `contacts`, `inboxes`, `conversations`, and `messages`, ensuring `tenant_id` is present on all tables.
  2. Implement the corresponding Rust entity structs and SeaORM/SQLx models in `src/server/domain/inbox.rs` or appropriate data layer.
  3. Create the REST/gRPC API endpoints to create an Inbox, list Conversations for an Inbox, and append a Message to a Conversation.
  4. Ensure all API endpoints enforce multi-tenant isolation (Zero Trust).
  5. **Verification**: Write 100% coverage unit tests for the new API endpoints and ensure `bazel test //...` passes.

  **Acceptance Criteria**: A non-technical owner can have a simulated conversation created via the API, and the data is correctly isolated by their `tenant_id`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, core-platform]
assignees: []