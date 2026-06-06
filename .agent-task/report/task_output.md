issue_title: "Implement Autonomous AI Quote & Proposal Engine with Deposit Escrow"
issue_description: |
  ## Mission Queue Protocol Brief: Autonomous AI Quote & Proposal Engine

  ### Problem Statement
  For service-based business owners like **Carlos (The Freelance Handyman)** and **Maya (The Home Baker)**, a significant source of friction is the manual process of handling custom requests, estimating costs, drafting proposals, and collecting deposits. Currently, platforms like Shopify and Wix are built for fixed-price products, making custom services hard to manage.

  When a customer requests a "custom vegan wedding cake" or "plumbing fix for a leaky sink", the business owner spends hours going back and forth via DMs or email to figure out the scope, send a price, and chase a down payment. If the deposit isn't collected securely, the owner risks lost time and materials. They need a zero-touch, AI-driven way to convert a free-text customer request into a paid deposit securely, integrated directly into their existing platform.

  ### Research Report
  - **Competitor Analysis:**
    - **Shopify:** Primarily built for physical, standardized goods. Custom orders require third-party apps (e.g., Globo Request a Quote) which add $10-20/mo and complex configuration.
    - **Wix/Squarespace:** Offer basic contact forms, but linking a form submission to a dynamic payment intent and invoice requires manual backend work.
    - **HoneyBook/Dubsado:** Excellent for service providers, but they are separate CRM tools, meaning the owner has to sync them with their primary website, breaking the "all-in-one" OHC promise.
  - **User Persona Alignment:**
    - **Carlos:** Can have customers describe a repair issue. The AI Salesperson reads the description, estimates the standard labor time, and auto-generates a quote with a 30% deposit link.
    - **Maya:** Customers request a custom cake design. The AI reads the requirements (vegan, 3-tier), checks ingredient costs via the Operations agent, and sends a beautifully formatted proposal with a 50% deposit requirement to lock the date.

  ### Design Doc
  #### High-Level Architecture
  The system will introduce a new core entity, `Proposal`, which links a `CustomerInquiry` to a `PaymentIntent` (deposit) and an eventual `Order`/`Booking`. The AI Salesperson department will own the generation of these proposals.

  ```mermaid
  sequenceDiagram
      participant C as Customer (Web/Mobile)
      participant API as OHC API
      participant SalesAI as Sales AI Agent (The Salesperson)
      participant OpsAI as Ops AI Agent (The Manager)
      participant FinAI as Finance AI Agent (The Accountant)
      participant DB as Multi-Tenant Postgres

      C->>API: Submits Custom Request (Text/Images)
      API->>DB: Save CustomerInquiry
      API->>SalesAI: Trigger Quote Generation
      SalesAI->>OpsAI: Check availability / base pricing
      OpsAI-->>SalesAI: Return context
      SalesAI->>DB: Create Draft Proposal (Line items, Deposit %)
      SalesAI->>FinAI: Request Stripe Payment Link for Deposit
      FinAI-->>SalesAI: Return Payment Link
      SalesAI->>C: Auto-send Proposal via SMS/Email/DM
      C->>FinAI: Pays Deposit (Stripe)
      FinAI->>DB: Update Proposal to Accepted, create Order/Booking
      FinAI->>API: Notify Business Owner (Push)
  ```

  #### Mobile UX Flow (375px First)
  - **Customer View:**
    - Clean, full-screen glassmorphism form: "Tell us what you need." (Text area + photo upload).
    - Receives an SMS/Email with a link.
    - Link opens a minimalist Proposal card: Scope of work, Total Estimated Cost, "Pay $X Deposit to Lock" sticky bottom button.
  - **Business Owner View (OHC App):**
    - "Inbox" tab shows "New Quote Sent: $500".
    - Tap to view Proposal details. Options to manually "Edit Quote" or "Revoke".
    - Push notification: "Cha-ching! Maya, $250 deposit received for custom vegan cake."

  #### AI Agent Integration Points
  - **Sales & Acquisition ("The Salesperson"):** Parses the natural language request. Uses RAG against the business's past accepted quotes and base pricing list to generate accurate line items.
  - **Finance & Payments ("The Accountant"):** Generates the Stripe Payment Intent for the specific deposit amount and handles the webhook when the deposit clears.
  - **Operations ("The Manager"):** Blocks out the calendar date or reserves raw inventory once the deposit is paid.
  - **Legal & Compliance ("The Protector"):** Auto-appends standard terms of service (e.g., "Deposits are non-refundable within 48 hours of the event") to the proposal based on the business type.

  ### Implementation Prompt
  **To the Implementer Agent:**
  Implement the Autonomous Proposal Engine.
  1. Define the `Proposal` and `ProposalLineItem` schema in the multi-tenant PostgreSQL database with RLS.
  2. Create the internal gRPC and external REST endpoints to handle custom request submissions.
  3. Integrate with the `Sales` AI Agent department to intercept new requests, generate the proposal structure, and interface with the `Finance` agent to create a Stripe payment link for a calculated deposit percentage.
  4. Build the mobile-first (375px) React/Flutter frontend for the customer to view the proposal and pay the deposit (using standard OHC Translucent Glass tokens).
  5. Ensure zero mock data is used; the UI must reflect the actual proposal state from the database.
  6. Verify the flow end-to-end: Submit request -> AI generates quote -> Deposit paid -> Status updates.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []