issue_title: "Implement Native Rust Omnichannel Chat System to Replace Chatwoot"
issue_description: |
  # Native Rust Omnichannel Chat System

  ## Problem Statement
  OHC requires a high-performance, unified, omnichannel messaging architecture natively integrated into its multi-tenant platform. Currently, the system references retiring Chatwoot as an external dependency. A business owner (like Maya the Baker or Carlos the Handyman) needs a unified inbox that brings together WhatsApp, Instagram DMs, web chat, and SMS in real-time. They don't care about the underlying channels; they just want to see "Customer said X, Agent drafted Y." We need to bring Chatwoot's omnichannel concepts into our native Rust/Go backend with strict multi-tenancy and mobile-first latency requirements.

  ## Research Report
  Based on an audit of Chatwoot's source code (v3+ architecture), we observe their domain model:
  - `Account` (maps to our `Tenant`)
  - `Inbox` (the aggregation point for channels)
  - `Channel::*` (adapters like `Channel::Whatsapp`, `Channel::WebWidget`, `Channel::Instagram`)
  - `Conversation` (the threaded discussion)
  - `Message` (the individual payload, supporting text, attachments, templates)
  - `Contact` (the customer identity unified across channels)

  To achieve parity and integrate deeply with OHC's AI agent triage and Row Level Security (RLS), we must build these core entities natively.

  Competitors like Shopify Inbox, Meta Business Suite, and Zendesk all centralize webhook ingress and WebSocket egress into a unified real-time event bus. By building this natively in Rust (within `onehumancorp/mono`), we can eliminate third-party SLA risks, enforce Zero-Trust SPIFFE/SPIRE identity, and minimize latency for our 375px mobile-first frontend.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      subgraph Webhook Ingress
          W[WhatsApp Webhook] -->|HTTP POST| Gateway[API Gateway / Router]
          I[Instagram Webhook] --> Gateway
          S[SMS / Twilio Webhook] --> Gateway
      end

      subgraph Rust Chat Engine (onehumancorp/mono)
          Gateway --> ChannelAdapter[Channel Adapter Layer]
          ChannelAdapter -->|Normalize to OHC Event| InboxService[Inbox Service]
          InboxService --> DB[(PostgreSQL with RLS)]
          InboxService --> AI[AI Triage Queue]
          InboxService --> Redis[Redis Pub/Sub]
      end

      subgraph Real-time Egress
          Redis --> WebSocketServer[WebSocket Server]
          WebSocketServer --> FlutterApp[Flutter Mobile/Web App]
      end
  ```

  ### Data Model & Invariants
  1. **Strict Tenant Isolation:** Every table (`inboxes`, `conversations`, `messages`, `contacts`) MUST have `tenant_id` and enforce PostgreSQL RLS.
  2. **Core Entities:**
     - `Inbox`: Aggregates one or more channel configurations.
     - `ChannelAdapter`: Stores provider credentials securely.
     - `Conversation`: Links a `Contact` to an `Inbox`.
     - `Message`: Immutable ledger of communication, storing provider-specific `source_id` for idempotency.
  3. **Idempotency:** Webhook processing must use unique provider event IDs to prevent duplicate message creation on retries.

  ### Mobile UX Flow (375px First)
  - **Bottom Tab:** "Inbox" tab with a badge for unread unified messages.
  - **List View:** Conversations sorted by last activity. Each row shows the customer name, last message snippet, channel icon (e.g., WhatsApp logo), and AI agent draft status (e.g., "Agent drafted reply").
  - **Detail View:** Standard chat bubble interface. The input bar handles sending text or attachments seamlessly across the underlying channel.
  - **Action Sheet:** Tapping a customer avatar opens a bottom sheet showing their OHC profile (past orders, LTV, preferences).

  ### AI Agent Integration Points
  - **Work Triage:** Every incoming message triggers an async event to the AI Job Queue.
  - **Customer Assistant:** AI drafts a reply based on the message content, tenant knowledge base, and past conversation history. The draft is stored with `status = 'draft'` and displayed in the UI for the owner to approve/edit/send.

  ### Zero Trust & Security
  - All inter-service communication (e.g., Webhook Ingress to Rust Chat Engine) must use mutual TLS provided by SPIFFE/SPIRE.
  - Channel API keys must be encrypted at rest in the database.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the native Rust omnichannel chat system data models and the core Inbox service.
  1. Define the SQL schemas for `inboxes`, `channels`, `conversations`, `messages`, and `contacts` ensuring `tenant_id` RLS is applied to all.
  2. Implement the Rust structs and database repository layer for these entities.
  3. Create a unified webhook ingress endpoint that can accept normalized payloads from different channel adapters.
  4. Ensure all database operations are wrapped in transactions and handle unique constraint violations for idempotency.
  5. The primary CUJ: A webhook payload arrives, a contact is found or created, a conversation is updated or created, and a message is persisted. Verify this CUJ using a local test mimicking a real webhook.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
