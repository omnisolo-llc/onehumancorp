issue_title: "Architectural Gap: AI Agentic Quoting & Proposal Generator"
issue_description: |
  ## Title
  Architectural Gap: AI Agentic Quoting & Proposal Generator for Service Businesses

  ## Problem Statement
  Service-based and project-based small businesses (such as Carlos the Handyman and Nora the Agency Principal) lose significant time and potential revenue due to the manual effort required to generate quotes, proposals, and invoices. Gathering requirements, drafting the document, calculating costs, and sending the proposal often takes hours or days. The longer the delay, the lower the close rate. Existing platforms (like HoneyBook or QuickBooks) require manual data entry and complex template configuration, which are hostile to mobile-first users.

  ## Research Report
  - **Market Context**: Platforms like Jobber, Housecall Pro, and HoneyBook offer robust quoting tools, but they require owners to manually build out itemized lists and write descriptions.
  - **Competitive Analysis**:
    - *Shopify/Wix*: Heavily optimized for product sales; quoting for custom services is practically nonexistent natively.
    - *HoneyBook/Dubsado*: Good for freelancers but require extensive setup of templates and manual intervention for every lead.
  - **The OHC Opportunity**: OHC can differentiate by leveraging the "Sales & Revenue Assistant" to autonomously turn a rough customer inquiry (e.g., an Instagram DM or a quick phone call summary) into a fully drafted, professional quote or proposal. The owner simply reviews the draft on their mobile device, makes quick adjustments, and taps "Approve & Send".
  - **Data Justification**: Service businesses that respond to inquiries with a quote within 1 hour are 7x more likely to win the job. Reducing quoting time from hours to minutes via AI drafting directly impacts the owner's bottom line.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry: Chat/Form/Call] -->|Ingestion| B(Work Triage Pipeline)
      B --> C{Sales & Revenue Assistant}
      C -->|RAG Lookup| D[(Past Quotes, Pricing Matrix, Context)]
      C -->|LLM Draft| E[Drafted Quote Proposal]
      E --> F[Agent Feed Action Card]
      F -->|Owner Approves| G(Stripe Invoicing / Quotes API)
      G --> H[Sent to Customer with Payment Link]
  ```

  ### AI Agent Integration Points
  - **Work Triage Pipeline**: Captures unstructured incoming requests (e.g., "I need my kitchen sink fixed, it's leaking from the bottom pipe").
  - **Sales & Revenue Assistant (The Estimator)**:
    - Uses RAG to pull the tenant's pricing guidelines, past similar jobs, and available materials.
    - Drafts an itemized quote (Labor, Materials) and a professional message to the customer.
  - **Finance Assistant**: Automatically creates a pending Stripe Quote or Payment Link for a deposit upon owner approval.

  ### Mobile UX Flow
  1. **Notification (375px)**: The owner receives a push notification: "New Quote Drafted for John Doe (Kitchen Sink Repair)."
  2. **Action Card**: In the Agent Feed, a card displays the inquiry summary and the AI-itemized quote (e.g., $150 Labor, $50 Parts).
  3. **Interaction**: The owner can tap an item to quickly adjust the price (native mobile numpad) or edit the description.
  4. **Execution**: A prominent, full-width `44x44px` "Approve & Send" button generates the final document via Stripe and sends the link to the customer via SMS/Email.

  ## Implementation Prompt
  Implement the AI Agentic Quoting capability within the Sales & Revenue Assistant domain. Create the data models necessary to store pricing guidelines and past jobs for RAG. Develop the asynchronous worker that listens for "Quote Request" intents from the Work Triage pipeline. Integrate with the LLM to generate the itemized quote structure, and build the Mobile-First (375px) Action Card in the Agent Feed that allows the owner to review, adjust, and approve the quote via Stripe. Ensure zero mock data is used; the system must generate a real Stripe Quote/Invoice upon approval.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []