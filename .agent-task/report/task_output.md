issue_title: "Native Rust Omnichannel Chat & Support System"
issue_description: |
  ### Problem Statement
  OmniSolo (formerly One Human Corp) has retired the legacy external chat service dependency, meaning there is currently a critical architectural gap for our owner/operator personas. They need a unified omnichannel inbox to manage customer communications across SMS, email, WhatsApp, Web Widget, and Instagram DMs. Without a robust native chat architecture, our non-technical operators (like Maya the baker and Carlos the handyman) cannot reliably coordinate customer inquiries, send quotes via chat, or enable AI agents to automatically handle incoming leads.

  ### Research Report
  **Legacy Source Code Audit Findings:**
  I conducted a deep-dive analysis of the legacy chat codebase to understand its data model and real-time execution flows:
  - **Data Models:**
    - `Inbox`: Represents a channel endpoint (e.g., specific Facebook Page, Email inbox). Includes configuration like auto-assignment rules.
    - `Conversation`: Links an `Inbox` to a `Contact`. Stores metadata like `status` (open, resolved, pending), `assignee_id`, and `unread_count`.
    - `Message`: The core primitive. Can be incoming or outgoing. Supports `attachments` and rich `content_attributes` (useful for interactive templates).
    - `Contact`: Represents the customer. Can have multiple `contact_inboxes` if they reach out across different channels.
  - **Real-time Architecture:** Uses ActionCable/WebSockets to push events to agents instantly.
  - **Competitor Benchmarking:** Shopify Inbox provides a deeply integrated storefront chat that understands cart context. Our new Rust system must go beyond generic support to integrate deeply with OHC's product catalog and booking systems.

  ### Design Doc

  **Architecture Diagram:**
  ```mermaid
  erDiagram
      ACCOUNT ||--o{ INBOX : owns
      INBOX ||--o{ CHANNEL : configures
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains
      MESSAGE ||--o{ ATTACHMENT : includes
      AGENT_BOT ||--o{ INBOX : assigned_to

      INBOX {
          uuid id
          string name
          jsonb config
      }
      CONVERSATION {
          uuid id
          string status
          timestamp waiting_since
      }
      MESSAGE {
          uuid id
          text content
          string message_type
      }
  ```

  **Mobile UX Flow (375px First):**
  - **Inbox Hub:** The primary app screen displays a clean, unified feed of open conversations. Uses Apple-style list cards with bold unread indicators and snippet previews.
  - **Conversation View:** Tapping a conversation opens a standard chat UI. A sticky bottom input bar dynamically swaps between "Reply via WhatsApp" or "Reply via Email" based on the inbox context.
  - **Translucent Glass UI:** Employs vibrant background blur for the header and input areas to maximize reading space on small screens.
  - **Action Sheet Integration:** A floating "+" action button allows owners to instantly attach a "Payment Link" or "Booking Quote" directly into the chat flow.

  **AI Agent Integration Points:**
  - **Customer & Relationship Assistant:** Triggers instantly on `MessageCreated` webhook/event when the conversation is unassigned or pending. Evaluates intent and automatically replies or drafts a response for the owner.
  - **Handoff Mechanism:** Agents update the conversation status from `pending` (bot handling) to `open` (human intervention needed) if they detect complex sentiment or escalation requests.

  **Key Design Decisions:**
  - **Native Rust & gRPC:** We will implement the core engine in Rust within the `onehumancorp/mono` repo to ensure microsecond latency and memory safety, communicating with the UI via standard REST/gRPC.
  - **Multi-tenant Zero-Trust:** Every table and query will strictly enforce `tenant_id` Row-Level Security (RLS) in PostgreSQL.
  - **No External Dependencies:** Webhooks from Twilio/Meta will hit our Rust endpoints directly, replacing external SaaS dependencies.

  ### Implementation Prompt
  **For the Implementer Agent:**
  Your mission is to build the backend API and database schemas for the new OHC Native Inbox.
  1. Define the SQL schemas and migrations for `inboxes`, `conversations`, `messages`, and `contacts`, enforcing multi-tenant isolation via `tenant_id`.
  2. Implement the Rust service layer for creating messages and updating conversation states.
  3. Ensure the API supports receiving webhook payloads and emitting events that the AI Work Triage can subscribe to.
  4. Write comprehensive E2E and unit tests to ensure that an incoming message creates a conversation and correctly triggers an AI draft reply.
  Do not worry about the frontend UI in this PR; focus purely on the robust data and service layers.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
