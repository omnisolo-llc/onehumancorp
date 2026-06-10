issue_title: "Agentic Client Intake & Autonomous Proposal Generation Engine"
issue_description: |
  # Mission Queue Protocol Brief

  ## Problem Statement
  Service-based small business owners, agency principals, and independent professionals (e.g., Nora the Agency Principal, Carlos the Handyman) currently lose massive amounts of time on the manual back-and-forth of client intake. When a new lead contacts them via DM, email, or a web form, the owner has to manually gather project requirements, switch contexts to a document editor, draft a tailored proposal or quote, calculate pricing, and send it back. This fragmentation leads to delayed responses, lost leads (drop-offs due to friction), and operational burnout. Existing tools (like HoneyBook or Dubsado) are powerful but require heavy manual setup of templates and complex dashboard management, failing the "Grandmother Test."

  ## Research Report
  - **Market Context:** Traditional CRM and proposal software (HubSpot, Jobber, HoneyBook) focuses on providing templates and pipelines. However, they lack true autonomous agentic capabilities. They wait for the user to initiate the drafting process.
  - **Competitor Gaps:**
    - *Shopify:* Focused heavily on physical e-commerce and catalog sales; weak on custom service quoting.
    - *Durable:* Excellent zero-to-one website generation but lacks a deep operational engine for ongoing custom project intake and proposal drafting.
    - *11x / Skyvern:* High-end B2B sales agents that are too complex and expensive for a micro-SMB or solopreneur.
  - **The OHC Opportunity:** By integrating an "Intake & Proposal Agent" directly into the OHC unified feed, we can capture demand from any channel (DM, email, widget) and instantly convert it into a drafted, structured proposal. The owner simply reviews the AI-generated draft on their phone and taps "Approve & Send," reducing a 45-minute task to a 30-second review.

  ## Design Doc
  ### High-Level Architecture
  - **Intake Gateway:** Unifies inbound requests (Forms, DMs, Emails) into a single standard Lead object.
  - **Context & Memory Engine:** The agent retrieves the business's past proposals, pricing rules, and service catalog to understand the owner's style and pricing structure.
  - **Proposal Generation Pipeline:** An AI pipeline that maps the raw lead intent to a structured `Quote` or `Proposal` data model.
  - **Approval UI:** A mobile-optimized (375px) card in the Unified Agent Feed presenting the drafted proposal.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      autonumber
      actor Client as Client (Web/DM)
      participant Gateway as Intake Gateway
      participant Agent as Sales & Revenue Agent
      participant Memory as Knowledge & Memory
      participant UI as OHC Mobile Feed
      actor Owner as Nora (Owner)

      Client->>Gateway: "I need a brand design for my new cafe."
      Gateway->>Agent: New Lead Event
      Agent->>Memory: Fetch Nora's pricing & past brand proposals
      Memory-->>Agent: Pricing rules & brand templates
      Agent->>Agent: Generate structured Proposal Draft
      Agent->>UI: Push "Action Required" Card
      UI->>Owner: Push Notification: "New Proposal Drafted for Cafe Brand"
      Owner->>UI: Opens app, reviews draft, adjusts price slider
      Owner->>UI: Taps "Approve & Send"
      UI->>Client: Delivers secure, branded Proposal & Deposit Link
  ```

  ### Mobile UX Flow (375px First)
  1. **Notification:** Nora receives a push notification indicating a new proposal draft is ready based on a client inquiry.
  2. **Feed Card:** Opening the app, the first item in the Unified Feed is the "Drafted Proposal: Cafe Brand Design."
  3. **Review Modal:** Nora taps the card. A translucent glass half-sheet slides up showing a simplified summary:
     - Client Name & Request
     - AI-Suggested Scope of Work
     - Price (with a simple slider or editable text field for adjustments)
     - Deposit requirement
  4. **Approval:** A massive "Approve & Send" button (min 44x44px touch target) sits at the bottom.
  5. **Confirmation:** The card transforms into a "Sent" state, and the Operations Agent is notified to await the deposit payment.

  ### AI Agent Integration Points
  - **Work Triage Agent:** Intercepts the raw inquiry and routes it to the Sales Agent.
  - **Sales & Revenue Agent:** Drafts the actual proposal content, matching the owner's tone and calculating the estimated price based on prior data.
  - **Finance Assistant:** Prepares the linked deposit invoice that is attached to the final proposal link.

  ### Key Design Decisions
  - **Abstract the Document:** The owner never edits a traditional "document" on their phone. They review structured data (Scope, Price, Timeline) which OHC automatically renders into a beautiful web-based proposal for the client.
  - **Optimistic Generation:** The system must draft the proposal *before* the owner even opens the app, making the experience feel magical and proactive.

  ## Implementation Prompt
  Implement the "Agentic Client Intake & Autonomous Proposal Generation Engine."
  Create the core data models for capturing inbound leads and structuring proposals. Build the pipeline for the Sales Agent to automatically generate a draft proposal when a new lead is captured. Finally, implement the mobile-first (375px) UI in the Unified Agent Feed that displays the drafted proposal, allows for quick pricing/scope adjustments, and provides a 1-tap "Approve & Send" interaction. The UI must utilize macOS-style Translucent Glass materials and UniFi-style card layouts, ensuring all complex document generation happens invisibly in the background. Ensure the E2E Playwright tests cover the flow from lead creation to proposal approval.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
issue_scope: Large
