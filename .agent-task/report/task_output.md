issue_title: "Architecting OHC-Native Omnichannel Chat and Real-Time Routing Engine"
issue_description: |
  # Native Omnichannel Support Platform & Real-Time Sync Engine

  ## 1. Problem Statement

  For independent operators like Maya (home baker) and Carlos (handyman), managing customer conversations across Instagram DMs, SMS, WhatsApp, and Web Chat is an operational nightmare. Currently, they must constantly context-switch between multiple standalone native apps, losing thread histories, order records, and customer details along the way. Existing third-party customer support software (like Zendesk, Intercom, or the retired Chatwoot integration) is over-engineered, jargon-heavy, and built for massive, siloed support departments—not busy business owners.

  They need OHC to deliver a premium, single unified inbox experience that automatically routes customer inquiries from any channel directly to their phone or desktop. This system must proactively triage incoming messages, let AI agents safely draft replies or generate order quotes with one-tap operator approval, maintain a unified "Customer Memory" context card, and handle real-time sync seamlessly—even over unreliable cellular networks—without requiring any technical setup.

  ---

  ## 2. Research Report: Omnichannel Messaging & Sync Engine Benchmarking

  ### A. Competitor Architecture Comparison
  To build an industry-leading native support system, we benchmarked our OHC native design against Chatwoot, Shopify Inbox, Wix Chat, Squarespace, and GoDaddy Conversations:

  | Platform / Capability | Chatwoot (Source Audit) | Shopify Inbox | Wix Chat | Squarespace | OHC Native Inbox (Proposed) |
  |---|---|---|---|---|---|
  | **Core Architecture** | Ruby on Rails + Sidekiq, Postgres, Redis WebSockets | Proprietary backend + Twilio/Meta adapters | Proprietary + App-level widgets | Form integrations + basic email threads | **Native Rust + PostgreSQL/SQLite + WebSockets** |
  | **Mobile-First Real-time Sync** | Standard REST/WebSockets (prone to disconnects) | Native mobile push + REST | App-based polling / WebSockets | Email-centric (no live chat sync) | **HttpOnly single-use WebSocket tickets + PowerSync convergence** |
  | **AI Integration Model** | External bot integrations / static triggers | Shopify Sidekick auto-drafts | Basic template replies | Static auto-responders | **AI Department coordination (CS, Ops, Finance) + 1-tap approval** |
  | **Multi-Tenancy Guard** | Database tenant IDs (soft-enforced) | Tenant isolation by store | Application isolation | Standard tenant DB separation | **Row-level database security (Postgres RLS) + SQLite tenant predicates** |
  | **Offline Tolerance** | Read-only in-app, writes fail | Connection required to reply | Connection required | No offline mobile capability | **Local-first SQLite synchronization, queueing outbound jobs** |

  ### B. Deep-Dive Findings on Real-Time Sync & Queueing
  1. **Omnichannel Ingestion & Signature Verification**: Competitors like Chatwoot parse raw webhook bodies and compare HMAC-SHA256 signatures for Meta/WhatsApp and Twilio. If body parsers are lossy or normalize JSON keys, signature verification fails. OHC must enforce raw payload verification before any schema parsing.
  2. **The Delivery Outbox Pattern**: When an operator presses "Send", most tools make an blocking API call directly to Twilio or Meta. If the connection fails, the write is lost or hangs. Standard best practice dictates a transactional outbox: save the message and queue a background delivery job in the same atomic database transaction.
  3. **Real-time Push & Connection Recovery**: To prevent message loss when Carlos is driving through low-signal zones, OHC utilizes single-use, audience-bound WebSocket tickets (`aud=ohc-realtime`) combined with PowerSync's offline-first synchronization. When connection is restored, a deterministic sequence gap reconciliation triggers instead of relying on fragile state push.

  ---

  ## 3. High-Level Design Doc

  ### A. System Architecture & Component Interactions
  The native OHC omnichannel platform unifies third-party webhooks, our Rust microservices, PostgreSQL (cloud storage with row-level security), SQLite (local-first Tauri storage), and WebSockets for ephemeral states.

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      actor Operator
      participant Webhook as Webhook Ingestion (Rust)
      participant Core as Omnichannel Domain (Rust)
      participant DB as Database (Postgres RLS / SQLite)
      participant WS as Real-Time WebSocket Gateway
      participant UI as Operator UI (Next.js / Tauri)

      Customer->>Webhook: Inbound Message (WhatsApp/DMs)
      activate Webhook
      Webhook->>Webhook: Fail-closed Signature Verification & Replay Check
      Webhook->>Core: Normalize Message (ReceiveMessage Command)
      deactivate Webhook
      activate Core
      Core->>DB: Start DB Transaction
      DB->>DB: Resolve Channel & Tenant Context
      DB->>DB: Persist Message, Contact, & Conversation Sequence
      DB->>DB: Enqueue Outbox AI Triage Job
      Core->>DB: Commit Transaction
      deactivate Core

      DB->>WS: Emit Change Event (Transaction Log)
      WS->>UI: Ephemeral & Durable Push (Sync/PowerSync)
      UI->>Operator: Real-time UI Update (New Message + AI Draft Badge)

      Operator->>UI: Click "Approve Draft & Send"
      UI->>Core: Action: ApproveDraft(MessageId)
      activate Core
      Core->>DB: Update Outbox: Create Outbound Delivery Job (Queued)
      Core->>DB: Update Conversation State (Pending -> Sent)
      deactivate Core
      DB->>WS: State Change
      WS->>UI: Sync Status: "Accepted by provider"
  ```

  ### B. Entity-Relationship Data Model (Mermaid.js)
  ```mermaid
  erDiagram
      INBOX {
          uuid id PK
          varchar tenant_id
          varchar name
          varchar policy_boundary
      }
      CHANNEL_CONNECTION {
          uuid id PK
          varchar tenant_id
          uuid inbox_id FK
          varchar provider_type
          json encrypted_credentials
      }
      CONTACT {
          uuid id PK
          varchar tenant_id
          varchar display_name
          json attributes
      }
      CONTACT_IDENTITY {
          uuid id PK
          varchar tenant_id
          uuid contact_id FK
          varchar provider_type
          varchar provider_identity_key
      }
      CONVERSATION {
          uuid id PK
          varchar tenant_id
          uuid inbox_id FK
          uuid contact_id FK
          varchar status
          int seq_number
          timestamp last_message_at
      }
      MESSAGE {
          uuid id PK
          varchar tenant_id
          uuid conversation_id FK
          varchar sender_type
          text content
          varchar status
          timestamp created_at
      }
      OUTBOX_JOB {
          uuid id PK
          varchar tenant_id
          uuid message_id FK
          varchar state
          int retry_count
          timestamp lease_expires_at
      }

      INBOX ||--o{ CHANNEL_CONNECTION : "manages"
      INBOX ||--o{ CONVERSATION : "routes"
      CONTACT ||--o{ CONTACT_IDENTITY : "resolves"
      CONTACT ||--o{ CONVERSATION : "initiates"
      CONVERSATION ||--o{ MESSAGE : "contains"
      MESSAGE ||--o| OUTBOX_JOB : "triggers"
  ```

  ### C. UI Wireframes & Screen Flows (Mobile-First Viewport: 375px)

  We establish a strict premium, glassmorphism-styled mobile workspace. Documented touch targets are at least 44x44px.

  ```
  +---------------------------------------+
  |  OHC Unified Inbox [💡 AI Enabled]    |  <- 44px Header, Translucent Glass
  +---------------------------------------+
  |  Filter: [All]  [Unread Leads (2)]    |  <- Filter Pill Switches
  +---------------------------------------+
  |  MESSAGE QUEUE (375px Focus-First)    |
  |  +---------------------------------+  |
  |  | Maya's Cakes (Insta DM)  [Warn]  |  |  <- Touch target: 56px height
  |  | "Do you make vegan cakes?"      |  |
  |  +---------------------------------+  |
  |  | Carlos Repairs (WhatsApp) [Good] |  |  <- Auto-replied by AI
  |  | "Estimate for drywall fix"     |  |
  |  +---------------------------------+  |
  +---------------------------------------+
  |  CONVERSATION DETAIL (Maya's Cakes)   |
  |  +---------------------------------+  |
  |  | Customer: Do you do vegan cakes? |  |  <- Gray Bubble, 14px Text
  |  +---------------------------------+  |
  |  | ✨ AI Draft (Pending Approval)   |  |  <- Purple Glow Border
  |  | "Yes Maya! We offer vegan choc  |  |
  |  | cake. Click to approve quote."  |  |
  |  +---------------------------------+  |
  +---------------------------------------+
  |  [ Approve Draft ] [ Edit Reply ]     |  <- Primary Actions: Min 44x44px
  +---------------------------------------+
  ```

  ### D. AI Agent Coordination & Autonomy
  - **The Customer Success Department**: Automatically triggers on incoming customer messages, checks contextual customer memory, and drafts accurate responses referencing tenant knowledge (e.g., pricing, return policies, menu configurations).
  - **The Operations Department**: Monitors incoming booking requests, cross-references calendar availability in SQLite/Postgres, drafts schedule proposal options, and locks them temporarily in a "Pending Tenant Approval" state.
  - **Human Overrides**: Once an operator clicks `Edit Reply` or drafts a manual response, the system triggers a **"Human Fencing Check"** that immediately increments the conversation version, suspending AI automated responders on that thread to prevent collision or double-response.

  ### E. Key Design Decisions
  1. **Strict Multi-Tenancy**: OHC-Native chat tables strictly enforce `tenant_id` row-level security (RLS) policies on Postgres. Under no circumstances are cross-tenant queries authorized.
  2. **Truthful UI Indicators**: The operator dashboard must label the state of messages exactly. When a reply is queued, it displays "Queued". Only when Twilio/Meta webhooks deliver a `delivered` or `read` status does the UI transit to "Delivered" or "Read".
  3. **No Decorative Placeholders**: In accordance with OHC Core Values, zero mock data is permitted in production views. Empty states are explicitly labeled "No conversation history found for this inbox."

  ---

  ## 4. Implementation Prompt (For the Implementer Agent)

  ### User-Facing Outcome
  Deliver an OHC-Native Omnichannel Inbox workspace where operators can securely manage conversations across channels in real-time, approve AI-drafted replies, see unified customer memory summary cards, attach media securely, and send manual replies. The application must be 100% usable on mobile screens (375px) without horizontal scrolling, utilizing Apple/Ubiquiti translucent glass tokens.

  ### Critical User Journey (CUJ) to Implement
  1. **Log in** to OHC and navigate directly to the `/inbox` route.
  2. **View the Message Queue** on a 375px simulated mobile layout. Confirm that the items are clear, categorized by priority status ("warn", "good", "bad"), and have touch targets of at least 44x44px.
  3. **Select an open conversation** (e.g., Maya's Cake inquiry). The main pane loads:
     - The customer's message.
     - The **Unified Customer Memory Context Card** at the top, showing total interactions, customer segments, and preference notes.
     - The **AI-generated Draft Reply** inside a clean card with a subtle purple-glow border.
  4. **Approve and Send the Draft**: Click the "✨ Approve & Send Draft" button. The UI immediately shows a "Sending reply..." loader, then transitions to "Draft approved and sent."
  5. **Draft Quote with AI fallback**: If no active draft exists, the operator clicks "✨ Draft Quote with AI", which routes them to the interactive quote generator with pre-filled context.
  6. **Manual Reply Override**: Type a custom message in the manual reply textbox, click "Attach Photo", select an image, and click "Send Reply". The interface updates in real-time.

  ### Acceptance Criteria
  - **Backend Stability**: Build a robust, schema-validated Rust repository interface for `Message`, `Conversation`, `Inbox`, and `Contact` with PostgreSQL and SQLite support. Pass `bazel test //...` with 100% coverage on new code.
  - **No Mock Data**: Ensure that conversation items, drafts, customer preferences, and interaction summaries are retrieved dynamically from the real application database—zero hardcoded placeholders.
  - **Web Security Hardening**: Every endpoint must authenticate HttpOnly sessions, verify tenant isolation using RLS constraints, and perform Raw Signature checks on incoming mock provider webhooks.
  - **Verification Standard**: Deliver at least 5 comprehensive Playwright E2E tests executing the exact journey from Home -> Login -> Inbox -> Approve Draft -> Manual Override -> Verify Database Persistence.

  ---

  ## 5. Metadata
  - **Priority**: P1 (High)
  - **Estimated Scope**: Large
  - **Issue Category**: research
  - **Issue Type**: task
  - **Issue Label**: [agent-report]
  - **Assignees**: []
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
