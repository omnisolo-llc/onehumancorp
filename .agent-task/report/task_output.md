issue_title: "Implement High-Scale Operations Sync for Autonomous Auto-Quoting"
issue_description: |
  # Research Report: High-Scale Operations Sync for Autonomous Auto-Quoting

  ## 1. Problem Statement
  Service-based small business owners like Carlos (Handyman) and Maya (Baker) spend an inordinate amount of time managing lead inquiries, pricing custom jobs, drafting quotes, and waiting for customer deposits. This manual "inbox to quote to payment" cycle is prone to delays, causing them to lose high-intent leads to faster competitors. Existing tools like Shopify or Wix are e-commerce-first and handle service bookings poorly, requiring a fragmented stack of CRMs, invoicing tools, and calendar apps.

  ## 2. Research Report
  - **Market Context**: Platforms like Jobber or Housecall Pro provide robust quoting tools for service businesses but require manual data entry by the owner. AI tools like HubSpot Breeze or Shopify Sidekick help with general marketing but do not integrate directly into the transactional quoting flow.
  - **The OHC Opportunity**: OHC can differentiate by offering an **Agentic Negotiator & Booker**. Instead of just providing a form for the customer to fill out, OHC provides an AI agent that converses with the lead, gathers job requirements, queries the owner's pricing model, drafts a quote, and requests a deposit.
  - **Competitor Gaps**:
    - *Shopify/Wix*: Lack native service quoting and negotiation capabilities.
    - *Jobber/Housecall Pro*: Require manual quote generation by the business owner.
    - *Durable*: Generates a website quickly but lacks deep operational automation for lead conversion.

  ## 3. Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Inbox
      participant Sales_Agent
      participant PostgreSQL
      participant Owner_Mobile

      Customer->>OHC_Inbox: "I need a leaky faucet fixed."
      OHC_Inbox->>Sales_Agent: Triage Message
      Sales_Agent->>PostgreSQL: Query service pricing (Plumbing)
      Sales_Agent-->>Sales_Agent: Draft Quote ($150, $50 deposit)
      Sales_Agent->>Owner_Mobile: Push: "Quote Drafted for Faucet Repair"
      Owner_Mobile->>Sales_Agent: Approve Quote
      Sales_Agent->>Customer: Send Quote Link (Stripe Checkout)
  ```

  ### Data Model (PostgreSQL)
  Leverage the existing `quotes` table but ensure it supports AI-generated metadata.
  - `Quote`: Contains `total_amount_cents`, `required_deposit_cents`, `status`, and a link to the `customer_id`.
  - Ensure multi-tenant isolation using the established `tenant_id` pattern.

  ### Mobile UX Flow (375px)
  1. **Owner Dashboard**: A unified feed showing "Pending AI Actions".
  2. **Quote Review Card**: A clear, mobile-optimized card displaying the customer inquiry, the AI's proposed quote amount, the deposit required, and a brief rationale.
  3. **One-Tap Actions**: Buttons for "Approve & Send", "Edit", and "Reject".

  ### AI Agent Integration
  - **Role**: The Sales/Operations Agent.
  - **Trigger**: New incoming message in the unified inbox categorized as a lead/inquiry.
  - **Action**: Uses RAG against the owner's past quotes and service pricing to draft a structured quote.

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Auto-Quoting Agent
  **Target Persona**: Carlos the Handyman
  **Outcome**: Carlos receives an SMS/Push notification that his AI assistant has drafted a $200 quote for a drywall repair lead. He reviews the breakdown on his phone, taps "Approve", and the system automatically emails the customer a payment link for the $50 deposit.

  **Critical User Journey (CUJ)**:
  1. A customer sends a message via the OHC hosted contact form: "Can you fix a hole in my drywall? It's about 2x2 feet."
  2. The Message Triage Worker identifies this as a lead.
  3. The Sales Agent drafts a quote in the PostgreSQL `quotes` table with status `DRAFT`.
  4. Carlos logs into the OHC mobile UI (375px) and sees a "Draft Quote" card at the top of his feed.
  5. Carlos taps "Approve".
  6. The system updates the quote status to `SENT` and triggers a Stripe payment link generation.

  **Acceptance Criteria**:
  - The UI must render correctly on a 375px screen with 44x44px touch targets.
  - Zero mock data; use the real database and AI provider.
  - Complete Playwright E2E test covering the customer inquiry -> AI draft -> Owner approval flow.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
