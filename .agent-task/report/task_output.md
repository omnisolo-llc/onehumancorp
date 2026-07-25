issue_title: "Architecture Design: Native Rust Omnichannel Chat System & Chatwoot Source Audit"
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_description: |
  # OHC Native Rust Omnichannel Chat System: Strategic Architecture & Chatwoot Benchmarking Report

  ## 1. Executive Summary & Context (Mission Queue Protocol Brief)
  The OneHumanCorp (OHC) platform is transitioning its conversational layer from a third-party Ruby on Rails-based service (Chatwoot, now 100% retired) to an OHC-native, high-performance, and deeply integrated **Rust-native Omnichannel Chat Engine**.

  By embedding this engine directly within OHC's core service layers, we eliminate external dependencies, reduce infrastructure overhead, ensure absolute data sovereignty, and leverage our first-class AI teammate architecture.

  This architectural design doc establishes the blueprints for a tenant-isolated, offline-tolerant, and AI-first support platform. It serves as the primary guidance for the implementation swarm.

  ---

  ## 2. Market Research & Competitor Benchmarking

  ### 2.1 Competitor Systems Audit
  - **Shopify Inbox / Wix Inbox / GoDaddy / Squarespace:** Traditional SMB builders offer communication panels as generic, reactive add-ons. They aggregate live chat and social messages but rely entirely on manual agent entry or simplistic, static macro auto-responses. These tools are administrative burdens rather than active work-assistance partners.
  - **Zendesk / Intercom / Salesforce Service Cloud:** Enterprise platforms support complex routing and omnichannel workflows but are prohibitively expensive, overly technical, and require dedicated operational administrators. They fail the "grandmother test" for micro-operators.
  - **The OHC Unfair Advantage:** OHC merges commerce, operations, and communication into an **AI-First / Assistant-First Shell**. The assistant doesn't just display a message; it acts as an invisible teammate (The Ambassador) that reads context, queries inventory/calendars, calculates pricing, drafts responses, and places 1-tap actionable cards directly in the owner's mobile feed.

  ### 2.2 Chatwoot Source Code Audit & Feature Benchmarking
  An audit of the Chatwoot Ruby on Rails codebase reveals a standard Web 2.0 Ruby-on-Rails architecture:
  - **Data Models:**
    - `Account`: Standard tenant/workspace.
    - `Inbox` / `Channel`: Polymorphic associations mapping message streams to provider-specific configuration.
    - `Contact` / `ContactInbox`: Represents a customer identity and their channel-specific source identifiers (e.g., telephone number, social handles).
    - `Conversation`: The central thread tying an Account, Contact, and Inbox together, maintaining state (`status`, `priority`, and assignees).
    - `Message` / `Attachment`: Messages have a `message_type` enum (incoming, outgoing, activity, template), `content`, and polymorphic attachments mapping to direct file uploads.
  - **Controllers & API Ingress:**
    - Rails REST endpoints handle CRUD on conversations, messages, and contact details. Webhook endpoints verify signatures using provider-specific signature verification filters.
  - **Real-Time WebSocket Layer:**
    - Built on Rails **ActionCable**. Clients subscribe to channels (`AccountChannel` for agents, `RoomChannel` or `WidgetChannel` for widget customers) and exchange JSON commands representing events like message creation, updates, and ephemeral state (presence and typing indicators).

  ### 2.3 OHC Architectural Enhancements over Chatwoot
  1. **From ActiveRecord Filter to Row-Level Security (RLS):** Instead of relying on ActiveRecord application-level scope filtering (which is prone to leak queries if developers miss an account scope), OHC enforces **PostgreSQL Row-Level Security (RLS)** using `tenant_id` on every table, combined with SQLite tenant query parameters in desktop standalone mode.
  2. **From ActionCable (Ruby) to Rust-Native WebSockets & PowerSync:** Instead of Rails' single-threaded or thread-pool bounded ActionCable process, OHC utilizes a highly efficient **Rust axum/tokio WebSocket server** combined with **PowerSync** for immediate offline-first local replication and convergence.
  3. **From Add-on Chatbots to AI-First Teammates:** OHC moves from secondary bot webhooks to first-class, background-managed **AI Department Orchestration** with budget limits, strict policy boundaries, and human-takeover circuit breakers.

  ---

  ## 3. Product Vision & Persona Journeys

  Every architectural decision is grounded in our real user personas to guarantee the platform is non-technical, accessible, and high-value.

  - **Maya (Home Baker):** Receives cake custom order DMs on Instagram overnight. The native chat engine ingests the DMs, matches her customer profile, queries her custom cake catalog, crafts a draft reply with a custom Stripe deposit payment link, and schedules a delivery placeholder on her calendar. She wakes up, opens her 375px mobile feed, and taps "Approve" (1 second).
  - **Carlos (Handyman):** Works on roofs and cannot answer SMS/Twilio calls. The agent handles inbound service requests, analyzes calendar slots, coordinates route notes, drafts a quote, and synchronizes the state locally to his Android phone via PowerSync so he can read conversation history even in low-connectivity areas.
  - **Priya (Boutique Owner):** Needs real-time stock-aware offers and customer notifications. The system integrates inventory events with her conversational channels, allowing instant automated inventory reservation when customers inquiry about a product variant (size/color).

  ---

  ## 4. Bounded Context & System Architecture

  OHC's native omnichannel system consolidates three historical data models into a unified, high-integrity architecture. It isolates channels and ensures transactional atomicity using the outbox pattern.

  ### 4.1 System Topology
  ```mermaid
  graph TD
      A[Customer Web Widget / Public Clients] -->|Secure WebSocket / REST| B[OHC API Gateway - Rust Server]
      C[Instagram / Meta Webhook] -->|Verified Webhook Ingress| B
      D[WhatsApp / SMS Webhook] -->|Verified Webhook Ingress| B
      B -->|PostgreSQL RLS / SQLite| E[(Durable Multi-Tenant Store)]
      B -->|Secure SPIFFE/SPIRE mTLS| F[Orchestration Hub]
      F -->|CS Agent - The Ambassador| G[AI Teammate Department]
      F -->|Ops Agent - The Manager| G
      F -->|Finance Agent - The CFO| G
      B -->|Durable Outbox Queue| H[Outbound Dispatcher]
      H -->|Idempotent Delivery| I[External Providers - Resend / Meta / Twilio]
  ```

  ### 4.2 Entity-Relationship Diagram (ERD)
  ```mermaid
  erDiagram
      TENANT ||--o{ INBOX : "owns"
      TENANT ||--o{ CONTACT : "owns"
      INBOX ||--o{ CHANNEL_CONNECTION : "configures"
      INBOX ||--o{ CONVERSATION : "contains"
      CONTACT ||--o{ CONTACT_IDENTITY : "resolves"
      CONTACT ||--o{ CONVERSATION : "initiates"
      CONVERSATION ||--o{ MESSAGE : "records"
      CONVERSATION ||--o{ PARTICIPANT : "tracks"
      MESSAGE ||--o{ ATTACHMENT : "encloses"
      MESSAGE ||--o{ RECEIPT : "generates"
  ```

  ### 4.3 Data Entities & Invariants
  1. **`Inbox`:** Represents a routing boundary and policy definition.
     * *Invariants:* Every Inbox must belong to a single valid tenant. It specifies SLA targets, working hours, and whether AI Autonomy is active or draft-only.
  2. **`ChannelConnection`:** Encrypted third-party connection credentials.
     * *Invariants:* Credentials must be stored with envelope encryption; decrypted forms must never be returned to client-side browsers or logged in telemetry.
  3. **`Contact` & `ContactIdentity`:** Represents a customer and their polymorphic provider identities.
     * *Invariants:* A single Contact can have multiple identities (e.g., social handle, phone, email). Identities must be unique per tenant and channel provider.
  4. **`Conversation`:** A channel-aware support thread.
     * *Invariants:* Must maintain strict state transitions: `open` <-> `snoozed` <-> `resolved`. Contains sequence indicators for message ordering.
  5. **`Message`:** Inbound, outbound, private, or system events.
     * *Invariants:* Every message has a monotonically increasing sequence number per conversation. Output-draft messages remain private/uncommitted until owner approval.
  6. **`Attachment`:** Storage pointers and metadata.
     * *Invariants:* Inline Base64 data is forbidden. Must reference S3-compatible cloud storage (or local sandbox directory). Must maintain `scan_status` (quarantined, clean, infected).

  ---

  ## 5. AI Department Coordination & Autonomy

  ### 5.1 Orchestration Workflow
  The AI assistant is managed as a unified coordination of specialized departments (CS, Operations, Finance, and Legal) to automate business workflows invisibly in the background.

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      participant GW as Omnichannel Ingress Gateway
      participant AMB as CS Agent (The Ambassador)
      participant MGR as Ops Agent (The Manager)
      participant CFO as Finance Agent (The CFO)
      participant DB as Transactional Outbox
      actor Owner

      Customer->>GW: Inbound message ("Do you have vegan cake free for Saturday?")
      GW->>GW: Verify Signature & Deduplicate
      GW->>DB: Write Inbound Message Transaction
      GW->>AMB: Trigger message.created Event
      AMB->>MGR: Request Catalog & Calendar check ("Saturday, Vegan Cake")
      MGR-->>AMB: Return true (available, 3 slots left)
      AMB->>CFO: Generate payment link for $50 custom cake deposit
      CFO-->>AMB: Return checkout URL
      AMB->>AMB: Draft reply with checkout URL
      AMB->>DB: Write Outbox Draft (Requires Approval)
      DB-->>Owner: Mobile Push Notification ("Approve custom order draft")
      Owner->>GW: 1-Tap Approve Reply
      GW->>DB: Commit Draft to Sent State & Queue Outbox Delivery
      DB->>Customer: Dispatch message via Meta WhatsApp Cloud API
  ```

  ### 5.2 Mandatory Safeguards & AI Budgets
  - **Human-Takeover Fence:** When the human owner enters a conversation or edits a thread, a `human_takeover_fence` is incremented. Any background AI delivery task verifies the active fence value immediately before execution; if a mismatch is detected, the task is canceled to prevent AI from stepping on human interaction.
  - **Token and Call Budgets:** Every conversation has strict resource limits: maximum 5 LLM calls and 10,000 tokens per 24 hours. When limits are exceeded, AI autonomy is suspended and a "Budget Exhausted" card is escalated to the operator.
  - **Material Actions Gate:** Irreversible actions (refunds, cancellations, scheduling modifications) are forbidden from autonomous execution and must be rendered as "Pending Action Required" cards for explicit 1-tap owner approval.

  ---

  ## 6. Real-Time & Offline Architecture

  ### 6.1 Low-Latency WebSocket Synchronization
  - **Single-Use WebSocket Tickets:** To prevent session hijacking and ambient cookie exploits, clients request a 60-second single-use `realtime-ticket` from the axum API server. This ticket is signed with a dedicated 256-bit key, includes user and tenant claims, and is consumed atomically upon WebSocket connection upgrade.
  - **PowerSync Integration:** Offline-first synchronization uses PowerSync to maintain read-only and write-pending replicas on desktop/mobile SQLite engines. Synchronization rules must enforce strict tenant-isolation filters (`tenant_id = user_claims.tenant_id`). Client-side parameter manipulation cannot expand these scopes.

  ### 6.2 Offline Write Resolution
  - **Transactional Outbox:** Any offline message draft or action is queued in local transactional storage. Upon reconnection, queue items are dispatched with stable client-generated idempotency keys.
  - **Conflict Invariants:** Concurrent thread assignment edits resolve using an explicit document-versioning schema. For messages, appends are strictly ordered using monotonic sequence markers, preventing race-condition overwrites.

  ---

  ## 7. Premium Mobile UX Flow & Visual Design

  ### 7.1 Visual Tokens & Glassmorphic Materials
  Every screen must look and feel premium, drawing from Apple-style minimalist interfaces and Ubiquiti UniFi modular dashboard cards.
  - **Translucent Glass:** Card backgrounds use `rgba(255, 255, 255, 0.05)` coupled with `backdrop-filter: blur(16px)` and a subtle `border: 1px solid rgba(255, 255, 255, 0.1)`.
  - **Touch Targets:** All buttons, toggles, and interactions must maintain a minimum bounding box of 44x44px.
  - **Adaptive Viewport:** Standardized layout begins on a **375px baseline viewport** without horizontal scrolling or clipping.
  - **Selecting Layout Composition:** Operators can choose between a classic three-pane layout, focus-first mobile two-pane canvas, or the AI operations console.

  ---

  ## 8. Technical Security & Isolation

  - **PostgreSQL Row-Level Security (RLS):** All table definitions must be appended with:
    ```sql
    ALTER TABLE <table_name> ENABLE ROW LEVEL SECURITY;
    CREATE POLICY tenant_isolation_policy ON <table_name>
      USING (tenant_id = current_setting('app.current_tenant_id', true));
    ```
  - **Workload Identity (SPIFFE/SPIRE):** Internal gRPC communication between OHC microservices and background agents relies strictly on SPIFFE identity verification. No long-lived static API secrets are allowed.
  - **Attachment Sandboxing:** File uploads undergo content-sniffing and asynchronous malware scanning within a separate quarantine namespace. SVG, HTML, and active elements are converted to inert PDF formats or forced to download with `Content-Disposition: attachment` and `X-Content-Type-Options: nosniff`.

  ---

  ## 9. Implementation Prompt (Strategic Feature Issue Dispatch)

  **To the Implementer Swarm:**
  Your mission is to construct OHC's high-performance, native Rust Omnichannel Chat Engine to completely replace Chatwoot's legacy functionality.

  ### Critical User Journey (CUJ)
  1. A micro-SMB owner (e.g., Maya) navigates to the inbox setup and connects their WhatsApp Cloud and Instagram channels using our simplified "Zero-Key" onboarding screen.
  2. An external mock customer dispatches an inbound message query: *"Do you have availability for custom vegan orders this Saturday?"*
  3. The Ingress Gateway parses the signature, validates freshness, and atomically saves the message to the database while triggering the CS Ambassador Agent.
  4. The Ambassador Agent queries the merchant's local SQLite catalog and coordinates with the Ops and Finance agents to prepare a detailed draft response, including a checkout session link.
  5. The draft is placed in the tenant's feed. The owner opens the 375px OHC mobile interface, views the frosted glass card, taps "Approve", and the delivery worker dispatches the message idempotently through the external channel.

  ### Rigorous Acceptance Criteria
  1. **Zero Chatwoot Residue:** The implementation must completely pass `deploy/tests/no_chatwoot_residue_test.sh` with zero residue of Chatwoot packages or deployment components in active directories.
  2. **100% Passing Tests:** Build, unit tests, and Playwright integration suites must run and pass 100% green under `bazel test //...` and `pnpm vitest run`.
  3. **No Network/API Mocking in E2E:** E2E Playwright tests must run against the real axum API, real PostgreSQL with enabled RLS, and real PowerSync container setups, utilizing actual test credentials for third-party endpoints.
  4. **Multi-Tenant Denial Fixtures:** Run comprehensive cross-tenant query tests to verify that tenant A can never read or mutate conversations belonging to tenant B, throwing strict unauthorized exceptions.
  5. **Visual Hardening:** Ensure all UI interactions are verified on mobile (375px), showing premium translucent glass effects with fully functional touch controls.
