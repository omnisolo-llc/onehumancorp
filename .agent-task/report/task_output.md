issue_title: "[Architectural Gap] Agentic Smart Quote & Proposal Generation System"
issue_description: |
  ## Title
  Agentic Smart Quote & Proposal Generation System

  ## Problem Statement
  Service-based and project-based owners (such as Carlos the Handyman and Nora the Agency Principal) spend hours drafting estimates, proposals, and contracts using disconnected tools (Word, Excel, QuickBooks, Joist). These manual workflows delay the "quote-to-cash" cycle, lead to forgotten follow-ups, and force the owner to act as an administrator rather than an operator. When a customer agrees, the owner must manually convert the quote to an invoice and generate a Stripe payment link.

  ## Research Report
  - **Market Context**: Platforms like Shopify are heavily skewed toward physical/digital products and lack native, professional service-quoting tools. Competitors like QuickBooks, HoneyBook, and Joist offer quoting, but they are passive software suites requiring manual data entry for line items, taxes, and customer details.
  - **Competitor Systems Audit**:
    - *HoneyBook/Dubsado*: Excellent proposal-to-payment flows, but no AI agent to autonomously draft the proposal based on a rough 1-sentence prompt.
    - *Stripe Invoicing/Quotes*: High-scale and robust, but acts as a pure utility. It lacks the "owner-assistant" context to automatically chase the customer or draft the accompanying SOW (Statement of Work).
  - **The Gap**: OHC currently lacks a unified data model and agentic workflow for the "Quote-to-Cash" journey. We need an architecture where the Sales & Revenue Assistant can ingest a messy, informal input (e.g., an SMS from the owner: "Quote John $500 for roof repair, 50% upfront"), structure it into a formal Estimate, link it to the CRM, and deploy a mobile-optimized approval/deposit flow.

  ## Design Doc
  ### Data Model & Invariants (Multi-Tenant)
  - `Proposal`: Represents the quote/estimate. Contains `tenant_id`, `customer_id`, `status` (draft, sent, viewed, accepted, declined), `expires_at`, and `total_amount`.
  - `ProposalLineItem`: Links to `Proposal`, detailing services/products, quantities, and unit prices.
  - `ContractTemplate`: Optional legal text (managed by Knowledge & Compliance Assistant) dynamically attached to the Proposal.
  - **Invariants**:
    - Row-level security ensures strict isolation by `tenant_id`.
    - `ohc:lock:{tenant_id}:proposal:{proposal_id}` prevents race conditions when a customer accepts and pays the deposit simultaneously.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Owner
      participant SalesAgent as Sales & Revenue Agent
      participant CoreDB as PostgreSQL (Tenant Isolated)
      participant Stripe as Stripe API
      actor Customer

      Owner->>SalesAgent: "Draft quote: John, roof repair, $500, 50% deposit"
      SalesAgent->>CoreDB: Query John's CRM record & Tax Settings
      SalesAgent->>SalesAgent: Structure line items & draft professional message
      SalesAgent->>Owner: Push Agent Feed Card (Review Proposal)
      Owner->>SalesAgent: Taps "Approve & Send"
      SalesAgent->>CoreDB: Save Proposal (Status: Sent)
      SalesAgent->>Customer: Email/SMS with secure mobile Web link
      Customer->>Stripe: Views Proposal & Pays Deposit (Tap-to-Pay/Apple Pay)
      Stripe-->>CoreDB: Webhook: Payment Intent Succeeded
      CoreDB->>SalesAgent: Event: Proposal Accepted
      SalesAgent->>Owner: Push Agent Feed Card ("John paid deposit. Schedule work?")
  ```

  ### Mobile UX Flow (375px First)
  1. **Intake (Owner)**: A conversational input or quick-add form in the Agent Feed ("New Quote").
  2. **Approval Card (Owner)**: A unified card displaying the generated quote total, customer name, and drafted email text. Actions: [Approve & Send] [Edit] (Large 44x44px touch targets).
  3. **Customer View (Web)**: A clean, 375px-optimized translucent glass-styled page showing the breakdown, terms, and a sticky bottom bar: [Accept & Pay Deposit].
  4. **Success State**: Smooth transition to a confirmation screen, triggering the real-time webhook back to the owner's Agent Feed.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant**: Parses the unstructured input into structured PostgreSQL line items and drafts the customer-facing email.
  - **Operations Assistant**: Monitors the `ProposalAccepted` event to prompt the owner to schedule the service block.
  - **Knowledge & Compliance Assistant**: Automatically appends the correct liability waiver or terms of service based on the line items (e.g., "roof repair" triggers the "hazardous work" addendum).

  ## Implementation Prompt
  Implement the Agentic Smart Quote & Proposal Generation System.
  1. Define the PostgreSQL schema for Proposals and LineItems with RLS multi-tenancy.
  2. Create the backend gRPC/REST endpoints to support CRUD operations on Proposals.
  3. Implement the generative AI prompt chain for the Sales Agent to parse informal text into a structured Quote.
  4. Build the Flutter mobile-first (375px) UI components for the Owner's Agent Feed Approval Card and the public-facing Customer Proposal View using the OHC Premium Token translucent design system.
  5. Add Playwright E2E tests verifying the full flow from unstructured prompt to customer payment acceptance.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
