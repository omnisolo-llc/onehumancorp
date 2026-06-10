issue_title: "AI-Native Dynamic Quoting & Deposit Generation System"
issue_description: |
  # Research Report: AI-Native Dynamic Quoting & Deposit Generation System

  ## 1. Problem Statement
  **Target Persona:** Carlos (Field Service Owner - Handyman, 42).
  **The Gap:** Carlos receives service inquiries via text or Instagram DMs. To secure a job, he currently has to manually respond, assess the work, create an estimate in a separate app (like Jobber or Joist), send it to the customer via email or SMS, wait for approval, and then send a separate Stripe or PayPal link to collect a deposit. This multi-app process is high-friction, takes too long (causing lost leads), and relies heavily on Carlos's manual data entry while he is trying to work in the field.

  **The Vision:** OHC needs an AI-Native Quoting system. When an inquiry comes in, the Sales/Operations Agent should be able to instantly parse the request, draft a formalized quote (including estimated parts and labor), and generate a secure checkout link for a deposit—all within the same conversation thread, requiring only a single tap of approval from Carlos.

  ## 2. Research & Competitive Analysis
  We researched leading field service and quoting tools:
  - **Jobber / Joist / Housecall Pro:** Excellent for managing complex service businesses but rely heavily on manual form filling. They offer quote generation and deposit collection but treat them as rigid, distinct steps rather than conversational fluid actions.
  - **HoneyBook / Dubsado:** Great for creative professionals, combining proposals and invoices, but setup is extremely complex and not suitable for quick, mobile-first field service quoting.
  - **Shopify / Square:** Excellent for product sales but lack the dynamic nature required for custom service estimates (labor + materials + travel).

  **OHC's AI-Native Advantage:** By leveraging the Agent framework, OHC can eliminate the "create quote" form. The AI acts as the estimator, pulling from Carlos's past jobs, standard pricing catalog, and the customer's text inquiry to draft the quote autonomously.

  ## 3. Design Doc

  ### Architecture & Data Model
  - **PostgreSQL Ledger:** A new `Quote` entity linked to `Tenant`, `Customer`, and a `DepositInvoice`.
  - **AI Agent Coordination:**
    - **Customer Assistant (The Ambassador):** Receives the initial message ("Can you fix my leaky sink this Tuesday?").
    - **Sales Agent (The Estimator):** Intercepts the intent, references the tenant's pricing catalog, and drafts a Quote (e.g., "$150 flat fee + parts. $50 deposit required").
    - **Operations Agent:** Checks the calendar for Tuesday availability and places a soft hold.
  - **Stripe Integration:** The quote generation simultaneously creates a Stripe Payment Link or Checkout Session for the deposit amount.

  ### Sequence Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      actor Carlos (Owner)
      participant Ambassador as Customer Assistant
      participant Estimator as Sales Agent
      participant OHC as Core Backend (DB)
      participant Stripe as Stripe API

      Customer->>Ambassador: "Need my sink fixed on Tuesday. How much?"
      Ambassador->>Estimator: Request quote draft for "sink repair"
      Estimator->>OHC: Fetch pricing & calendar availability
      OHC-->>Estimator: Standard sink repair: $150, Tuesday PM open
      Estimator->>Stripe: Generate Deposit Payment Link ($50)
      Stripe-->>Estimator: Link URL
      Estimator->>Ambassador: Draft Quote & Reply text
      Ambassador->>Carlos: Push Notification: "Review Draft Quote for Sink Repair"
      Carlos->>Ambassador: Taps "Approve & Send"
      Ambassador->>Customer: Sends Quote + Deposit Link
  ```

  ### Mobile-First UX Flow (375px)
  1. **Notification Card:** Carlos receives a push. Tapping it opens a clean, unified view.
  2. **Quote Preview:** A translucent glass card showing the draft message to the customer, and a breakdown of the estimate (Labor, Parts, Deposit).
  3. **One-Tap Action:** A large, accessible (min 44x44px) "Approve & Send" button at the bottom of the screen.
  4. **Customer View:** The customer clicks the link and sees a mobile-optimized quote page with a "Pay Deposit" button integrated directly with Stripe.

  ## 4. Implementation Prompt for Engineering Swarm

  **Feature:** Autonomous Quote & Deposit Generation
  **Persona:** Carlos (Field Service)

  **Critical User Journey (CUJ):**
  1. Login to OHC as Carlos.
  2. Simulate receiving a new customer message requesting a service estimate.
  3. Verify the AI Agent automatically drafts a quote based on the inquiry.
  4. Review the quote draft in the mobile UI (375px width).
  5. Approve the quote.
  6. Verify a Stripe deposit link is generated and sent to the customer thread.
  7. Verify the Quote entity is persisted in the PostgreSQL database with correct tenant isolation.

  **Acceptance Criteria:**
  - Must include full PostgreSQL schema updates with row-level security for multi-tenancy.
  - The quoting flow must integrate with the existing Agent framework.
  - The mobile UX must have zero horizontal scroll on a 375px viewport.
  - Implement E2E Playwright tests covering the entire CUJ from inquiry to quote approval.
  - Do not prescribe specific function signatures; focus on the business outcome and robust tests.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
