issue_title: "Architecture and Dispatch Plan for OHC-Native Rust Omnichannel Chat Platform (Chatwoot Replication)"
issue_description: |
  # OHC-Native Rust Omnichannel Chat Platform (Chatwoot Replication)

  ## Problem Statement
  Small business owners and operators (such as Maya, Carlos, Priya, Leo, and Fatima) struggle to manage disjointed customer conversations across multiple channels (Web Widget, Meta/WhatsApp, Facebook Messenger, Instagram, Twilio SMS/voice, Email). Third-party omnichannel engines like Chatwoot add heavy dependencies (Rails, Redis/Sidekiq, complex configuration, lack of native Rust speed/safety) and pose major tenant isolation concerns for high-scale multi-tenant SaaS environments. By retiring Chatwoot entirely and implementing a high-performance, OHC-native, multi-tenant-safe omnichannel chat engine natively in Rust (integrated with a local-first web operator inbox and secure WebSockets), OHC will deliver a seamless unified communications hub that keeps the advanced setup completely hidden and guides non-technical owners from unclear incoming requests to automated AI-crafted or manual 1-tap actions in seconds.

  ---

  ## Competitive Benchmarking (Chatwoot vs. Shopify/Wix vs. OmniSolo)
  Based on our source code audit of Chatwoot (`https://github.com/chatwoot/chatwoot`) and leading commerce platforms, we benchmark the key dimensions of our native Rust implementation:

  | Dimension | Chatwoot (Traditional SaaS) | Shopify Inbox / Wix Chat | OmniSolo Native Rust Chat |
  |---|---|---|---|
  | **Core Architecture** | Ruby on Rails, PostgreSQL, Redis, Sidekiq | Cloud-native microservices (Go/Java), proprietary events | Rust (Axum + SQLx + Tokio), embedded memory-efficient design |
  | **Multi-Tenancy Isolation** | Account-based scoping, PostgreSQL tenant columns without deep native RLS | Proprietary organization structures, high infrastructure overhead | Strict PostgreSQL row-level security (RLS) with session-scoped `app.current_tenant_id` claims |
  | **Real-Time Delivery** | ActionCable (Ruby WebSockets) | Proprietary Server-Sent Events / Push | High-performance Tokio WebSockets with single-use, cryptographically bound tickets, integrated with PowerSync local-first sync |
  | **AI Integration** | Add-on bots, external webhooks | Manual assistant features, hardcoded triggers | Native, multi-agent AI departments (Operations, CS, Marketing, Finance) coordinating autonomously via transactional outboxes and automation rules |
  | **Device / Portability** | Mobile app wrapped in Cordova/Capacitor | Dedicated mobile/desktop apps | Tauri v2 desktop shell with shared SQLite repository, running 100% offline-first |

  ---

  ## Architectural System Design

  ### System Architecture Overview
  ```mermaid
  graph TD
      Widget[Customer Web Widget] -->|HTTPS/WS| API[Axum API Server]
      Operator[Tauri Desktop Operator App] -->|WebSocket/REST| API
      Webhook[External Channel Webhooks <br/> WhatsApp, Twilio, Messenger] -->|POST with Signatures| API
      API -->|SQLx Postgres Client| DB[(PostgreSQL with RLS)]
      API -->|Tokio Runtime| CoreChat[Native Rust Chat Engine]
      CoreChat -->|Event Bus| Realtime[WebSocket Gateway / PowerSync]
      CoreChat -->|Transactional Outbox| JobQueue[Postgres SKIP LOCKED Worker]
      JobQueue -->|AI Agent Interface| AIDept[AI Coordinator Agents]
      JobQueue -->|Egress Provider| Providers[Twilio, Resend, Meta APIs]
  ```

  ### Relational Entity-Relationship Diagram
  ```mermaid
  erDiagram
      chat_inboxes {
          uuid id PK
          uuid tenant_id
          text name
          timestamptz created_at
      }
      chat_channels {
          uuid id PK
          uuid tenant_id
          uuid inbox_id FK
          text channel_type
          jsonb config
          timestamptz created_at
      }
      chat_contacts {
          uuid id PK
          uuid tenant_id
          text name
          text email
          text phone
          timestamptz created_at
      }
      chat_conversations {
          uuid id PK
          uuid tenant_id
          uuid inbox_id FK
          uuid contact_id FK
          uuid assignee_id
          text status
          timestamptz created_at
      }
      chat_messages {
          uuid id PK
          uuid tenant_id
          uuid conversation_id FK
          text sender_type
          uuid sender_id
          text content
          timestamptz created_at
      }
      chat_inboxes ||--o{ chat_channels : "configures"
      chat_inboxes ||--o{ chat_conversations : "contains"
      chat_contacts ||--o{ chat_conversations : "initiates"
      chat_conversations ||--o{ chat_messages : "comprises"
  ```

  ---

  ## Mobile-First UX Flow (375px Viewports)

  ### Screen Flow
  1. **Omni Inbox Console (Unified Queue)**
     - Displays urgent incoming messages across all connected channels.
     - Top bar provides quick channel filters (e.g., WhatsApp, Web, SMS) with badge counters for unresolved leads.
     - Large touch targets (>= 44x44px) to easily swipe or tap messages.
  2. **Conversation Thread (Interactive Work Area)**
     - Restrained, premium macOS-style translucent glass backing (`backdrop-blur-[30px] saturate-[210%] bg-white/60 dark:bg-black/20`).
     - Split panels: the top 65% is the chronological timeline (including customer messages, automated AI translations, draft responses, and private team notes).
     - The bottom 35% is a sticky, responsive composer bar featuring a single-tap "Approve & Send Draft" button, a "Manual Reply" input field, and a "✨ Draft Quote with AI" option.
  3. **Unified Customer Memory Sheet**
     - Sliding drawer triggered by tapping the customer avatar.
     - Summarizes active segments, lifetime bookings, payment preferences, and notes parsed by the AI department.
     - "Grandmother Test" design: absolute clarity, zero engineering terminology (e.g., "Kubernetes", "Database", "REST APIs" are completely hidden).

  ### Layout Layout Details (375px Mobile Parity)
  - No horizontal scrolling; elastic viewport-width wrapping.
  - Form fields automatically activate native mobile keyboards (e.g., `type="tel"` for phone setup).
  - High-contrast visual status tokens (green for AI Handled, yellow for Awaiting Approval, red for Failed Egress).

  ---

  ## AI Agent Department Integration
  Our native Rust chat engine relies on the **Operations and CS AI Departments** to handle automated background triage:
  1. **Triage Agent**: Listens to database transaction events (transactional outbox) on new customer messages, analyzes sentiment and intent, and classifies urgency.
  2. **Translation & Context Agent**: Detects language barriers, translates the incoming content to the owner's language, and pulls relevant context from the customer memory graph.
  3. **Drafting Agent**: Auto-generates a context-aware proposed reply or draft quote, depositing it directly into the `chat_messages` or `agent_draft` model for operator approval.
  4. **Human Takeover Circuit Breaker**: If an operator manually starts typing or sends a message, an automated transactional database flag (`automation_fence`) is incremented, immediately suspending all automated background actions for that conversation thread.

  ---

  ## Implementation Prompt (Dispatch to Swarm)

  ### Objective
  Implement the backend core of the native, multi-tenant OHC Omnichannel Chat Platform in Rust, providing seamless database-backed persistence, robust webhook signature verification, and a unified API endpoint layer.

  ### Critical User Journey (CUJ)
  - **Step 1 (Ingestion):** An external customer sends a message over Web Widget / WhatsApp. The incoming payload hit our secure endpoint. The connector verifies the signature, resolves the `tenant_id`, deduplicates the payload, and commits it to `chat_messages` inside a single PostgreSQL database transaction.
  - **Step 2 (Real-Time Push):** The committed message triggers a WebSocket payload push to all operator subscribers authenticated with a single-use ticket.
  - **Step 3 (AI Drafting):** The background job worker processes the transactional outbox, triggers the CS AI Agent, and appends a `draft_reply` to the database.
  - **Step 4 (Operator Action):** The operator opens the `/inbox` on their mobile phone, views the translated customer message and the AI draft reply, and taps the single-tap "Approve & Send" button, pushing the response through the provider.

  ### Acceptance Criteria
  1. **Zero Chatwoot Footprint:** No references to Chatwoot in `Cargo.toml`, deployment scripts, HPA configurations, or Helm charts. Must pass `deploy/tests/no_chatwoot_residue_test.sh` cleanly.
  2. **Multi-Tenant RLS Enforcement:** All database writes and queries targeting chat tables must verify that `tenant_id` matches the active postgres session claim. Include SQL integration tests.
  3. **Signature Verification:** Replay-protected, fail-closed cryptographic signature verification implemented for Resend (email), Twilio, and Meta webhooks. Wildcard signature bypass is forbidden.
  4. **Real-time Gateway Safety:** WebSocket connection upgrade requires a valid, short-lived, single-use ticket containing audience-bound scopes. Reused tickets must be immediately rejected.
  5. **100% Test Coverage:** All newly created Rust modules, service endpoints, and webhook parsers must achieve 100% unit and integration test coverage. All Bazel tests must pass cleanly (`bazel test //...`).

  ---

  ## Project Execution Metadata
  - **Priority:** P0
  - **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
