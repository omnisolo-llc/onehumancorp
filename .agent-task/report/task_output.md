issue_title: "Architecture & Integration: 'The Dealmaker' Agent for Autonomous Proposal & Contract Generation"
issue_description: |
  # Research Report: The Dealmaker Agent - Proposal & Contract Automation

  ## 1. Problem Statement
  **Persona Focus:** Nora (Agency Principal)
  Nora runs a small design studio with contractors and clients. A critical friction point in her workflow is transitioning a client from initial interest (intake) to a signed contract and paid deposit. Currently, she manually pieces together notes from intake forms, copies past proposals, manually adjusts pricing and scope, generates a PDF, and sends it via email, then follows up manually for signatures and initial invoices. This fragmented process leads to lost leads, delayed starts, and a high administrative burden, taking her away from the actual work.

  ## 2. Research Report & Competitive Landscape
  **Market Gap:**
  - Tools like **PandaDoc**, **DocuSign**, and **HelloSign** excel at electronic signatures but lack native integration into the project's operational intake and billing lifecycle. They require manual document creation or complex template mapping.
  - Platforms like **HoneyBook** and **Dubsado** offer all-in-one solutions for creatives but present a steep learning curve with complex setup for workflows. They are not intrinsically "agentic" (i.e., they don't draft the proposal contextually based on an unstructured intake conversation).

  **OHC Advantage:**
  By leveraging OHC's underlying agents, the "Dealmaker" can automatically draft a proposal by reasoning over an initial client conversation or intake form, the agency's past successful proposals (RAG), and standard pricing models. This reduces the time-to-proposal from hours to seconds.

  ## 3. Design Doc: High-level Architectural Design

  ### Data Model & System Components
  - **Proposal/Contract Entities:** A `Document` entity stored in PostgreSQL with versioning. Linked to a `Project` and `Client`.
  - **Signature State:** A state machine tracking `Draft`, `Pending Approval` (Nora), `Sent`, `Viewed`, `Signed`, `Deposit Paid`.
  - **Knowledge Assistant (RAG):** Accesses previous successful proposals, standard clauses, and pricing guidelines to build contextual drafts.
  - **Sales & Revenue Assistant:** Generates the invoice/deposit link (Stripe Checkout Session) and embeds it into the proposal flow.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      autonumber
      participant Client
      participant Intake as Work Triage
      participant Dealmaker as Dealmaker Agent (Sales)
      participant Knowledge as Knowledge Agent
      participant Finance as Finance Agent
      participant DB as PostgreSQL Ledger
      participant Nora as Nora (Mobile App)

      Client->>Intake: Submits project inquiry (Form/DM)
      Intake->>Dealmaker: Triggers "New Lead" event
      Dealmaker->>Knowledge: Query past similar proposals & pricing
      Knowledge-->>Dealmaker: Return contextual guidelines
      Dealmaker->>Finance: Request deposit checkout link
      Finance-->>Dealmaker: Return Stripe Payment Link
      Dealmaker->>DB: Save drafted Proposal document
      Dealmaker->>Nora: Push Notification: "Proposal Drafted for new Lead"
      Nora->>Dealmaker: Reviews on Mobile & taps "Approve & Send"
      Dealmaker->>Client: Emails Proposal with Sign & Pay link
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification Card:** Nora receives a push notification and sees an Action Card in her Agent Feed: "New project inquiry from Acme Corp. Proposal drafted. [Review]"
  2. **Review Screen:** Tapping the card opens a streamlined, single-column view:
     - **Summary Block:** "Scope: Branding package. Value: $5,000. Deposit: 50%."
     - **Content Preview:** A vertically scrollable, simplified text view of the proposal, stripped of complex formatting but preserving the core text and clauses.
     - **Action Bar (Sticky at bottom):** Two primary buttons: "Edit" (opens native keyboard to tweak text) and "Approve & Send" (high contrast, 44x44px minimum touch target).
  3. **Confirmation State:** A translucent glass success overlay appears, then routes back to the main Agent Feed.

  ### AI Agent Integration Points
  - **Trigger:** Webhook from a new client message or intake form submission.
  - **Generation:** Gemini Pro prompt injected with the intake text and retrieved context (RAG) to generate a structured markdown or HTML proposal.
  - **Handoff:** The Sales Assistant orchestrates the document creation and interfaces with the Finance Assistant to attach the correct billing intent.

  ## 4. Implementation Prompt

  **Feature Name:** The Dealmaker Agent - Automated Proposal Generation

  **Target Persona:** Nora the Agency Principal

  **Outcome:** When a new lead submits an inquiry, the system automatically drafts a comprehensive proposal and contract, complete with a deposit payment link. Nora can review, modify, and send this document from her phone with a single tap.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. As a test user (Nora), log into the OHC mobile web app (375px viewport).
  2. Simulate a new lead inquiry via the Work Triage system.
  3. The Sales Assistant ("Dealmaker") must automatically draft a proposal by synthesizing the inquiry details and injecting a generated deposit payment link.
  4. An Action Card must appear in the Agent Feed with the drafted proposal summary.
  5. Nora must be able to tap the card, view a mobile-optimized text preview of the proposal without horizontal scrolling, and tap "Approve & Send".
  6. The system transitions the document state to "Sent" and simulates email dispatch to the client.

  **Notes for Implementer:**
  - Ensure the drafted proposal uses a clean, simple text structure suitable for a mobile review.
  - No complex visual document builder is needed for the MVP—focus on plain text/markdown contracts.
  - The feature must be resilient to network flakiness; if Nora taps "Approve & Send" offline, the action should queue and sync when reconnected.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
