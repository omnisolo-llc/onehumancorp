issue_title: "Implement ActionRequiredQueue and Agent Feed Mobile UX for Omnichannel Inbox"
issue_description: |
  # Research Report: Omnichannel Unified Inbox & ActionRequiredQueue Architecture

  ## 1. Track 1: Architectural Gap & Scaling Discovery
  ### Codebase & Docs Audit
  Based on our repository audit, the backend infrastructure for omnichannel message ingestion is partially built. We have `omnichannel_repo.rs` and `omnichannel_service.rs` which successfully ingest external signals (e.g., Instagram DMs, WhatsApp messages), resolve `CustomerProfile` records, create `WorkItem` entries, and utilize an LLM (Gemini/MiniMax) to generate an `AgentDraft`.

  However, a crucial architectural gap exists in the **Presentation and Action Layer**:
  1. The generated `agent_draft` records are stored but never surfaced to the user.
  2. There is no formalized `ActionRequiredQueue` service or API to paginate, filter, and serve these drafts.
  3. The Mobile-First (375px) UX component for the "Agent Feed" is entirely missing from the frontend repository.

  ### Competitor Systems Audit
  Leading platforms like Shopify and Wix offer "Unified Inboxes," but they primarily function as traditional chat aggregators requiring manual owner response. Shopify Inbox, for instance, requires the user to type out replies or rely on rigid FAQ auto-responders.

  ### The OHC Gap
  OHC's vision is "Invisible AI Automation." The missing capability is the **Proactive Action Feed**. Instead of reading an inbox and typing a reply, the owner/operator should open their OHC app to an `ActionRequiredQueue` consisting of "Action Cards," where the AI has already drafted the contextual reply. The owner merely taps "Approve" to send.

  ## 2. Track 2: Selected Architecture Deep Dive (System Design)
  ### Business Journey Mapping (Persona: Maya the Baker)
  - **Acquisition/Revenue**: A customer DMs Maya on Instagram asking if she has vegan cakes.
  - **Automation**: The Omnichannel Gateway ingests the webhook, resolves the customer, checks inventory, and The Ambassador agent drafts a reply: "Yes! We have 3 vegan cakes left. Would you like me to reserve one?"
  - **Activation/Frictionless Action**: Maya receives a push notification, opens the OHC app (375px view), sees the draft card in her feed, and taps "Approve."
  - **Outcome**: The message is dispatched without Maya typing a single word, saving her 5 minutes per inquiry and capturing the sale instantly.

  ### Data Model & Invariants
  - **PostgreSQL Ledger (Existing)**: `customer_profile`, `work_item`, `agent_draft`.
  - **ActionRequiredQueue (New Module)**: A new API layer (`src/server/api/inbox/`) must be built to query the database and serve `AgentDraft` objects bundled with their parent `WorkItem` and `CustomerProfile` context.
  - **Invariants & Multi-Tenancy**: All queries to the queue must strictly filter by the authenticated `tenant_id` to ensure absolute row-level data isolation. The `ActionRequiredQueue` must employ `SKIP LOCKED` if implementing multiple background workers to avoid race conditions.

  ### AI Department Coordination
  - **The Ambassador (Customer Success)**: Generates the draft based on the customer graph.
  - **The Manager (Operations)**: Provides real-time inventory context to The Ambassador during the draft generation phase.

  ## 3. Track 3: Technical Integrity & Mobile-First Review
  ### Mobile-First UX Flow
  The frontend implementation must target a 375px viewport (Mobile First):
  - **The Feed**: A vertical scroll of translucent glassmorphism cards (`background: rgba(255, 255, 255, 0.65)`, `backdrop-filter: blur(30px)`).
  - **The Card**:
    - **Header**: Customer Name, Channel Icon (e.g., Instagram), and a timestamp.
    - **Body**: The AI-drafted reply text.
    - **Actions**: Two full-width, touch-friendly buttons (min 44x44px): Primary "Approve & Send" (Blue), Secondary "Edit Draft" (Gray).

  ### Performance & Zero Trust
  - **Latency**: The feed endpoint must respond in < 200ms. All heavy AI generation occurs asynchronously upon ingestion, not at read-time.
  - **Isolation**: SPIFFE/SPIRE identity guarantees the user can only fetch their tenant's queue.

  ## 4. Track 4: Strategic Feature Issue Dispatch (Implementation Prompt)

  **Implementation Prompt for Engineering Swarm:**

  **User-Facing Outcome**: As an owner (like Maya), when a customer messages my business, I see a card in my mobile feed with an AI-written reply. I can tap "Approve" to send it instantly.

  **Critical User Journey & Acceptance Criteria**:
  1. **Backend Integration**: Implement a new REST/gRPC endpoint in the Go backend (`ActionRequiredQueue`) that fetches all pending `agent_draft` records, joining them with `work_item` and `customer_profile` data, filtered by `tenant_id`.
  2. **Approval Endpoint**: Implement an endpoint to accept an approval mutation (`POST /api/inbox/draft/{id}/approve`). This should update the draft status and trigger the dispatch logic back to the social channel (mock the external network call for now if necessary).
  3. **Frontend UI**: Build the Mobile-First (375px) "Agent Feed" UI using the OHC Premium Token library (glassmorphism cards, 44x44px touch targets).
  4. **Automated Verification**: Implement Playwright E2E tests validating that an authenticated user can view a seeded draft card in the UI, click "Approve", and see the card disappear from the queue, with the backend state correctly updated.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
