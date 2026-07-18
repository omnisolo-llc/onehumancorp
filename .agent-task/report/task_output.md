issue_title: "Design: AI-Driven Service Quote & Deposit Engine for Field Service Providers"
issue_description: |
  # Title
  AI-Driven Service Quote & Deposit Engine for Field Service Providers

  # Problem Statement
  Field service owners like Carlos (handyman, 42) rely on direct customer communication (calls/DMs/SMS) to capture demand. Creating manual quotes, checking calendar availability, and collecting deposits is a highly manual, multi-step process that requires jumping between a calendar app, messaging app, and payment platform. On a mobile device, this friction often leads to lost leads and double bookings. Traditional CRM solutions are too complex and desktop-centric. Carlos needs an AI assistant that can intercept a service request, generate a context-aware quote, hold a calendar slot, and collect a deposit—all through an automated, mobile-first workflow.

  # Research Report
  ## Competitive Analysis
  *   **Jobber & Housecall Pro**: Industry standards for field services, but they are heavy SaaS platforms that require extensive setup and are overly complex for solo operators. Their quoting flows are form-heavy and not conversation-first.
  *   **Durable & Wix**: Excellent for initial website creation but lack deep, automated back-office coordination for dynamic scheduling and deposit collection via chat/SMS.
  *   **Stripe Payment Links**: Great for simple checkout, but disconnected from scheduling and custom quoting.

  ## OHC Differentiator
  OHC will unify the inbox, calendar, and payments using the "Sales & Revenue Assistant" and "Operations Assistant" agents. The AI will parse the inbound message, draft a quote, generate a Stripe Payment Link with idempotency, and temporarily reserve a calendar slot using Redis distributed locks, presenting the owner with a single "Approve & Send" button on a 375px mobile screen.

  # Design Doc
  ## Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant WorkTriageAgent
      participant SalesAgent
      participant OperationsAgent
      participant DB (PostgreSQL)
      participant Cache (Redis)
      participant Stripe

      Customer->>WorkTriageAgent: SMS: "Need leaky faucet fixed on Tuesday"
      WorkTriageAgent->>SalesAgent: Parse request, determine service type
      SalesAgent->>DB: Fetch standard pricing for "Plumbing Repair"
      WorkTriageAgent->>OperationsAgent: Check calendar availability for Tuesday
      OperationsAgent->>Cache: Acquire Redlock for tentative Tuesday slot
      OperationsAgent-->>WorkTriageAgent: Slot reserved (15 min TTL)
      SalesAgent->>Stripe: Create Payment Link (Deposit)
      Stripe-->>SalesAgent: Payment URL
      SalesAgent-->>WorkTriageAgent: Quote & Deposit Link generated
      WorkTriageAgent->>Owner UI: Present Draft Quote Card (Mobile)
      Owner UI->>WorkTriageAgent: Owner clicks "Approve & Send"
      WorkTriageAgent->>Customer: SMS: "I can fix it Tuesday at 2 PM. Quote: $150. Deposit $50 here: [Link]"
  ```

  ## UI Wireframes & Mobile UX Flow (375px)
  1.  **Unified Feed (Home)**: Carlos opens the OHC app. A prominent translucent card at the top reads: *"New Request: Leaky Faucet (John Doe)"*.
  2.  **Draft Quote View**: Tapping the card opens a detail view. The UI shows the parsed context (Service: Plumbing, Date: Tuesday) and an AI-drafted reply with a generated quote ($150 total, $50 deposit).
  3.  **Action Bar**: A sticky bottom action bar contains a large primary button: "Approve & Send Quote" (touch target: 44px height minimum).
  4.  **Edit Mode (Fallback)**: If Carlos wants to adjust the price, tapping the price opens a native mobile numpad to quickly override the AI's estimate.

  ## AI Agent Integration Points
  *   **Work Triage Agent**: Intercepts the raw inbound text and categorizes it as a "Service Request".
  *   **Sales & Revenue Assistant**: Uses tenant-scoped memory to recall pricing for "leaky faucet" and interfaces with Stripe API to generate a checkout session for the deposit.
  *   **Operations Assistant**: Checks the tenant's calendar table and uses Redis (`ohc:lock:{tenant_id}:calendar:{timeslot}`) to prevent double booking while the quote is pending.

  ## Key Design Decisions and Why
  *   **Distributed Locking for Calendar Slots**: We use Redis Redlock to hold the timeslot for 15 minutes when a quote is drafted. If the deposit isn't paid or the owner rejects, the lock expires. This prevents double-booking in high-velocity scenarios.
  *   **Owner Approval Gate**: The AI drafts the quote but does not send it automatically. The owner must click "Approve & Send". This builds trust and ensures pricing accuracy for edge cases.
  *   **Mobile-First Numpad**: Price editing bypasses complex forms in favor of a direct tap-to-edit with a native numpad, ensuring 1-handed operation on the field.

  # Implementation Prompt
  **To the Implementer Agent:**
  Your objective is to implement the "AI-Driven Service Quote & Deposit Engine" targeting our field service persona, Carlos.

  **Critical User Journey (CUJ):**
  1.  Simulate an inbound customer message requesting a specific service (e.g., plumbing repair).
  2.  The backend must parse this message, identify the required service, and interact with a mock Stripe provider to generate a deposit payment link.
  3.  The backend must use a Redis lock to temporarily reserve the requested calendar slot.
  4.  On the frontend, render a mobile-optimized (375px) draft quote card containing the AI-generated response, the price, the deposit link, and an "Approve & Send" button.
  5.  When the owner clicks "Approve & Send", the quote is marked as sent and the system transitions to a pending payment state.

  **Acceptance Criteria:**
  *   Fully functional E2E flow from inbound request generation to quote approval.
  *   The UI must perfectly render on a 375px width, utilizing the OHC translucent design tokens. Touch targets for primary actions must be at least 44x44px.
  *   Redis locking must be correctly implemented and tested for race conditions.
  *   100% Unit and Playwright E2E test coverage.

  # Priority
  P1

  # Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
