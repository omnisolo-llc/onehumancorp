issue_title: "Architecture: Native Rust Omnichannel Chat System (Chatwoot Retirement)"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is replacing its reliance on external third-party customer support tools (like Chatwoot) with a native, highly-performant, multi-tenant Rust omnichannel chat system inside the `onehumancorp/mono` repository. OHC's core personas (Maya the Baker, Carlos the Handyman) need all their customer interactions—Instagram DMs, WhatsApp, Email, Web Chat, and SMS—unified into a single prioritized work feed. This system must handle multi-tenant routing, real-time messaging, and seamless AI agent coordination (Work Triage and Customer Assistant) invisibly in the background.

  ## Research Report
  An extensive audit of the Chatwoot source code (`https://github.com/chatwoot/chatwoot`) was conducted. Key architectural patterns from Chatwoot include:
  - **Inboxes & Channels:** An `Inbox` represents a collection point for messages, backed by various `Channel` adapters (e.g., `Channel::WebWidget`, `Channel::Whatsapp`, `Channel::Email`).
  - **Conversations & Messages:** A `Conversation` tracks the state (open, resolved, snoozed) and links a `Contact` to an `Inbox`. `Message` entities belong to conversations.
  - **Automation & Agent Bots:** System events trigger webhooks or agent bots that can auto-respond or assign conversations.

  To replicate and improve this for OHC, we will build a native Rust implementation utilizing our PostgreSQL backend with Row-Level Security (RLS) for strict multi-tenancy. Background jobs will be handled via PostgreSQL `SKIP LOCKED` queues, and real-time frontend synchronization will use WebSockets.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : configures
      TENANT ||--o{ CONTACT : manages
      INBOX ||--|| CHANNEL : is_backed_by
      INBOX ||--o{ CONVERSATION : receives
      CONTACT ||--o{ CONVERSATION : initiates
      CONVERSATION ||--o{ MESSAGE : contains

      TENANT {
          uuid id
      }
      INBOX {
          uuid id
          uuid tenant_id
          string name
      }
      CHANNEL {
          uuid id
          string provider_type
          jsonb credentials
      }
      CONVERSATION {
          uuid id
          uuid tenant_id
          uuid inbox_id
          uuid contact_id
          string status
      }
      MESSAGE {
          uuid id
          uuid tenant_id
          uuid conversation_id
          string content
          string sender_type
      }
      CONTACT {
          uuid id
          uuid tenant_id
          string name
          string identifier
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Unified Inbox Screen:** The primary screen for the owner shows a clean, Unifi-style list of active conversations. Each row displays the customer's name, a snippet of the latest message, a channel icon (e.g., Instagram, Web), and an urgency indicator.
  2. **Conversation Thread Screen:** Tapping a conversation opens the chat view. It features native mobile keyboard support, chat bubbles (incoming vs. owner vs. AI agent drafts), and quick-action buttons (e.g., "Send Quote", "Request Payment").
  3. **AI Draft Interaction:** If the Customer Assistant agent has prepared a response, it appears as a translucent, frosted-glass pending message bubble. The owner can tap "Approve & Send" or edit it directly.

  ### AI Agent Integration Points
  - **Message Ingestion:** When a new `Message` is inserted via API or webhook, a database trigger or application event pushes a job to the AI Job Queue (via `SKIP LOCKED`).
  - **Work Triage:** The Triage agent reviews the message context, categorizes the urgency, and determines if operations (e.g., creating a task) are required.
  - **Customer Assistant:** The Customer agent queries the tenant's knowledge base and previous conversation history to generate a draft reply. The draft is inserted as a `Message` with `status: pending_approval`.

  ### Key Design Decisions
  - **Rust Native Services:** All domain models and APIs will live under `src/server/ohc/domain/omnichannel` in Rust.
  - **Data Isolation:** `tenant_id` must be on every table (`inboxes`, `conversations`, `messages`) to leverage PostgreSQL RLS.
  - **Extensible Channels:** Channels will be implemented using a trait-based adapter pattern in Rust to easily add new providers (WhatsApp, IG, SMS) without modifying the core conversation engine.

  ## Implementation Prompt
  **Task for Implementer Agent:**
  Implement the foundational Rust domain models, database migrations, and core gRPC/REST APIs for the OHC Omnichannel Chat System.
  1. Create the database migrations for `inboxes`, `channels`, `contacts`, `conversations`, and `messages` ensuring `tenant_id` is present on all tables for RLS.
  2. Implement the Rust data structures and repository layer in `src/server/ohc/domain/omnichannel`.
  3. Build an API endpoint to ingest a new message from a simulated external webhook, which creates a contact (if new), a conversation, and a message record.
  4. Ensure 100% unit test coverage for the repository layer and ensure `bazel test //...` passes completely.
  *Acceptance Criteria:* A client can create an inbox, simulate receiving a message, and query the conversation via the API. No external Chatwoot dependencies are used.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
