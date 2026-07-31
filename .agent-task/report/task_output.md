issue_title: "Architecture & Strategic Dispatch: Native Omnichannel Chat System (Chatwoot Replication)"
issue_priority: "P2"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []
issue_description: |
  # Native Omnichannel Chat and Customer Support System (Chatwoot Replication)

  ## 1. Executive Summary & Persona Focus

  This research and architectural design document establishes the blueprint for **OneHumanCorp's (OHC) Native Omnichannel Chat and Customer Support System**, successfully replacing the retired third-party Chatwoot dependency. In alignment with our non-technical owner-operator personas, this engine consolidates customer relationship triaging, messaging, and automation directly inside OHC's high-performance Rust backend.

  Every design choice here addresses real owner friction:
  - **Maya (Home Baker)**: Can wake up to find her Instagram DMs automatically triaged, with custom cake options auto-drafted by OHC’s AI agents using her actual catalog prices, without her ever touching setup details.
  - **Carlos (Field Service Owner)**: Receives service requests directly from his mobile inbox, instantly drafts quotes, and schedules bookings while on-site, entirely through a responsive 375px Android viewport.
  - **Fatima (Food Cart Operator)**: Uses the offline-tolerant widget and compact pre-order interface that synchronizes seamlessly over low-bandwidth mobile connections.
  - **Nora (Agency Principal)**: Orchestrates client project approvals, task tracking, and proposals through a single cohesive, unified workspace thread.

  ---

  ## 2. Track 1: Architectural Gap & Benchmarking (Chatwoot Audit)

  ### 2.1 Chatwoot Source Code Audit
  An audit of Chatwoot's core architecture (`https://github.com/chatwoot/chatwoot`) reveals a dense Rails/Sidekiq footprint:
  1. **Omnichannel Ingestion**: Inbound webhooks (Twilio, Facebook, Instagram) are normalized by specific Rails controllers, matching them against `ContactInboxes` and creating a thread.
  2. **WebSocket & PubSub**: Redis PubSub distributes realtime state to agents (typing indicators, presence, message delivery states) using ActionCable.
  3. **Multi-Tenancy**: Scoped via `account_id` throughout the ActiveRecord model layers, relying on Rails application-level hooks.
  4. **AI/Automation Hooks**: Chatwoot hooks triggers and webhooks outward to external AI services, resulting in high latency and loose integration state.

  ### 2.2 Why OHC Native Rust Chat Succeeds
  Replacing Chatwoot with a native Rust implementation inside OHC resolves major gaps:
  - **Unification of Data Paths**: OHC currently suffers from three overlapping, unaligned tables (`inbox_messages`, `omni_inbox_messages`, and `unified_*`). Consolidating this into a single high-performance Rust domain allows bulletproof atomicity.
  - **Fail-Safe Tenant Isolation**: Instead of application-layer filtering, OHC leverages PostgreSQL **Row-Level Security (RLS)** using `tenant_id` on every table, combined with SQLite parameterized predicates on desktop, protecting the business owner's customers from leaks.
  - **Offline-First Local Sync**: PowerSync convergence is embedded natively, allowing Carlos or Fatima to check messages, draft replies, and view customer records with zero network connectivity, syncing instantly upon reconnect.

  ---

  ## 3. Track 2: Core Architecture & Systems Design

  ### 3.1 Business Journey Mapping
  The engine supports the customer lifecycle natively:
  1. **Acquisition**: A casual customer lands on Maya’s web page. The lightweight native customer widget loads instantly (origin-checked and sandboxed).
  2. **Onboarding/Activation**: The widget uses a secure anonymous-to-authenticated capability transfer to preserve conversation context when the client logs in or submits an order.
  3. **Retention**: Outbound delivery uses a transactional outbox. If Carlos sends an estimate, the platform guarantees exact-once delivery, keeping the state as `Accepted by provider` until receipts verify `Sent`, `Delivered`, and `Read`.

  ### 3.2 Data Model & Multi-Tenant Invariants
  - **Inbox**: Scoped by `tenant_id`. Routing boundary for incoming channels.
  - **Conversation**: Belongs to `Inbox`. Tracks assignment, SLA, status, and AI policy fence.
  - **Message**: Sequential conversation elements. Never stores Base64 media directly (always short-lived signed object references).

  ```mermaid
  erDiagram
      INBOX {
          uuid id PK
          string tenant_id FK
          string name
          string channel_type
      }
      CONTACT {
          uuid id PK
          string tenant_id FK
          string name
          string email
          string phone
      }
      CONVERSATION {
          uuid id PK
          uuid inbox_id FK
          uuid contact_id FK
          string status
          int seq_num
          timestamp sla_deadline
      }
      MESSAGE {
          uuid id PK
          uuid conversation_id FK
          string sender_type
          text content
          string delivery_state
          timestamp created_at
      }
      RECEIPT {
          uuid id PK
          uuid message_id FK
          string status
          timestamp updated_at
      }
      INBOX ||--o{ CONVERSATION : "contains"
      CONTACT ||--o{ CONVERSATION : "initiates"
      CONVERSATION ||--o{ MESSAGE : "appends"
      MESSAGE ||--o{ RECEIPT : "tracks"
  ```

  ### 3.3 Real-Time Message Routing Flow
  This sequence diagram outlines safe inbound ingestion and immediate real-time broadcast:

  ```mermaid
  sequenceDiagram
      autonumber
      participant Provider as Meta/Twilio Webhook
      participant API as OHC Ingress Endpoint
      participant DB as Postgres (RLS Active)
      participant Outbox as Transactional Outbox
      participant WS as Realtime WebSocket Service
      participant Client as Operator App (375px)

      Provider->>API: POST /api/v1/inbox/webhook (Signed Payload)
      Note over API: Verify Ingress Signature (Fail-Closed)
      API->>DB: Begin Tenant Isolation Transaction
      DB->>DB: Resolve Channel & Tenant Context
      DB->>DB: Upsert Contact & Conversation
      DB->>DB: Append Message (Sequence + Monotonic Lock)
      DB->>Outbox: Enqueue Transactional Outbox Event
      API-->>Provider: 200 OK (Idempotent Acknowledgment)
      Note over Outbox: Outbox Worker Polling (SKIP LOCKED)
      Outbox->>WS: Broadcast Event to WebSocket Group
      WS->>Client: Send Ephemeral Message Payload
      Note over Client: Reconcile Sequence gaps automatically
  ```

  ### 3.4 AI Department Coordination & Fencing
  AI CS agents handle routine questions (e.g. "Do you have vegan cakes?"). When Maya touches the input field, her typing triggers a **Human Takeover** event:
  - This increments the **Automation Fence Version** on the conversation table.
  - Any background AI task must re-verify this fence version before executing an irreversible outbound webhook. If the fence has shifted, the AI task is instantly cancelled.

  ---

  ## 4. Track 3: Mobile UX & Operational Integrity

  ### 4.1 Premium 375px Mobile UI Layout
  Adhering to our design system:
  - **Materials**: REST-based Translucent Glass overlays (`backdrop-blur-[30px] saturate-[210%] bg-white/60 dark:bg-black/40 border border-white/40`) with restraint status dots.
  - **Focus-First Composition**: On a 375px screen, the sidebar is collapsable into a bottom-sheet. The single operator canvas switches smoothly from conversation stream to customer intelligence details without layout shift or double scrolling.
  - **Touch Targets**: All control buttons (Review Draft, Approve as-is, Send Quote) are exactly `44x44px` minimum.

  ### 4.2 Secure Realtime Ticket Handshake
  To prevent raw bearer token exposure to frontend Javascript, OHC implements a Ticket Handshake:
  1. Next.js server resolves session -> Calls Rust backend to generate a 60-second, single-use, cryptographically signed Ticket (`jti` bound).
  2. The browser initiates WebSocket connection supplying this Ticket in the subprotocol header.
  3. Rust backend validates, consumes the Ticket (preventing replay), and authorizes the socket strictly for that tenant and inbox.

  ---

  ## 5. Track 4: Strategic Feature Issue Dispatch

  ### Project A: Database Schema & Tenant-Safe Repository (P0, Medium)
  - **Outcome**: Consolidated persistence schema for native omnichannel chat with Postgres RLS and SQLite desktop matching predicates.
  - **Persona Journey**: Carlos opens his local app and sees only his customers’ records. The underlying SQLite data matches Postgres exactly.
  - **Acceptance Criteria**:
    - Clean migrations establishing the tables `inboxes`, `contacts`, `conversations`, `messages`, and `receipts` with explicit unique and sequence constraints.
    - Active Postgres RLS with cross-tenant IDOR denial integration tests.
    - Deterministic backfill compatibility writer supporting legacy `omni_inbox_messages` data during switchover.

  ### Project B: Inbound Ingestion Webhooks & Fail-Closed Signature Verification (P1, Medium)
  - **Outcome**: Raw ingress webhook handlers verifying Meta (Instagram/WhatsApp), Twilio (SMS/Voice), and Resend (Email) signatures.
  - **Persona Journey**: Maya gets three messages on Instagram. OHC verifiers block malicious, non-signed payloads, only appending valid messages.
  - **Acceptance Criteria**:
    - Raw request signature verification utilizing exact provider-specified cryptographic inputs without lossy reparsing.
    - Idempotent transaction parsing provider event IDs to avoid duplicate appends.
    - Reject or fail closed when secrets are missing or unconfigured. No development bypasses.

  ### Project C: Real-Time Event Bus & Single-Use Ticket Handshake (P1, Large)
  - **Outcome**: Authenticated real-time event distribution and PowerSync synchronization.
  - **Persona Journey**: Priya updates inventory on desktop; her mobile phone instantly displays the synchronized state without lag.
  - **Acceptance Criteria**:
    - Backend API route `/api/v1/auth/realtime-ticket` issuing single-use, 60s expiring JWTs with a dedicated rotating key.
    - WebSocket upgrade protocol validating the ticket in subprotocol headers and rejecting reuse.
    - PowerSync sync rules filtering tables dynamically by tenant and inbox claims.

  ### Project D: Bounded AI Autonomy & Automation Fencing (P1, Medium)
  - **Outcome**: Secure, policy-bounded AI responder with human-takeover safety.
  - **Persona Journey**: Maya is typing a DM to a client. The OHC AI agent instantly halts its draft generation to prevent double-replies or confusing text.
  - **Acceptance Criteria**:
    - Atomic transaction that increments conversation `automation_fence_version` on operator keypress.
    - Bounded AI workers that recheck this fence immediately before sending.
    - Token, message-count, and retry budgets per conversation to prevent cascading automated loops.

  ### Project E: Focus-First Premium Mobile UI (375px) & Sandbox Widget (P2, Large)
  - **Outcome**: Translucent glass mobile UI layouts and cross-origin-safe customer widget.
  - **Persona Journey**: Carlos reads customer notes comfortably on his phone. A customer uses the embed widget on Carlos's site to send a message.
  - **Acceptance Criteria**:
    - Responsive 375px layout utilizing translucent glass components (`backdrop-blur-[30px] saturate-[210%] bg-white/60 dark:bg-black/40 border border-white/40`) with 44x44px touch areas.
    - Sandboxed iframe customer widget enforcing a tenant-specific `frame-ancestors` Content-Security-Policy.
    - Handshake handshake using `postMessage` with verified origin and freshly generated nonces on both sides.
