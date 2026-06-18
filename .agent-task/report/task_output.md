issue_title: "The Negotiator Agent: AI-Driven Field Estimates & Automated Proposal System"
issue_description: |
  # Research Report: The Negotiator Agent - AI-Driven Field Estimates & Automated Proposal System

  ## 1. Problem Statement
  Service-based small business owners, specifically field service operators like Carlos the Handyman and agency principals like Nora, experience significant friction in the lead-to-proposal pipeline. Currently, providing an accurate estimate involves site visits or lengthy back-and-forth communication, manual calculation of materials and labor, and drafting formal proposals. This process is time-consuming, prone to calculation errors, and often leads to lost leads due to slow response times. Existing CRM tools are disconnected from the primary workflow and lack proactive intelligence, requiring the owner to act as the primary data entry clerk.

  ## 2. Research & Competitive Analysis
  - **Traditional Invoicing Tools (Square, QuickBooks):** Provide templates for estimates but require complete manual data entry. They are reactive and do not assist in formulating the estimate itself.
  - **Specialized Field Service Software (Jobber, ServiceTitan):** Powerful but highly complex and expensive, aimed at larger fleets rather than the solopreneur or micro-agency. They often lack a mobile-first, zero-friction setup.
  - **OHC Opportunity:** By introducing "The Negotiator Agent," OHC can transform the proposal process from a multi-day manual task into a sub-5-minute guided approval flow, executed entirely on a mobile device. The AI agent bridges the gap between customer inquiry and a professional, bookable estimate.

  ## 3. Architecture & Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry/Photo via OHC Inbox] -->|Webhook| B(Omnichannel Gateway)
      B --> C{The Negotiator Agent}
      C -->|Analyze Request & Extract Needs| D[Gemini Pro / Vision]
      D -->|Query Pricing Models & Past Jobs| E[Tenant Knowledge Graph]
      C -->|Draft Estimate| F[Proposal Engine]
      F -->|Generate Action Card| G[Mobile Unified Agent Feed 375px]
      G -->|Owner Taps 'Approve & Send'| H[Omnichannel Dispatcher]
      H -->|Send Proposal Link| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  1. **Agent Feed (Mobile):** The owner sees a new card: "Carlos, a new lead requested a bathroom tile repair estimate. I've drafted a proposal based on the photos provided."
  2. **Card Expansion:** Tapping the card reveals a clean, structured summary of the drafted estimate: Labor (estimated hours x rate), Materials (extracted from photo analysis/description), and a suggested buffer.
  3. **Interaction:** The owner can adjust sliders for labor/materials if needed (native mobile UI, large touch targets ≥ 44x44px).
  4. **Approval:** A prominent primary button "Approve & Send Proposal".
  5. **Customer View:** The customer receives a link to a mobile-optimized web view of the proposal with an integrated "Accept & Pay Deposit" Stripe flow.

  ### AI Agent Integration Points
  - **The Negotiator (Sales/Estimating Agent):** Triggered by lead inquiries containing request details or images. Utilizes Gemini Vision to analyze user-uploaded photos (e.g., assessing the size of a repair job) and Gemini Pro to parse text. It cross-references the tenant's predefined pricing models (hourly rates, standard material costs) to formulate an initial estimate.
  - **The Manager (Operations Agent):** Collaborates with The Negotiator to ensure suggested project dates align with current calendar availability before drafting the proposal.

  ### Key Design Decisions
  - **Image-to-Estimate Processing:** Leveraging multimodal LLMs to reduce manual scoping.
  - **Approval-First Workflow:** The agent drafts the entire proposal; the owner merely reviews and approves, shifting from creation to curation.
  - **Integrated Deposit Flow:** The proposal isn't just a document; it's a transactional artifact linked directly to Stripe Checkout for immediate conversion.

  ## 4. Implementation Prompt
  **Feature Name:** The Negotiator Agent - AI Automated Proposals
  **Target Personas:** Carlos (Handyman) and Nora (Agency Principal)

  **Outcome:** When a lead requests a quote (via form or message) and provides details/photos, The Negotiator Agent automatically analyzes the request, calculates a draft estimate using the owner's pricing rules, and surfaces a ready-to-send proposal card in the mobile Agent Feed for 1-tap approval.

  **Critical User Journey (CUJ) & Acceptance Criteria:**
  1. A lead submits a service request containing a text description and an optional photo via the OHC storefront.
  2. The Omnichannel Gateway ingests the request and triggers The Negotiator Agent.
  3. The Agent successfully queries the tenant's pricing model and generates a structured draft proposal (Labor, Materials, Total).
  4. The draft proposal appears as an actionable card in the owner's mobile Agent Feed (375px viewport constraint).
  5. The owner taps "Approve & Send," which finalizes the proposal and sends a payment link to the lead.
  6. Provide Playwright E2E tests: Simulate a lead submission, verify the drafted proposal appears in the owner's feed, and verify the approval action correctly updates the proposal state and dispatches the link.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
