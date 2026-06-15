issue_title: "[Research] AI-Powered Field Service Quoting & Scheduling Engine"
issue_description: |
  # Research Report: AI-Powered Field Service Quoting & Scheduling Engine

  ## Executive Summary
  This report investigates the current landscape of small business field service management (FSM), specifically addressing the pain points of operators who rely entirely on mobile devices and lack administrative staff. The objective is to design a unified quoting, scheduling, and invoicing architecture for OneHumanCorp (OHC) that leverages our AI agents to provide a seamless, real-time experience for non-technical users.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Jobber, Housecall Pro, and ServiceTitan dominate the FSM space. However, they are often too complex and expensive for micro-SMEs or solo operators. These tools frequently require a desktop computer for initial setup and dispatching. For solo operators, managing leads, generating quotes, scheduling visits, and collecting deposits while on the road or at a job site is cumbersome and error-prone.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Carlos (Handyman) requires an integrated service request, estimating, booking, deposit, and customer follow-up system that works flawlessly on an Android phone.
  - **The Gap:** Currently, OHC lacks a unified AI-driven intake and quoting engine. Carlos has to manually read a message, check his calendar, draft a quote, send a payment link, and then manually create a calendar event if the deposit is paid. This disjointed process leads to lost leads and double-bookings.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & System Protocol
  - **Service Requests & Estimates:** New entities in PostgreSQL to track lead intake (`ServiceRequest`) and generated quotes (`Estimate`).
  - **Calendar Integration:** A robust scheduling system that integrates with the central ledger. We utilize row-level locking or optimistic concurrency control for critical time-slot reservations.
  - **Payment Integration:** Stripe Payment Intents linked to `Estimate` acceptance for upfront deposits.

  ### AI Agent Coordination
  - **Work Triage Agent ("The Dispatcher"):** Ingests incoming leads from various channels (SMS, web form, WhatsApp). It extracts key details (job type, location, urgency) and creates a structured `ServiceRequest`.
  - **Sales Agent ("The Estimator"):** Analyases the `ServiceRequest` and Carlos's historical pricing. It drafts an `Estimate` and proposes available time slots based on calendar availability.
  - **Operations Agent ("The Coordinator"):** Once the customer accepts the quote and pays the deposit, this agent finalizes the booking, blocks the calendar, and sends a confirmation to the customer.

  ### Architecture & Sequence Diagram
  ```mermaid
  sequenceDiagram
      participant Customer as Customer (WhatsApp/Web)
      participant WorkTriage as Work Triage Agent
      participant Sales as Sales Agent
      participant Ops as Operations Agent
      participant DB as PostgreSQL (Ledger)
      participant Carlos as Carlos (Mobile App)

      Customer->>WorkTriage: Sends Inquiry
      WorkTriage->>DB: Create ServiceRequest
      WorkTriage->>Sales: Trigger Quoting
      Sales->>DB: Check Availability & Pricing
      Sales->>DB: Draft Estimate
      Sales->>Carlos: Push Notification (Draft Ready)
      Carlos->>Sales: Approve & Send
      Sales->>Customer: Send Estimate & Payment Link
      Customer->>Ops: Pays Deposit
      Ops->>DB: Finalize Booking & Lock Calendar
      Ops->>Carlos: Notify Booking Confirmed
  ```

  ### Mobile-First Implementation
  - Ensure the quoting and scheduling interface operates flawlessly on a 375px viewport.
  - Provide Carlos with a clear daily "Route & Work Feed" displaying today's jobs, pending quotes, and unread leads.
  - 1-tap approval for AI-generated quotes before they are sent to the customer.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** OHC AI-Powered Field Service Quoting & Scheduling

  **Target Persona:** Carlos the Handyman

  **Outcome:** A seamless workflow where an incoming lead automatically generates a structured request, a draft quote, and proposed time slots, requiring only 1-tap approval from Carlos on his mobile phone.

  **Critical User Journey (CUJ):**
  1. A customer sends a WhatsApp message: "Hi, I need two ceiling fans installed in my living room."
  2. The Work Triage Agent intercepts the message, creates a `ServiceRequest`, and extracts the intent.
  3. The Sales Agent drafts an `Estimate` (e.g., $150 per fan) and finds three available slots in Carlos's calendar next week.
  4. Carlos receives a push notification. He opens the OHC app (375px viewport) and sees a card: "New Request: Ceiling Fan Install. Draft Quote Ready."
  5. Carlos reviews the draft quote and proposed times, and taps "Approve & Send".
  6. The customer receives a professional quote with a Stripe deposit link and time selection.
  7. Upon customer payment and selection, the Operations Agent finalizes the booking on Carlos's calendar.

  **Next Actions for Engineering:**
  - **Step 1:** Implement the `ServiceRequest` and `Estimate` data schemas in PostgreSQL with multi-tenant row-level security.
  - **Step 2:** Develop the Work Triage Agent to ingest and parse incoming leads into structured data.
  - **Step 3:** Develop the Sales Agent to generate quotes and propose time slots based on calendar availability.
  - **Step 4:** Build the mobile-first (375px) "Work Feed" card UI for Carlos to review and approve draft quotes.

  ### Top 5 Codebase Inconsistencies Discovered
  1. **Scattered CLI logic**: Some scripts are in `./deploy/scripts` while Rust CLI code exists in `./src/cli`.
  2. **Inconsistent multi-tenancy documentation**: The `OHC_MULTITENANT` toggle exists in docker-compose but its downstream effect on PostgreSQL RLS logic isn't clearly documented in a central spot.
  3. **Fragmented UI directories**: Code references both `src/frontend` (in some hypothetical agent tasks) and `src/ui/next` or `src/ui/tauri` in reality.
  4. **Docker overrides**: There is an override file `docker-compose.override.yml` referenced but the logic for local development vs CI testing is not completely isolated.
  5. **Missing Playwright tests**: Playwright is configured in the root (`playwright.config.ts`) and `src/e2e/playwright` but `README.md` focuses more on the Agent Harness than UI automated testing.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
