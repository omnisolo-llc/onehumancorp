issue_title: "Agent-Driven Local Service Quoting & Dispatch Architecture"
issue_description: |
  # Research Report: Agent-Driven Local Service Quoting & Dispatch Architecture

  ## Problem Statement
  Field service owners like **Carlos (handyman)** operate entirely from their mobile devices while on the go. They rely on word-of-mouth and manual text messages for scheduling, quoting, and dispatching. The current market solutions (e.g., Jobber, Housecall Pro) are feature-heavy, requiring extensive setup and manual data entry that slows down a single operator. OHC needs a zero-setup, AI-driven architecture that turns casual customer inquiries (via SMS/WhatsApp/Web) into actionable quotes, deposits, and scheduled routes, all manageable from a 375px mobile viewport.

  ## Research Report & Competitive Analysis
  - **The Status Quo:** Platforms like Jobber and ServiceTitan require owners to create a customer profile, manually build a quote from a price book, send an email, and manually convert it to a scheduled job. This involves 10+ clicks and significant screen time.
  - **The OHC Opportunity:** Instead of the owner doing the data entry, the OHC **Sales Assistant Agent** and **Operations Assistant Agent** intercept the inbound demand, generate a probabilistic quote based on past jobs/parameters, and propose a calendar slot.
  - **Key Finding:** Field service operators lose 20-30% of leads due to slow response times while they are on a job site. An autonomous quoting and deposit-capture system would directly increase their revenue.

  ## Design Doc

  ### 1. Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Inquiry via SMS/Web] --> B(Unified Inbox / Work Triage)
      B --> C{Sales Agent}
      C -->|Drafts Quote based on Context| D[PostgreSQL: Quotes Table]
      D --> E[Stripe Payment Link for Deposit]
      C --> F[Owner Push Notification: 'Approve Quote?']
      F -->|1-Tap Approve| G(Send to Customer)
      G -->|Customer Pays Deposit| H(Operations Agent)
      H --> I[Schedule Job & Route]
      I --> J[PostgreSQL: Field Ops Calendar]
  ```

  ### 2. Mobile UX Flow (375px Target)
  1. **Inbound Feed:** Carlos opens the OHC app. The top card on his feed reads: "New lead: Sarah wants a ceiling fan installed. Draft quote ready."
  2. **Quote Review Card:** A simple, translucent Glassmorphism card shows:
     - **Service:** Ceiling Fan Installation
     - **Price:** $150
     - **Deposit Required:** $50
     - **Suggested Slot:** Tuesday 2 PM.
  3. **Action:** Carlos taps a single primary button: **"Approve & Send"**.
  4. **Confirmation:** The card transforms into a success state, and the Sales Agent takes over customer communication.

  ### 3. AI Agent Integration Points
  - **Work Triage:** Intercepts incoming messages, extracts intent (e.g., "ceiling fan"), and maps it to a standard service catalog.
  - **Sales Agent:** Queries historical data to estimate price. Creates a quote draft and an idempotent Stripe Payment Link for the deposit.
  - **Operations Agent:** Monitors the Stripe webhook. Upon deposit payment, it updates the quote status, allocates time on Carlos's calendar, and generates a routing task.

  ### 4. Key Design Decisions
  - **Optimistic UI:** Quote approval on mobile must feel instantaneous. We will use a local optimistic update while the background queue processes the actual external dispatch.
  - **Idempotency:** Deposit generation must be strictly idempotent to avoid double-charging if Carlos has a spotty cellular connection and taps "Approve" twice.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Implement the "1-Tap Quote & Deposit" mobile-first flow for field service operators.
  1. **Backend:** Extend the quoting and field operations services. Ensure the quoting data model supports a deposit paid status and links to a Stripe checkout session for deposit collection. Use a distributed lock mechanism to prevent double-booking the calendar slot when the deposit is paid. Let the implementer design the specific schemas and API endpoints.
  2. **Frontend:** Build a review draft quote card component using the OHC Premium Token library (translucent materials, clean hierarchy). The card must fit perfectly on a 375px screen without horizontal scrolling. The primary action is an "Approve & Send" button that submits the quote draft. Let the implementer design the specific function signatures and file locations.
  3. **Agent:** Hook the `Sales Agent` into the `Work Triage` event stream so that when a field service intent is detected, a draft quote is automatically generated and pushed to the owner's feed.
  4. **Tests:** Write comprehensive E2E tests that simulate a customer inquiry, verify the draft quote appears in the UI, simulate the owner tapping "Approve & Send", and confirm the system state transitions correctly. No mocked data in the UI.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
