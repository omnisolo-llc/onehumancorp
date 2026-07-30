issue_title: "Native Rust Omnichannel Chat System Architecture"
issue_description: |
  ## Native Rust Omnichannel Chat System Architecture (Replacing Chatwoot)

  **Problem Statement:**
  OneHumanCorp (OHC) currently relies on external systems or lacks a deeply integrated native omnichannel customer support and chat engine. Relying on an external service like Chatwoot breaks the unified multi-tenant architecture, introduces latency, complicates Zero-Trust SPIFFE/SPIRE identity management, and makes it harder for our AI agents to seamlessly participate in real-time conversations. Our target non-technical owners (like Maya the baker, Carlos the handyman) need a unified inbox where Instagram DMs, WhatsApp messages, website live chat, and email all flow into one place, allowing them—or their AI assistant—to reply instantly without managing multiple tools or understanding complex routing.

  **Research Report:**
  As mandated, Chatwoot is being 100% retired. A detailed audit of the `chatwoot/chatwoot` source code reveals its core architectural strengths:
  - **Data Models:** `Account` (Tenant), `User` (Agent/Owner), `Inbox` (Channel grouping), `Channel` (Specific adapter like WhatsApp, Twitter, WebWidget), `Contact`, `Conversation`, and `Message`.
  - **Real-time Messaging:** WebSockets pushing updates to connected clients.
  - **Extensibility:** Webhooks and Agent Bot integrations.
  - **State Management:** Conversations have statuses (Open, Resolved, Snoozed) and clear assignments.

  Leading platforms (Shopify, Stripe, Intercom) implement similar native architectures using high-performance languages (Rust/Go) to handle thousands of concurrent WebSocket connections while maintaining strict tenant isolation. OHC must implement this natively in Rust to leverage our existing multi-tenant PostgreSQL schema, Bazel build system, and AI Agent Swarm (Operations, CS, Sales).

  **Design Doc (Architecture & UX):**

  *Mobile-First UX Flow (375px):*
  1. **Unified Inbox View:** A single scrollable list of open conversations. Each item shows a channel icon (Insta, WhatsApp, Web), contact name, last message preview, and an unread badge. Translucent glass styling applies to the header and bottom nav.
  2. **Conversation View:** Tapping a conversation opens a standard chat UI. Messages are clearly bubbled (Agent/Owner on right, Contact on left).
  3. **AI Assistant Integration:** A prominent "Draft Reply" button (sparkles icon) uses the Customer Assistant agent to suggest a context-aware response. The owner can tap to approve and send.
  4. **Context Drawer:** Swiping left reveals contact details, past orders, and custom attributes, ensuring the owner has all context.

  *System Architecture (Rust):*
  - **Data Model (PostgreSQL):**
    - Strict Multi-Tenancy: Every table includes `tenant_id` and enforces Row Level Security (RLS).
    - `Inboxes`: Groups channels for a tenant.
    - `Channels`: Configurations for specific platforms (Web Widget, WhatsApp API, IG Graph API).
    - `Contacts`: Unified customer profiles.
    - `Conversations`: Belongs to Inbox and Contact. Tracks status (`open`, `resolved`, `snoozed`).
    - `Messages`: Belongs to Conversation. Tracks sender type (Contact, Owner, AI Agent).
  - **Real-Time Layer (WebSocket):**
    - A dedicated Rust WebSocket service handling client connections.
    - Uses Redis Pub/Sub for broadcasting message events across instances to the correct connected clients based on `tenant_id` and `user_id`.
  - **Channel Adapters (Ingress/Egress):**
    - Rust traits defining the interface for `send_message` and `receive_webhook`.
    - Initial implementations: Web Widget API, Dummy/Mock Adapter for E2E testing.
  - **AI Department Coordination:**
    - When a new message arrives, a background job is enqueued (via Postgres `SKIP LOCKED`).
    - The **Customer & Relationship Assistant** picks up the job, analyzes intent, updates the `Contact` memory, and optionally drafts a reply in the `Messages` table with a specific `draft` status, which the real-time layer pushes to the owner's UI for approval.

  *Mermaid Diagram:*
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : has
      TENANT ||--o{ CONTACT : has
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : contains
      CONTACT ||--o{ CONVERSATION : participates_in
      CONVERSATION ||--o{ MESSAGE : contains

      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
          datetime last_activity_at
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string sender_type
          boolean is_draft
      }
  ```

  **Implementation Prompt for Implementer Agent:**
  "Implement the core Native Rust Omnichannel Chat backend and database schema to replace Chatwoot.
  1. Create the PostgreSQL migration for `inboxes`, `channels`, `contacts`, `conversations`, and `messages`, ensuring `tenant_id` and RLS are strictly enforced.
  2. Implement the Rust data models and generic repository layer for these entities.
  3. Create a unified REST API endpoint `/api/v1/conversations` for the frontend to fetch the unified inbox feed.
  4. Implement a basic WebSocket handler that allows clients to subscribe to conversation updates.
  5. The CUJ to satisfy: A non-technical owner (like Maya) opens the app, fetches her open conversations across all simulated channels, and can send a message that is persisted and broadcasted back via WebSocket.
  6. Ensure 100% unit test coverage for the new Rust modules and at least one Playwright E2E test verifying the unified inbox feed loads correctly with mock data flowing through the real DB."

  **Priority:** P0
  **Estimated Scope:** Large

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
