issue_title: "AI-Powered Field Service Quoting & Instant Deposit Engine"
issue_description: |
  # Research Report: AI-Powered Field Service Quoting & Instant Deposit Engine

  ## 1. Problem Statement
  Field service operators (e.g., Carlos the Handyman) struggle with quoting jobs and securing commitments while on the move. They typically rely on disconnected tools or expensive vertical SaaS (like Jobber or Housecall Pro) to manage estimates, scheduling, and payments. These platforms are often complex, requiring desktop setup, and their mobile apps function as admin portals rather than assistive tools. Crucially, capturing leads via text/WhatsApp and turning them into paid deposits requires manual intervention, causing lost revenue.

  ## 2. Research Report
  - **Market Context**: Platforms like Jobber and Housecall Pro dominate the home service sector. They provide estimates, scheduling, and invoicing, but they lack autonomous lead capture and proactive engagement. They act as a digital filing cabinet.
  - **The OHC Opportunity**: By integrating a conversational AI agent (The Sales/Operations Agent) with a dynamic quoting system and immediate Stripe payment links, OHC can turn a simple text message from a lead into a scheduled, deposit-paid job autonomously.
  - **Competitor Gaps**:
    - *Jobber/Housecall Pro*: Powerful but complex, expensive ($100+/mo), and passive (wait for the owner to draft the quote).
    - *Square Appointments*: Good for simple bookings but lacks robust custom quoting for unpredictable service jobs.
    - *Shopify/Wix*: Not built for service businesses; poor handling of variable pricing and field scheduling.

  ## 3. Design Doc
  ### Architecture & Data Model (PostgreSQL)
  - `ServiceLead`: Represents an incoming request (e.g., via SMS, WhatsApp, or Web Form) containing images or descriptions of the problem.
  - `Estimate`: A proposed scope of work and variable price range, linked to a `ServiceLead` and an `AvailabilityBlock`.
  - `DepositRequirement`: Defines the upfront payment needed to convert an `Estimate` into a confirmed `Booking`.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Integrations (SMS/WhatsApp)
      participant Sales Agent ("The Estimator")
      participant Operations Agent ("The Dispatcher")
      participant OHC Mobile App
      participant Stripe
      participant Database (Tenant DB)

      Customer->>OHC Integrations (SMS/WhatsApp): Sends Lead & Photos
      OHC Integrations (SMS/WhatsApp)->>Sales Agent ("The Estimator"): Webhook Event
      Sales Agent ("The Estimator")->>Database (Tenant DB): Analyzes Lead & Reads Historical Pricing
      Sales Agent ("The Estimator")->>Database (Tenant DB): Creates `ServiceLead` & Draft `Estimate`
      Sales Agent ("The Estimator")->>OHC Mobile App: Push Notification
      OHC Mobile App-->>Sales Agent ("The Estimator"): Carlos taps "Approve & Send"
      Sales Agent ("The Estimator")->>Customer: Sends SMS with Deposit Link
      Customer->>Stripe: Pays Deposit
      Stripe->>Operations Agent ("The Dispatcher"): Payment Success Webhook
      Operations Agent ("The Dispatcher")->>Database (Tenant DB): Creates `Booking` & Updates `Estimate`
      Operations Agent ("The Dispatcher")->>Customer: Sends Confirmation SMS
      Operations Agent ("The Dispatcher")->>OHC Mobile App: Push Notification: "Job Booked"
  ```

  ### AI Agent Integration
  - **Sales Agent ("The Estimator")**:
    - Ingests incoming lead messages and images (using Gemini Vision).
    - Classifies the job type and matches it against Carlos's standard rate card or historical jobs.
    - Drafts an `Estimate` with a price range and available dates.
    - Presents the draft to Carlos for one-tap approval on mobile.
  - **Operations Agent ("The Dispatcher")**:
    - Once the estimate is approved by the owner and the customer pays the deposit, it automatically schedules the job and blocks the calendar.
    - Sends automated day-before reminders with arrival windows.

  ### Mobile UX Flow (375px First)
  1. **Lead Ingestion**: Carlos receives a push notification: "New Lead: Broken Pipe under sink. AI has drafted an estimate."
  2. **Approval View**: A clean, 375px card shows the customer's photo/message, the AI-suggested price ($150-$250), a 20% deposit requirement, and suggested slots.
  3. **Action**: Carlos taps "Approve & Send Link".
  4. **Customer View**: Customer receives an SMS with a lightweight mobile web link to view the estimate, pick a time slot, and pay the deposit via Apple/Google Pay (Stripe).
  5. **Confirmation**: Carlos gets a notification: "Job booked & deposit secured. Added to calendar for Tuesday at 10 AM."

  ## 4. Implementation Prompt
  **Feature Name**: Autonomous Field Service Quoting & Deposit Engine
  **Target Persona**: Carlos the Handyman

  **Outcome**: Carlos can receive a text message from a potential client, have the AI instantly draft an estimate based on his historical pricing, and send a one-tap approval to the client that includes scheduling and deposit collection.

  **Next Actions for Engineering**:
  1. Implement the core Data Models (`ServiceLead`, `Estimate`, `DepositRequirement`) with strict multi-tenant isolation.
  2. Develop the "Estimator" AI Agent workflow using Gemini to parse incoming text/images and draft variable-priced estimates.
  3. Build the Mobile-First (375px) "Approval Card" UI for the owner dashboard, ensuring clear visibility of the drafted quote and a prominent "Approve & Send" button.
  4. Create the Customer-facing Estimate Approval & Checkout flow, integrating Stripe Checkout Sessions for deposit collection and updating the `Booking` state upon success.
  5. Implement E2E Playwright tests simulating Carlos receiving a lead, approving the quote, and the customer paying the deposit.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
