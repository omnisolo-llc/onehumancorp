issue_title: "Implement AI-Driven Autonomous Proposal Drafting & Client Intake for Agency Principals"
issue_description: |
  # Research Report: AI-Driven Autonomous Proposal Drafting & Client Intake

  ## Problem Statement
  Service-based small business owners, independent professionals, and agency principals (e.g., Nora the Agency Principal) spend a disproportionate amount of non-billable time capturing client requirements, estimating project scope, and manually drafting proposals and contracts. Existing tools either offer disconnected intake forms (Typeform) or require manual drafting in rich-text editors (Notion, Google Docs), lacking an integrated AI agent to transform rough client input directly into an owner-approved, sendable proposal synced with downstream project management and invoicing systems.

  ## Research Report & Gap Analysis
  - **Competitor Systems (HoneyBook, Dubsado, Notion AI):**
    - HoneyBook and Dubsado offer robust templated workflows but rely heavily on static forms and manual proposal creation. Their AI features are mostly bolt-on text assistants, not autonomous drafting agents.
    - Notion AI provides excellent drafting capabilities but lacks built-in billing, intake form, and project pipeline coordination.
  - **OHC Gap:**
    - OHC currently lacks an integrated, AI-first intake-to-proposal pipeline.
    - OHC needs an architecture where an intake form or incoming email automatically triggers an AI Agent (Sales/Operations) to draft a comprehensive, brand-aligned proposal based on the tenant's past successful proposals, standard pricing, and service catalog.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Intake Form / Email / DM] -->|Webhook / API| B(Intake Gateway)
      B --> C[Project Lead DB]
      C -->|Trigger| D[Sales & Proposals Agent]
      D -->|Query Past Docs| E[(Tenant Memory & Knowledge Base Vector DB)]
      D -->|Query Pricing| F[(Catalog & Pricing DB)]
      D -->|Generate Draft| G[Proposal Draft DB]
      G --> H[Owner Agent Feed Mobile UI 375px]
      H -->|1-Tap Approve & Send| I[Client Presentation Layer]
      I -->|Accept & Sign| J[Project Creation & Invoicing Trigger]
  ```

  ### Mobile UX Flow (375px First)
  - **Owner Feed (Dashboard):**
    - The owner sees an action card: "New Intake from Acme Corp. Proposal draft ready for review."
  - **Draft Review Screen:**
    - Clean, mobile-optimized view of the generated proposal (Scope, Timeline, Pricing).
    - Quick-action buttons: "Regenerate with lower budget", "Add Rush Fee", or "Approve & Send".
  - **Client View:**
    - A responsive, unbranded or custom-branded web link displaying the proposal, complete with an e-signature block and optional deposit payment integration.

  ### AI Agent Integration Points
  - **Sales Assistant Agent:**
    - Triggered on new lead creation. Uses a RAG pipeline against the `Knowledge Base` to fetch context on how the owner writes proposals.
    - Formulates the draft, calculates estimated costs based on standard pricing models, and pushes to the `Action Required` queue.
  - **Knowledge Assistant Agent:**
    - Continuously indexes sent and signed proposals to improve future draft accuracy and tone matching.

  ### Key Design Decisions
  - **Vector DB for Context:** Utilize pgvector for the Tenant Memory & Knowledge Base to enable semantic similarity searches on past proposals.
  - **Agent Handoff:** Ensure seamless transition from the Sales Agent (drafting) to the Finance Agent (deposit invoicing) upon client acceptance.
  - **Zero-Touch Drafting:** The default state should be a fully formed document requiring only a quick review, minimizing owner input.

  ## Implementation Prompt
  **User-Facing Outcome:** Nora the agency owner receives a new project request via her website. Within minutes, her OHC mobile app notifies her that a tailored proposal draft, complete with accurate pricing and scope, is ready. She reviews it on her phone, taps "Approve", and it is sent to the client.

  **CUJ & Acceptance Criteria:**
  1. Implement a `ProjectLead` and `ProposalDraft` database schema, including multi-tenant support and versioning.
  2. Integrate pgvector to store and query past proposal embeddings for the Knowledge Assistant.
  3. Build the Sales Assistant Agent workflow: triggered by a new lead, it retrieves relevant context, generates a proposal document (Markdown/HTML), and creates a pending action item for the owner.
  4. Develop the Owner Mobile UI (375px) for reviewing and approving the proposal, featuring quick-edit AI commands.
  5. **Verification:** Provide Playwright E2E tests simulating an intake submission, the generation of the proposal in the backend, the appearance of the action card in the owner feed, and the successful approval and dispatch of the proposal link.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []