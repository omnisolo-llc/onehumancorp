issue_title: "OHC-04: Inquiry Capture & Quoting vs HoneyBook"
issue_description: |
  # Title: OHC-04: Customer inquiry capture, qualification, and proposal generation

  # Problem Statement
  Small business owners (such as Nora, our target persona) struggle with capturing inbound inquiries across various platforms (Instagram, Email, WhatsApp) and turning them into qualified, priced proposals without significant manual data entry and context switching. Existing solutions like HoneyBook are heavily desktop-oriented and require complex setup, which creates friction.

  # Research Report
  - **HoneyBook Baseline**: HoneyBook offers comprehensive intake forms and proposal-to-payment workflows. However, user sentiment and verified app store reviews frequently cite the steep learning curve for non-technical users and the heavy reliance on rigid, predefined forms rather than fluid, conversational omnichannel capture.
  - **Claude for Small Business**: Offers great conversational ability but lacks native integrations with the owner's booking systems and payment gateways without complex API stitching.
  - **OHC Gap**: OHC possesses the foundational code for parsing inquiries via LLM (`analyze_intake_inquiry`) and generating draft quotes (`QuoteDraft`). However, the mobile (375px) agent feed needs refinement to ensure owners can immediately review, adjust (price/deposit), and approve drafts generated from raw omnichannel inputs seamlessly.

  # Design Doc
  - **Architecture**: Omnichannel Webhook -> `ClientIntake` / `analyze_intake_inquiry` -> LLM classification -> `QuoteDraft` DB insertion -> `UnifiedAgentFeed`.
  - **UI Wireframes**: A mobile-first feed displaying incoming triage cards.
  - **Mobile UX Flow (375px first)**:
    1. A new inquiry arrives from Instagram DM.
    2. OHC parses the text, determining intent ("SERVICE_REQUEST") and drafts a quote (e.g., $1,500).
    3. The owner's OHC mobile app feed shows an "Approve Estimate" card.
    4. The owner taps "Review", sees the suggested price and scope.
    5. The owner adjusts the price if needed, toggles deposit requirements, and taps "Approve & Send".
  - **AI Agent Integration Points**: The `SalesAgent` acts as the orchestrator to parse the inquiry, query long-term memory for past proposals to find pricing heuristics, and output a structured draft.

  # Implementation Prompt
  - Ensure the `client_intake` webhook fully parses raw conversational text using `OMNISOLO_SALES_LLM_PROVIDER`, extracting the service name, suggested price, and project scope.
  - Render a "Quote Draft" review card in the `UnifiedAgentFeed` that prominently displays the AI's suggested price and scope.
  - Implement the "Review Proposal" modal that allows the owner to quickly edit the line items and click "Approve & Send" without navigating away from the feed.
  - Acceptance Criteria: A new webhook payload creates a `QuoteDraft` which appears in the feed; the owner can approve it, generating a final `Quote` with a Stripe payment link.

  # Priority
  P2

  # Estimated Scope
  Medium

  # Strategy Admission
  - **OHC Target ID**: OHC-04
  - **Stage**: Launch
  - **Observed Gap**: Owners need conversational intake mapped to structured quotes.
  - **Evidence Level**: Documented operator friction with complex form builders.
  - **Baseline/Measurable Result**: Time-to-quote reduced from 20 minutes (manual) to 1 minute (AI draft approval). Denominator: total qualified inquiries.
  - **Dependencies/Reuse**: Reuses `analyze_intake_inquiry` and `QuoteDraft` entities.
  - **Non-Goals**: We are not building a visual drag-and-drop form builder like HoneyBook.
  - **Authority Class**: Routine digital operation under explicit standing authority for drafting; owner review required for final quote sending.
  - **Cost/Measurement Plan**: Track LLM compute cost per inquiry against the subscription margin.
  - **Acceptance Checks**: Happy path successfully generates quote from natural language; failure path handles ambiguous inquiries by flagging for owner clarification.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
