issue_title: "Agentic Project Intake & Smart Proposal Engine"
issue_description: |
  # Mission Queue Protocol: Agentic Project Intake & Smart Proposal Engine

  ## Problem Statement
  Service-based owners and operators—such as Nora (Agency Principal) and Carlos (Handyman)—spend disproportionate amounts of time translating casual customer inquiries (DMs, emails, simple forms) into structured project proposals. Traditional e-commerce platforms (Shopify, Wix) treat services as static products, failing to accommodate dynamic scoping, custom pricing, and multi-step approvals. Meanwhile, enterprise CRMs (HubSpot, Salesforce) are far too complex for micro-SMEs, requiring manual data entry and configuration that small business owners do not have time for. They need a system that acts as an intelligent sales assistant: capturing intent, drafting a scoped proposal based on historical pricing, and queuing it for owner approval.

  ## Research Report & Competitive Analysis
  - **Shopify/Wix/Squarespace**: Treat services essentially as physical products with variations. They lack native project scoping, milestone billing, and contract generation without heavy third-party app reliance.
  - **HoneyBook / Dubsado**: Popular among freelancers, these tools provide good proposal and invoicing features but require heavy manual setup of templates and workflows. They are not AI-first, meaning the owner still does the heavy lifting of drafting each proposal.
  - **OHC Differentiation**: OHC integrates the "Sales Agent" and "Operations Agent" directly into the work feed. When an intake request arrives, the agents automatically parse the requirements, retrieve similar past projects from the tenant's history via RAG, draft a line-item proposal with a payment schedule, and present it to the owner in a unified mobile-first feed for 1-tap approval.

  ## Design Doc: System Architecture & Data Model

  ### 1. Data Model (PostgreSQL)
  - `ProjectIntake`: Stores raw customer inquiry data and extracted intent.
  - `Proposal`: Linked to `ProjectIntake` and `Customer`. Contains `status` (draft, pending_approval, sent, accepted, rejected).
  - `ProposalLineItem`: Services, descriptions, estimated hours, and prices.
  - `ProjectTask`: Auto-generated operational tasks linked to a `Proposal` that activate upon customer acceptance.

  ### 2. AI Department Coordination
  - **Sales Agent ("The Closer")**: Triggered by a new `ProjectIntake`. Uses RAG to find similar past proposals to estimate pricing. Drafts the `Proposal` and a personalized message to the client.
  - **Operations Agent ("The Manager")**: Drafts the preliminary `ProjectTask` list based on the proposed scope, ensuring the owner has visibility into the fulfillment effort before sending the quote.
  - **Finance Agent ("The Accountant")**: Automatically stages the Stripe Payment Link / Invoice for the initial deposit once the proposal is approved.

  ### 3. Mobile-First UX Flow (375px)
  1. **Triage Feed**: Nora opens the OHC app and sees a new card: "New Project Request: Website Redesign for Local Cafe."
  2. **Agent Draft**: Tapping the card reveals the Sales Agent's drafted proposal, displaying line items, total cost ($3,500), and the drafted email response.
  3. **Edit & Approve**: Nora can tap any line item to adjust the price or edit the text. Touch targets are large (44x44px min). She taps a prominent "Approve & Send" floating action button.
  4. **Customer Experience**: The customer receives a responsive, translucent-glass styled web link to review the proposal, sign digitally, and pay the deposit via Stripe.

  ## Implementation Prompt
  **Feature Name**: Agentic Project Intake & Smart Proposal Engine
  **Target Persona**: Nora (Agency Principal), Carlos (Handyman)

  **Outcome**: When a customer inquiry is received, OHC autonomously drafts a detailed, line-item proposal and fulfillment task list, presenting it to the owner for one-tap approval.

  **Acceptance Criteria**:
  1. Implement the `ProjectIntake`, `Proposal`, and `ProposalLineItem` schema with strict row-level security by `tenant_id`.
  2. Extend the `Sales Agent` capabilities to intercept new intakes and generate a `Proposal` draft.
  3. Create a mobile-first (375px) UI component for the owner's Work Feed that displays pending proposal drafts with edit and "Approve & Send" functionalities.
  4. Build a customer-facing responsive view for accepting the proposal and initiating the Stripe deposit.
  5. Include E2E Playwright tests verifying the end-to-end flow from intake creation to owner approval.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
