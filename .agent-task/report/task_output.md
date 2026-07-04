issue_title: "Automated Quote-to-Cash Agentic Orchestration"
issue_description: |
  # Research Report: Automated Quote-to-Cash Agentic Orchestration

  ## Executive Summary
  This report investigates the architectural gaps in the quote-to-cash workflow for service-based small businesses (e.g., repair, home improvement, tutoring). Our research indicates that while OHC has the foundational data models (Quote, Invoice, DepositRequirement, Booking), the end-to-end flow requires manual intervention. The goal is to design an agent-driven architecture that seamlessly transforms a customer lead into an approved quote, an automated deposit requirement, and a scheduled booking, requiring only a single tap of approval from the business owner.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Jobber, Housecall Pro, and Thumbtack dominate the field service software market. However, these platforms function primarily as CRMs—they require the owner to manually read a message, draft an estimate, send it, check for a response, issue an invoice, and confirm a booking slot.
  By contrast, AI-native solutions are beginning to emerge, but they often lack deep operational integration (e.g., they can draft an email but cannot lock a calendar slot and enforce a Stripe Terminal deposit). OHC has the opportunity to unify this through "The Operations Agent" and "The Sales Agent".

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Carlos (Field Service Owner, 42) and Leo (Music Tutor, 22). Carlos runs a repair business from his Android phone and needs service requests, estimates, bookings, and deposit collections in one flow.
  - **The Gap:** Currently, an incoming lead from an online form or message does not automatically trigger the Quote-to-Cash pipeline. The `ServiceLead` exists, and a `Quote` can be created, but there is no AI agent coordination to proactively draft the `Quote`, propose a `Booking` slot, and generate a `DepositRequirement` for the customer. The burden of administrative state transition rests on the human owner.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC Triage Agent
      participant OHC Sales Agent
      participant OHC Operations Agent
      participant PostgreSQL Ledger
      participant Owner Mobile (375px)

      Customer->>OHC Triage Agent: Sends request (e.g., "Need sink fixed on Tuesday")
      OHC Triage Agent->>OHC Sales Agent: Classifies as ServiceLead, requests Quote
      OHC Sales Agent->>PostgreSQL Ledger: Drafts Quote & DepositRequirement
      OHC Sales Agent->>OHC Operations Agent: Requests proposed Time Slots
      OHC Operations Agent->>PostgreSQL Ledger: Checks AvailabilityBlock, locks provisional slot
      OHC Sales Agent->>Owner Mobile (375px): Push Notification: "Review $150 Sink Repair Quote"
      Owner Mobile (375px)->>OHC Sales Agent: Taps "Approve & Send"
      OHC Sales Agent->>Customer: Sends Quote + Payment Link + Proposed Time
      Customer->>PostgreSQL Ledger: Pays Deposit via Stripe
      PostgreSQL Ledger->>OHC Operations Agent: Webhook triggers PaymentEvent
      OHC Operations Agent->>PostgreSQL Ledger: Converts provisional slot to Booking
      OHC Operations Agent->>Owner Mobile (375px): Notification: "Sink Repair Booked for Tuesday"
  ```

  ### Mobile UX Flow (375px First)
  - **Lead Triage Inbox:** The owner sees a consolidated feed of incoming leads. Each card contains the customer's intent, the drafted quote, and the provisional calendar slot.
  - **Action Card:** The card has primary buttons that are at least 44x44px.
    - `[ Approve & Send Quote ]` (Green, prominent)
    - `[ Edit Draft ]` (Secondary, translucent glass style)
  - **Offline/Resilience:** If the network is flaky, approving the quote optimistically updates the UI and queues the backend dispatch worker.

  ### AI Agent Integration Points
  - **Work Triage:** Parses the incoming text/DM into a structured `ServiceLead`.
  - **Sales & Revenue Assistant:** Uses the `ServiceLead` details and the business's `Service` catalog (price_cents) to draft a `Quote` and a `DepositRequirement`.
  - **Operations Assistant:** Queries the `AvailabilityBlock` table to find overlapping free time and proposes a `Booking` slot, attaching it to the `Quote`.

  ## 4. Implementation Prompt
  **Feature Name:** AI-Orchestrated Quote-to-Cash Pipeline
  **Target Persona:** Carlos the Handyman

  **Outcome:** When a customer requests a service, the system automatically drafts a Quote, a Deposit Requirement, and a provisional Booking slot. Carlos only needs to tap "Approve" on his Android phone to send it all to the customer. Once the customer pays the deposit, the booking is automatically finalized.

  **Critical User Journey (CUJ):**
  1. A new `ServiceLead` is created via the public API (e.g., from an Instagram DM parsed by the Triage agent).
  2. The Sales Agent detects the new lead, matches it to a known `Service` (e.g., "Plumbing Repair"), and drafts a `Quote` and `DepositRequirement`.
  3. The Operations Agent drafts a provisional `Booking` slot based on Carlos's calendar availability and attaches it to the quote.
  4. Carlos opens the OHC mobile app, sees the Action Card in his feed, and taps "Approve & Send".
  5. The quote is emailed/messaged to the customer with a Stripe Checkout link.
  6. The customer pays the deposit. A webhook triggers the Operations Agent to confirm the `Booking` and transition the `ServiceLead` to "Won".

  **Acceptance Criteria:**
  - Create the orchestration service that chains `ServiceLead` -> `Quote` -> `Booking` -> `DepositRequirement`.
  - Build the mobile-first (375px) React/Flutter action card component to display the drafted pipeline and accept owner approval.
  - Write Playwright E2E tests validating the full pipeline from lead creation to the approval tap in the UI, and the final state transitions upon simulated payment.
  - Do NOT prescribe the exact API signatures; design them to fit the existing gRPC/REST patterns.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
