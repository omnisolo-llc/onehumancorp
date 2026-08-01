issue_title: "Native Omnichannel Chat Engine (Rust): WhatsApp, Web Widget, & Email Parity"
issue_description: |
  **Mission Queue Protocol Brief**

  ## Problem Statement
  Owners and operators currently need to triage work across too many tools (WhatsApp for some clients, Instagram DMs for others, web chat for support, and email). They are dropping leads and missing follow-ups because there is no unified inbox that they own natively. Relying on an external tool like c-woot fractures the workflow, prevents deep integration with our other AI agent capabilities, and isn't cost-effective to scale for multi-tenant deployment.

  ## Research Report (c-woot Benchmarking)
  Based on our source code analysis of `c-woot/c-woot` (commit current as of benchmarking), c-woot provides a robust open-source omnichannel inbox. Key components analyzed:
  - **Channel Adapters**: Native models (`app/models/channel/`) handle WhatsApp, Web Widget, API, Email, Facebook, Instagram, Line, SMS, Telegram, and Twitter.
  - **Data Models**: The core `Conversation` model (`app/models/conversation.rb`) connects an `account_id`, `inbox_id`, `contact_id`, and `assignee_id`. It features robust state management (status: open, resolved, pending, snoozed), SLA tracking, and metadata mapping.
  - **Web Widget**: Provides real-time WebSocket events for typing indicators, presence, and message delivery.
  - **SaaS Viability (Why Native?)**: Operating c-woot as a third-party service requires separate database infrastructure, redundant contact synchronization, and complicates AI agent handoffs (e.g. if the OHC AI needs to draft a reply). Building it natively in Rust inside `onehumancorp/mono` guarantees performance, tight coupling with OHC's existing tenant boundaries (via Row-Level Security), and zero-latency access for our AI Job Queue workers.

  ## Design Doc
  - **Architecture**: A new Rust microservice/crate `chat_engine` serving gRPC to internal OHC components and exposing Webhooks/REST to external providers (Twilio/WhatsApp, SendGrid/Email).
  - **Tenant Isolation**: All chat entities (Contacts, Conversations, Messages, Channels) will enforce `tenant_id` at the database level with PostgreSQL RLS.
  - **Channels**:
    1. **Web Widget**: Provide an embeddable JS snippet connecting to OHC via WebSockets for live chat on the owner's website.
    2. **WhatsApp**: Webhook endpoint to parse incoming WhatsApp messages (via Twilio/WhatsApp Business API) and map them to standard OHC conversations.
    3. **Email**: Inbound email parsing via SendGrid/Mailgun webhooks mapped to customer threads.
  - **Work Triage Integration**: When a message arrives, it triggers the OHC AI Job Queue. The AI `Customer & Relationship Assistant` automatically drafts a reply and puts it into the owner's "pending actions" feed.

  ## Implementation Prompt
  Create the core Rust chat engine crate that achieves feature parity with c-woot's core omnichannel routing.
  1. Implement the database schema (PostgreSQL) for `Contacts`, `Inboxes`, `Channels` (WhatsApp, Web, Email), `Conversations`, and `Messages`, ensuring strict `tenant_id` RLS.
  2. Implement the incoming webhook parsers for WhatsApp and Email, normalizing the payloads into the `Messages` table.
  3. Create the WebSocket server layer in Rust for real-time Web Widget communication (typing indicators, unread counts, message delivery).
  4. Ensure the system integrates directly with the existing OHC `Work Triage` AI agents so that every incoming inquiry automatically gets an AI-drafted response in the owner's feed.

  **Acceptance Criteria:**
  - A real owner (e.g., Maya the baker) can receive a WhatsApp message and a Web Widget chat in the exact same native OHC unified inbox.
  - The UI updates in real-time without reloading when a new message arrives.
  - The backend is written entirely in Rust, securely isolated per tenant.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
