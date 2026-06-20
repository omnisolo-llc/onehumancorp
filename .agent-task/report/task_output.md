issue_title: "[Core Architecture] AI Department Coordination & Multi-Tenant Event Mesh Design"
issue_description: |
  # Research Report: AI Department Coordination & Multi-Tenant Event Mesh Design

  ## Problem Statement
  Small business owners (e.g., Maya the baker, Nora the agency principal) need their operations, customer support, and sales handled seamlessly across disconnected channels (Instagram, email, website). Currently, if Maya receives an Instagram DM about an order change, the Customer Success Agent might reply, but it lacks the real-time architectural capability to natively trigger the Operations Agent to check the kitchen schedule, or the Finance Agent to issue a refund. OHC needs a robust, multi-tenant Event Mesh architecture that allows independent AI "Departments" (Sales, Ops, Support) to coordinate invisibly behind the scenes, ensuring the owner only sees a unified, actionable outcome rather than fragmented agent tasks.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Competitors (Shopify Sidekick, Wix AI, Zendesk):** These platforms treat AI as a single monolithic chatbot or a siloed co-pilot. They do not possess specialized, communicating AI agents. If you ask Shopify Sidekick about an order, it queries the DB; it doesn't "ask the Operations Agent" to calculate a complex fulfillment schedule based on raw materials.
  - **Industry Trends (Multi-Agent Systems - MAS):** High-scale autonomous platforms are moving towards specialized agent coordination (e.g., AutoGen, CrewAI concepts) where agents have narrow scopes (Sales vs Ops) and communicate via an event bus.
  - **OHC Opportunity:** By building a resilient Multi-Tenant Event Mesh backed by Redis/PostgreSQL (Skip Locked), OHC can launch specialized AI Departments. When a message arrives, a triage agent routes it, and relevant departmental agents (Ops, CS, Finance) publish/subscribe to coordination events. The owner (Nora or Maya) receives a synthesized "Action Required" card on their 375px mobile feed, representing the combined work of 3 AI agents.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[External Webhook: IG/Email/Payment] --> B(API Gateway / Triage Router)
      B --> C{Multi-Tenant Event Mesh - Redis Streams / PG Queue}
      C -->|Event: New Lead| D[Sales Agent Dept]
      C -->|Event: Inventory Check| E[Operations Agent Dept]
      C -->|Event: Refund Request| F[Finance Agent Dept]
      D -->|Publishes: Proposal Drafted| C
      E -->|Publishes: Inventory Held| C
      C --> G[Owner Feed Synthesizer]
      G --> H[Mobile App Feed 375px]
      H -->|Owner Approves Action| I[Execution Engine]
  ```

  ### Mobile UX Flow (375px First)
  - **The "Morning Briefing" Feed:** The owner opens the app. Instead of an inbox, they see a synthesized feed card.
  - **Card Example:** "New Catering Request from Alex."
    - *Subtext:* "Sales Agent drafted a $500 proposal. Ops Agent confirmed calendar availability for Saturday."
  - **Interaction:** The owner taps the card to expand. They see the draft proposal and the calendar block.
  - **Action:** A single primary button: "Approve & Send Proposal".
  - **Visual Design:** UniFi/Apple translucent glass cards. Deeply nested agent-to-agent negotiation logs are hidden behind an "Advanced Settings / Agent Log" toggle.

  ### AI Agent Integration Points
  - **Triage Router:** A fast, low-latency LLM call that categorizes incoming webhooks and drops strongly-typed events onto the Event Mesh.
  - **Departmental Agents:** Subscribed to specific event types (e.g., `ohc.tenant.123.sales.lead_received`). They process the event, update the shared DB (row-level security enforced), and emit completion/draft events.
  - **Synthesizer:** A separate agent or rule engine that listens for related completion events and packages them into a single, owner-friendly Action Card.

  ### Key Design Decisions
  - **Decoupled Architecture:** Agents do not call each other directly via gRPC/HTTP. They communicate asynchronously via the Event Mesh to ensure resilience, retryability, and traceability.
  - **Multi-Tenant Security:** Every event on the mesh MUST contain `tenant_id`. Subscribers MUST filter and execute within that tenant's database context.
  - **Owner-Centric Abstraction:** The owner never sees "Agent X sent message to Agent Y". They only see the business outcome.

  ## Implementation Prompt
  **User-Facing Outcome:** As an agency principal (Nora), when a potential client fills out my website contact form, my OHC app automatically generates an "Action Required" card. This card contains a drafted proposal (from Sales) and a set of drafted project tasks (from Ops). I just hit "Approve" to send the proposal and stage the project.

  **CUJ & Acceptance Criteria:**
  1. Implement the core Multi-Tenant Event Mesh infrastructure (using existing PG Skip-Locked or Redis patterns) supporting Publish/Subscribe with guaranteed `tenant_id` isolation.
  2. Create a generic "Event Router" that can accept an incoming payload and publish it to the mesh.
  3. Create at least two mock "Departmental Agents" (e.g., Sales and Ops) that subscribe to a shared event (e.g., `CustomerInquiry`).
  4. Ensure both agents independently process the event, update a shared database entity (e.g., a "Task" or "Draft Proposal"), and emit a "Completed" event.
  5. Provide Playwright E2E tests: Simulate a webhook intake, wait for the background queue to process both agent subscribers, and assert that the unified UI displays the combined result (e.g., a drafted proposal and a created task) on the mobile feed.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
