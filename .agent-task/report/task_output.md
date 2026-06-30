issue_title: "Agentic Autonomous Proposal & Contract Generation Workflow"
issue_description: |
  ## Title
  Agentic Autonomous Proposal & Contract Generation Workflow

  ## Problem Statement
  Service business owners and agency principals (like Nora the agency principal or Carlos the handyman) lose critical hours manually creating project proposals, estimating costs, and chasing down invoice approvals. Traditional platforms like HubSpot, HoneyBook, or Dubsado are either overly complex enterprise CRMs or require heavy upfront template configuration. OHC needs a lightweight, AI-driven flow that turns a casual client conversation directly into a professional, personalized proposal, secures approval, and schedules automatic invoice reminders—all without the owner typing a single paragraph on their mobile device.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **HoneyBook & Dubsado:** Excellent for independent creatives, but they rely heavily on manual data entry and static template builders. The user still has to "do the work" of writing the proposal.
  - **HubSpot / Salesforce:** Extremely powerful but complex; they demand an enterprise mindset, manual pipeline management, and desktop-first configurations that alienate micro-SMBs.
  - **Shopify / Wix:** Heavily biased toward e-commerce and physical products or simple booking widgets; they lack robust native quoting and milestone-based invoicing for custom service work.
  - **OHC Opportunity:** Leverage our Agent Feed and tenant-scoped memory. When a client requests custom work (e.g., "I need my kitchen painted" or "We need a new brand identity"), the *Sales & Revenue Assistant* automatically queries past successful proposals, standard pricing, and inventory/availability. It proactively drafts a proposal and contract. The owner's only job is to review the generated draft in their mobile Agent Feed and tap "Approve".

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Request / Intake Form] -->|Webhook| B(Work Triage Gateway)
      B --> C{Sales & Revenue Assistant}
      C -->|RAG Lookup| D[Tenant Knowledge & Past Proposals DB]
      C -->|Query| E[Operations Calendar / Inventory]
      C -->|Draft Generation| F[Action Required Queue]
      F --> G[Mobile Agent Feed 375px]
      G -->|Owner Taps Approve| H[Client Facing Proposal View]
      H -->|Client Approves| I[Stripe Checkout / Deposit]
      I --> J[Finance Agent - Schedules Invoice Reminders]
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Owner View (Agent Feed):** A translucent glass card appears on the 375px feed: "New Proposal Drafted for Acme Corp".
  - **Interaction:** Tapping the card opens a unified review screen. The top section summarizes the AI's logic ("Based on your previous 3 branding projects, I estimated 40 hours at $150/hr"). The bottom section shows the client-ready proposal.
  - **Action:** Primary button `Approve & Send`, secondary button `Edit Terms`, and a quick-adjust slider for `Total Price`.
  - **Client View:** A responsive, elegant web view adopting OHC Premium Tokens (Glassmorphism, #0066FF primary actions) presenting the scope, timeline, terms, and an integrated Stripe element for a one-tap deposit.

  ### AI Agent Integration Points
  - **Work Triage:** Parses the raw, unstructured client request (email, DM, or web form) into structured project requirements.
  - **Knowledge & Compliance Assistant:** Retrieves similar past work and ensures standard contract clauses (e.g., liability limits, payment terms) are included based on the tenant's policies.
  - **Sales & Revenue Assistant:** Synthesizes the data into a persuasive, plain-language proposal and calculates a recommended price.
  - **Finance & Decision Assistant:** Listens for the Stripe deposit webhook and automatically queues milestone invoice reminders.

  ### Key Design Decisions
  - **Zero-Template Drafting:** Instead of building a drag-and-drop template editor (which introduces friction), the LLM dynamically structures the proposal based on the context of the job.
  - **Mobile-First Approval:** The owner must be able to confidently read and approve a $10,000 proposal from a 375px phone screen while waiting in line for coffee.
  - **Unified Customer Graph:** The proposal is permanently linked to the client's identity, so future agents can reference the exact terms agreed upon.

  ## Implementation Prompt
  **User-Facing Outcome:** As an agency principal or field service owner, when a lead requests a custom project, I open my OHC app to find a fully drafted, accurately priced proposal waiting for my approval. I can adjust the price with a slider and send it to the client with one tap.
  **CUJ & Acceptance Criteria:**
  1. A simulated external client request is ingested by the Work Triage gateway.
  2. The Sales & Revenue Assistant agent is triggered, queries the tenant's previous pricing data, and generates a structured `Proposal` record.
  3. The `Proposal` appears in the tenant's Action Required queue.
  4. Implement the 375px mobile Agent Feed UI where the owner can view the proposal, adjust the price, and tap "Approve".
  5. Upon approval, the system generates a public URL for the client.
  6. Provide Playwright E2E tests: A user logs in, sees the drafted proposal in their feed, approves it, navigates to the public client URL, and verifies the generated content and Stripe deposit checkout flow.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
