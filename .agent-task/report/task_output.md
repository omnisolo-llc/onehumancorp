issue_title: "Nora's Agent: AI-Guided Client Intake & Automated Proposal Generation"
issue_description: |
  ## Problem Statement
  Agency principals and freelancers like Nora spend hours manually triaging client requests, qualifying leads, scoping work, and drafting proposals. Often, high-value inquiries slip through the cracks or take days to get a response. Legacy platforms like HoneyBook or Dubsado offer form templates and workflows, but they still require the owner to manually read the intake, decide the scope, and compose the actual proposal. Nora needs a unified work assistant that not only captures the intake but autonomously queries her past projects to scope the work, drafts a personalized proposal, and presents it to her for a 1-tap approval.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **HoneyBook / Dubsado:** Excellent at workflow automation (if X form is submitted, send Y brochure), but they lack true AI reasoning. The owner still has to draft the custom proposal.
  - **Notion AI / ChatGPT:** Great for drafting text, but disconnected from the business's CRM, pricing ledger, and operational context. Requires manual copy-pasting.
  - **OHC Opportunity:** By leveraging the Sales & Revenue Assistant combined with Knowledge & Document memory, OHC can instantly turn an ambiguous intake email or form submission into a structured project scope and drafted proposal.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Intake Form/Email] -->|Webhook/Parse| B(Intake Processing Service)
      B --> C{Sales & Revenue Assistant}
      C -->|Query Past Proposals| D[(Knowledge DB/Vector Store)]
      C -->|Query Services/Pricing| E[(Ledger/Offers DB)]
      C -->|Generate Scope & Pricing| F[Draft Proposal]
      F --> G[Action Required Queue]
      G --> H[Mobile Agent Feed 375px]
      H -->|1-Tap Approve & Send| I[Email/Link Dispatcher]
  ```

  ### Mobile UX Flow (375px First)
  - **Agent Feed (Mobile Home):** Nora sees an urgent card: "New Intake: ACME Corp Branding. Proposal Drafted."
  - **Interaction:** Nora taps the card.
    - **Top Section (Context):** A brief summary of the client's request ("ACME wants a logo refresh and 3-page site").
    - **Middle Section (The Draft):** The AI-generated scope, timeline (based on Nora's current calendar availability), and price (based on past similar projects).
  - **Action:** Primary button "Approve & Send Proposal". Secondary buttons "Edit Draft" and "Ask Agent to Adjust (e.g. 'Make it 20% more expensive')".
  - **Visual Design:** Clean, translucent glass cards (macOS style), large touch targets (>= 44px), zero technical jargon.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Acts as the primary orchestrator for this CUJ.
  - **Knowledge Assistant:** Provides semantic search over Nora's past successful proposals to ensure the tone and pricing match her standards.
  - **Operations Assistant:** Queried briefly to ensure Nora actually has availability to start the project.

  ## Implementation Prompt
  **Objective:** Build the automated client intake to proposal generation pipeline and its corresponding mobile-first approval card in the Agent Feed.

  **User-Facing Outcome:** When a new lead submits a request, Nora immediately receives an Agent Feed card containing a fully drafted, context-aware proposal ready for approval.

  **Critical User Journey (CUJ):**
  1. Nora logs into the OHC app (375px viewport).
  2. A simulated client intake payload is ingested (e.g., via a test webhook).
  3. The Sales Agent queries Nora's mock past projects and generates a draft proposal.
  4. Nora sees the "Proposal Drafted" card in her feed.
  5. Nora taps "Approve & Send", and the system records the proposal as sent.

  **Acceptance Criteria:**
  - Intake processing endpoint must accept unstructured text and extract key project parameters.
  - Proposal generation must utilize a configured LLM provider and incorporate tenant-scoped knowledge.
  - The UI must be implemented following the 375px mobile-first standard with a 1-tap approval flow.
  - All backend-to-frontend interactions must be covered by Playwright E2E tests simulating Nora's journey. No mock data in the UI.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
