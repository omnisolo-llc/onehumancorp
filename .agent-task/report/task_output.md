issue_title: "Architecture & Strategic Dispatch: OHC Native Omnichannel Support & Chat Engine"
issue_description: |
  # Problem Statement
  Small business owners and operators like Maya the baker, Carlos the handyman, and Fatima the food cart operator interact with customers across highly fragmented channels (Instagram DMs, WhatsApp, SMS, email, and live website widgets). Manually responding to these messages leads to missed opportunities, delayed replies, and poor containment. Existing vertical and generic portals (Shopify Inbox, Wix Inbox, GoDaddy, Squarespace) aggregate messages as simple raw streams without deep business-awareness, necessitating manual typing of repetitive answers. Meanwhile, enterprise-grade helpdesks (Zendesk, Intercom, Salesforce Service Cloud) are too complex, expensive, and require administrative overhead that a solo operator cannot afford.

  OHC needs an assistant-first, native omnichannel chat engine that is premium, secure by default (SPIFFE/SPIRE-backed), isolated per tenant, and leverages coordinated AI agents (like The Ambassador) to draft replies, check calendars, and propose next actions directly in the owner's mobile work triage feed.

  ---

  # Research Report & Feature Benchmarking

  ### 1. Competitive Landscape Analysis
  - **Shopify Inbox:** Great e-commerce integration (products, carts), but relies on static automated rules, canned replies, and manual triage. It does not natively orchestrate background AI departments to draft contextualized replies based on previous user behaviors across other unconnected channels (e.g. associating an Insta handle with an email address).
  - **Wix Inbox / Squarespace / GoDaddy:** Basic feed aggregators. AI assistance is restricted to textual "tone improvement" or generic copy-generation inside the composer box, failing to act as a proactive, autonomous background agent.
  - **Chatwoot (Omnichannel Standard):** Excellent open-source engine with robust support for WhatsApp, Instagram, Facebook, Line, Twitter, and custom inbox webhooks. It utilizes standard Rails controllers, sidekiq jobs, and database tables (`conversations`, `messages`, `contacts`, `contact_inboxes`, `attachments`, `canned_responses`, `macros`, `webhooks`) to link contacts across channels via "contact_inboxes". However, it introduces complex third-party infrastructure dependencies and lacks native AI agency, multi-tenant Postgres row-level security (RLS), and a local-first offline synchronization model.

  ### 2. OHC Native Rust Parity Benchmarking
  By auditing the Chatwoot database schema (`db/schema.rb`) and architectural patterns, the following key paradigms have been replicated and enhanced for OHC's high-performance native Rust microservice:
  - **Omnichannel Identity Graph:** Replicating Chatwoot's `contacts`, `contact_inboxes`, and `source_id` structure into a native, high-performance identity resolver. Instead of disjointed records, a single customer has an omnichannel graph linking their email, phone, and Instagram handles, isolated cleanly using PostgreSQL `ENABLE ROW LEVEL SECURITY` on `tenant_id`.
  - **Delivery State & Transactional Outbox:** Unlike traditional chat systems that collapse statuses or lose track of offline writes, OHC establishes a resilient transactional outbox in Rust (`messages` + `delivery_jobs` + `receipts`). This guarantees message delivery states: `draft`, `committed`, `provider_accepted`, `sent`, `delivered`, and `read`.
  - **Webhook Verifier Matrix:** Standardizing signature verification per provider (e.g., Meta, Twilio, Resend, SendGrid) over constant-time comparisons and freshness controls to eliminate insecure development signature bypasses or fallback tenants in production.

  ---

  # Design Doc

  ### 1. Core Architecture Diagram (Mermaid.js)
  ```mermaid
  graph TD
      A[Customer Inquiries: Instagram / WhatsApp / Email / Widget] -->|Secure TLS Ingress| B(Connector & Verifier Matrix)
      B -->|Verify Signature & Normalize| C{Tenant-Aware Identity Resolution}
      C -->|Graph Query| D[(Multi-Tenant DB with Postgres RLS)]
      C -->|Commit Event| E[Transactional Outbox & Event Mesh]
      E -->|Trigger| F[The Ambassador CS Agent]
      F -->|Query Context & Policies| D
      F -->|Draft Contextual Reply| G[Action Required Queue]
      G -->|Push Notification / Real-time WS / PowerSync| H[Operator Mobile UI 375px]
      H -->|1-Tap Approve & Send| I[Outbox Delivery Worker]
      I -->|idempotent Dispatch| J[Outbound Provider Gateway]
      J -->|API Callback| A
  ```

  ### 2. UI & Mobile UX Flow (375px focus-first layout)
  - **Screen 1: Priority Queue (Work Command Center):** A clean, macOS-style Translucent Glass header displaying "Work Triage". Beautifully formatted UniFi dashboard cards. One card highlights an urgent issue: `📸 Instagram DM from Sarah (Urgent)` with the subtitle: *"Needs custom cake details for this Friday."*
  - **Screen 2: Expanded AI Operations View:** Tapping the triage card smoothly transitions to an expanded sheet.
    - **Top Half (Business Context):** Displays Sarah's purchase history (Sarah bought a Vegan Chocolate Cake on June 12th).
    - **Middle Half (Action Workspace):** Shows the proposed response drafted by The Ambassador: *"Hi Sarah! I have a spot open on Friday morning for our 8-inch Vegan Chocolate Cake ($55). Shall I secure this for you with a deposit link?"*
    - **Bottom Half (Touch Controls):** Large, highly interactive touch targets (at least 44x44px) aligned horizontally:
      - **"Approve & Send"** (Primary blue button, shows animated loading state during transaction).
      - **"Edit"** (Opens native keyboard with a responsive, glass-styled text editor).
      - **"Dismiss"** (Secondary outline button).
  - **Offline Resilience:** If the operator is offline (e.g. in Carlos's field service van), the UI clearly shows a subtle yellow warning banner. Approving actions queues them locally using SyncManager. Once online, they automatically synchronize, providing realistic pending and error states.

  ### 3. AI Agent Coordinated Integration
  - **The Ambassador (Customer Success Agent):** Monitors incoming messages via the transactional event mesh. Performs tenant-scoped semantic retrieval against the business's product catalog and Sarah's context (e.g. past orders, preferred flavors).
  - **The Manager (Operations Agent):** Runs in the background to verify real-time calendar slots and stock availability before The Ambassador commits to a proposal, ensuring the draft is accurate and actionable.
  - **Human-Takeover Fence:** If the owner manually edits the response or taps a button, a transaction-safe automation fence increment occurs, immediately suspending AI autonomous actions to avoid duplicate/overlapping conversations.

  ### 4. Key Design Decisions
  - **SPIFFE/SPIRE Secure Identity:** Trust is delegated entirely to cryptographic workload identity. All backend gRPC and database connections require authenticated peers, removing insecure API keys from application memory.
  - **No PowerSync Client-Side Broadening:** PowerSync sync rules enforce verified tenant context and team inbox permissions in PostgreSQL; client-side parameters are forbidden from widening data access boundaries.
  - **Quarantined Attachments:** Uploads are placed in a non-public namespace and undergo sniff-testing, checksum validation, and malware scanning before being made available to operators, customers, or AI RAG loops.

  ---

  # Implementation Prompt
  **User-Facing Outcome:** As an independent owner/operator (e.g., Maya the baker), when a customer messages me via Instagram or Email about an order, OHC should automatically resolve their identity, coordinate with my AI department, and draft a contextualized response. This draft is placed directly into my mobile work triage feed, allowing me to review, edit, and send it with a single tap in under 2 seconds, keeping me focused on running my business.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. Ingest an external message webhook via the native connectors, triggering signature verification against the provider-specific verifier.
  2. Resolve the client's identity across multiple channels using the Omnichannel Identity Graph under strict PostgreSQL Row-Level Security scoped by `tenant_id`.
  3. Ensure the CS Agent (The Ambassador) queries the customer history and catalog, generating a proposed message with a calculated confidence rating.
  4. Write the message and the AI draft atomically to the database, dispatching a real-time event to the mobile workspace.
  5. The mobile operator UI (375px) displays the triage feed cards in UniFi style, enabling focus-first review, editable text fields with native keyboards, and 1-tap dispatching.
  6. E2E Playwright tests must automate this entire workflow: starting from login, validating empty states, receiving webhooks, expanding cards, editing the payload, tapping "Approve", and asserting database mutations alongside simulated outbound gateway receipt transitions.

  ---

  # Strategy & Scope
  - **Priority:** P1
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
