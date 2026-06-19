issue_title: "AI-Automated Project Intake & Proposal Generation"
issue_description: |
  ## Title
  AI-Automated Project Intake & Proposal Generation

  ## Problem Statement
  Agency principals (like Nora) struggle with the manual effort required to intake new client requests, estimate scope, and draft proposals. This process involves switching between emails, CRM, and document editors. They need an automated system that captures client requests, uses historical data to estimate scope, and proactively drafts a formal proposal for review, significantly reducing the turnaround time from lead to quote.

  ## Research Report
  - **Market Context:** Traditional project management and CRM tools (like HubSpot, Asana, or Monday.com) require significant manual data entry to create proposals. Specialized proposal software (like Proposify or PandaDoc) handles document creation well but lacks deep, autonomous AI integration that actively learns from past successful proposals to draft new ones without user intervention.
  - **OHC Opportunity:** By integrating a custom Intake Form and utilizing the Sales/Operations Agents, OHC can automatically ingest new leads, extract requirements, estimate pricing based on the agency's configured rates, and instantly draft a complete proposal.
  - **The Gap:** OHC currently lacks an end-to-end autonomous flow from initial client contact to a drafted, ready-to-send project proposal that aggregates context across past projects.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Intake Form / Email] -->|Webhook| B(Omnichannel Gateway)
      B --> C{Lead Intake Service}
      C --> D[Sales Agent]
      D -->|Query Past Proposals| E[(Vector DB/PostgreSQL)]
      D -->|Estimate Scope & Cost| F[Operations Agent]
      F --> D
      D -->|Draft Proposal| G[Action Required Queue]
      G --> H[Mobile App Feed 375px]
      H -->|1-Tap Approve| I[Email/PDF Dispatcher]
  ```

  ### Mobile UX Flow (375px)
  1. **Notification:** Nora receives a push notification: "New Project Request from Acme Corp. Proposal drafted."
  2. **Feed View:** Tapping the notification opens a card in the Agent Feed detailing the client's request summary, the AI's estimated price, and timeline.
  3. **Proposal Review:** A "Review Draft" button opens a simplified, mobile-optimized view of the proposal document.
  4. **Action:** Nora can tap "Approve & Send" directly, or "Edit" to modify the scope and price fields using native mobile inputs. The interface must maintain a 44x44px touch target size for all interactive elements.

  ### AI Agent Integration Points
  - **Sales Agent ("The Promoter/Sales"):** Triggers upon receiving a new lead. It uses RAG against previous successful proposals in the tenant's memory to match the requested services with standard verbiage and scope descriptions.
  - **Operations Agent ("The Manager"):** Calculates estimated costs and timelines based on the agency's configured hourly rates and historical project durations, feeding this back to the Sales Agent for the final draft.

  ### Key Design Decisions
  - **Proactive Proposal Generation:** The system does not wait for Nora to click "Create Proposal"; it is drafted immediately upon lead capture.
  - **Mobile-First Editing:** Complex documents are often hard to edit on mobile. OHC abstracts the proposal into key variables (Price, Scope, Timeline) that can be easily adjusted via mobile sliders/inputs, while the AI manages the document formatting in the background.

  ## Implementation Prompt
  **Feature Name:** AI-Automated Project Intake & Proposal Generation
  **Target Persona:** Nora the Agency Principal

  **User-Facing Outcome:** When a potential client submits a project request via Nora's OHC-hosted intake form, the OHC system immediately drafts a tailored proposal. Nora opens her phone, reviews the summary and estimated price on a clean, translucent glass-styled card, and taps "Approve" to send a professional PDF proposal to the client.

  **Next Actions for Engineering:**
  1. Implement the `LeadIntake` data model to capture structured requirements.
  2. Extend the `SalesAgent` to orchestrate proposal drafting using the LLM and past project context.
  3. Create the mobile-first "Proposal Draft Review" UI component, ensuring 375px layout compatibility and 44x44px touch targets.
  4. Integrate PDF generation and email dispatch for the finalized proposal.

  **Acceptance Criteria:**
  - E2E Playwright test must cover the full flow from intake submission to proposal approval via the mobile UI.
  - The UI must contain zero mock data; all proposals must be generated from real or test-seeded backend state.
  - Must use the repository's standard `execute_with_retry` and transaction patterns for data persistence.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
