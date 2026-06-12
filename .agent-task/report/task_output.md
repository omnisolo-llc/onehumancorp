issue_title: "Agentic Quote-to-Cash & Proposal Engine"
issue_description: |
  # Research Report: Agentic Quote-to-Cash & Proposal Engine

  ## 1. Problem Statement
  Service-based small business owners and agency principals (e.g., Nora the Agency Principal, Carlos the Handyman) spend countless hours managing the quote-to-cash lifecycle manually. When a lead comes in, they must gather requirements, calculate estimates, draft a custom proposal in Word or specialized tools like Proposify, send it via email, wait for client approval, manually generate an invoice in Stripe or Quickbooks, and chase down payments. This fragmented workflow delays deal closures, increases administrative overhead, and frustrates non-technical users who want to focus on delivering their service. Existing CRMs and invoicing tools offer discrete solutions but fail to provide a unified, automated, end-to-end agentic workflow that actively moves the deal forward.

  ## 2. Research Report
  - **Market Context**: Platforms like HoneyBook and Dubsado are popular among freelancers and small agencies because they combine proposals, contracts, and invoices. However, they rely entirely on the user to manually trigger the next step. Shopify focuses almost exclusively on product e-commerce and lacks native B2B or service quoting workflows.
  - **The OHC Opportunity**: OneHumanCorp can differentiate itself by offering an "Agentic Quote-to-Cash Engine." By leveraging the Sales and Finance AI Agents, OHC can autonomously transition a lead from an initial inquiry into a drafted proposal, automatically convert an approved proposal into an invoice, and follow up on outstanding payments—requiring only one-tap approvals from the business owner.
  - **Competitor Gaps**:
    - *HoneyBook / Dubsado*: Excellent workflow tools, but lack autonomous AI drafting and require heavy initial template setup.
    - *Shopify*: No native proposal or quoting mechanism for services; B2B features are gated behind enterprise tiers.
    - *Stripe Invoicing*: Powerful billing API, but lacks the upstream proposal drafting and negotiation interface.

  ## 3. Design Doc
  ### Data Model (PostgreSQL)
  - `Lead`: The initial inquiry and customer details.
  - `Proposal`: The drafted estimate, including line items, terms, and state (draft, sent, viewed, approved, rejected).
  - `Invoice`: The final billable entity linked to the approved Proposal and integrated with Stripe.
  - `Payment`: The record of the settled invoice.

  ### AI Agent Coordination
  - **Sales Agent ("The Closer")**: Analyzes the initial Lead inquiry, queries the business's service catalog and pricing guidelines, and drafts a complete `Proposal`. It pushes a notification to the owner's mobile feed for approval.
  - **Operations Agent ("The Manager")**: Monitors proposal states. Once a client approves the proposal online, it triggers the Finance Agent.
  - **Finance Agent ("The Accountant")**: Automatically generates the `Invoice` from the approved `Proposal` line items, schedules it in Stripe, and drafts a payment request email. It also monitors for overdue payments and drafts polite follow-up reminders.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Client Inquiry/Form] -->|Creates| B(Lead)
      B --> C[Sales Agent]
      C -->|Drafts| D[Proposal]
      D --> E{Owner Approval on Mobile}
      E -->|1-Tap Approve| F[Proposal Sent to Client]
      F -->|Client Accepts| G[Operations Agent]
      G -->|Triggers| H[Finance Agent]
      H -->|Generates| I[Invoice via Stripe]
      I --> J[Client Pays]
  ```

  ### Mobile UX Flow (375px First)
  1. **Owner View (Agent Feed)**: The owner receives a card: "New inquiry from Acme Corp. Proposal drafted. [Review & Send]".
  2. **Proposal Review**: The owner taps the card, viewing a clean, glassmorphism-styled summary of the drafted line items and total cost. They can edit fields or tap a large "Send Proposal" button (touch target ≥ 44x44px).
  3. **Client View**: The client receives a beautiful, mobile-friendly web link to review the proposal, with a prominent "Accept & Sign" button.
  4. **Post-Acceptance**: Once the client accepts, the owner receives another card: "Acme Corp accepted the proposal. Invoice drafted. [Send Invoice]".

  ## 4. Implementation Prompt
  **Feature Name**: OHC Agentic Quote-to-Cash & Proposal Engine
  **Target Persona**: Nora the Agency Principal
  **Outcome**: Nora receives a project inquiry on her OHC site. The Sales Agent immediately drafts a proposal based on her standard rates. Nora approves it with one tap. When the client accepts the proposal, the Finance Agent automatically generates the invoice and schedules payment reminders, handling the entire workflow invisibly.

  **Next Actions for Engineering**:
  1. Implement the core Data Models (`Lead`, `Proposal`, `Invoice`) ensuring strict multi-tenant isolation.
  2. Develop the Sales Agent capability to parse an unstructured inquiry and draft structured `Proposal` line items.
  3. Build the mobile-first (375px) Proposal Review and Approval UI for the owner's Agent Feed.
  4. Implement the client-facing Proposal Acceptance view.
  5. Connect the Operations and Finance Agents to automatically transition an approved `Proposal` into a Stripe `Invoice`.

  **Acceptance Criteria**:
  - The entire owner flow must be executable within a 375px viewport with no horizontal scrolling.
  - The transition from Proposal to Invoice must require zero manual data entry from the owner.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []