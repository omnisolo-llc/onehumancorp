issue_title: "[Research] Architect Native Rust Omnichannel Chat System"
issue_description: |
  **Problem Statement**
  The overarching goal for OneHumanCorp (OHC) is to act as an AI-powered assistant for business owners (like Maya the Baker, Carlos the Handyman). A critical requirement is an Omnichannel Chat system (replacing third-party services like Chatwoot) integrated directly into the core Rust/Go system.
  Currently, there is no native, deeply integrated omnichannel communication system that acts as a central hub (Work Triage) capturing DMs, SMS, Web Chat, and Email into a unified 'Inbox'. Without this, our AI Agents (e.g., the Customer Assistant) cannot draft replies seamlessly, nor can we trace interactions perfectly into work/bookings.

  **Research Report**
  As mandated, Chatwoot dependency is retired. We need a native Rust implementation.
  Benchmarking Chatwoot’s core source code (`https://github.com/chatwoot/chatwoot`), the primary domain model for real-time customer messaging involves:
  - Accounts (Tenants in OHC)
  - Inboxes (Channels like Web Widget, Email, FB Messenger, SMS)
  - Contacts (Customers)
  - Conversations (Threads between a Contact and an Inbox)
  - Messages (Individual payloads inside a Conversation)
  - Agents/Assignees (Staff or AI Bots handling a Conversation)

  To achieve parity while honoring OHC’s multi-tenant architecture and "assistant-first" philosophy, our core Rust architecture must handle:
  - Zero-Trust multi-tenant isolation (`tenant_id` on every table).
  - WebSockets for real-time delivery to the Flutter frontend and incoming channel webhooks.
  - An Agentic bridge, so when a Message is created, it triggers the "Customer Assistant" AI job (via a Postgres SKIP LOCKED queue) to draft a response.

  **Design Doc**
  *Architecture*
  We will introduce a Rust microservice/module: `chat_core`.
  - **Data Models (PostgreSQL Row-Level Security)**
    - `inboxes`: `id, tenant_id, name, channel_type (email, sms, web), settings`
    - `contacts`: `id, tenant_id, name, email, phone, external_id`
    - `conversations`: `id, tenant_id, inbox_id, contact_id, status (open, snoozed, resolved), assignee_id (AI or human)`
    - `messages`: `id, tenant_id, conversation_id, sender_type (contact, agent, ai), content, status (sent, read)`
  - **WebSocket Hub**: A Rust-based WebSocket server tracking connected clients (by `tenant_id` and `user_id`) to push real-time `MessageCreated`, `ConversationUpdated` events.
  - **AI Coordination**: On `MessageCreated` (where sender is 'contact'), a background job is enqueued in the AI Job Queue (Postgres `SKIP LOCKED`). The "Customer Assistant" dequeues, reads the conversation history, and drafts a reply, creating a `Message` with `sender_type='ai', status='draft'`. The owner sees this draft in their Work Triage UI.

  *Mobile-First UX (375px Flow)*
  - The OHC Assistant Home (Work Triage) shows a unified list of "Conversations needing attention".
  - Tapping a conversation opens a standard Chat view:
    - Top bar: Customer name + Channel icon (e.g., Instagram, Web).
    - Chat bubble stream.
    - Floating Action Button (FAB) or bottom docked area: "Approve AI Draft" (if pending), or a text input to manually reply.
    - All elements must use OHC Premium Token (macOS Translucent Glass styles).

  **Implementation Prompt**
  Implement the `chat_core` native Rust omnichannel models, database migrations, and a basic REST/gRPC API for Conversations and Messages.
  - Create the exact schema for `inboxes`, `contacts`, `conversations`, and `messages` ensuring `tenant_id` is present on all and Row Level Security is enforced.
  - Build endpoints to Create an Inbox, Start a Conversation, and Send a Message.
  - When a message is sent by a contact, insert a row in the background AI job queue table to trigger the AI Assistant.
  - DO NOT build the full WebSocket system yet; focus on the data model and API first.
  - Acceptance Criteria: A full E2E test in Playwright that provisions a tenant, creates an inbox, simulates an incoming webhook message, and verifies the message appears via API for the tenant (and triggers the AI draft queue).

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
