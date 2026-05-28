issue_title: "[Architecture] Autonomous Smart Scheduling & Resource Booking Engine"
issue_description: |
  # [Architecture] Autonomous Smart Scheduling & Resource Booking Engine

  ## Problem Statement
  Small business owners like Leo (a music tutor) and Carlos (a handyman) rely heavily on scheduling their time. Currently, Leo has to manually coordinate with students to find open slots, negotiate times, and ensure he doesn't double-book himself across online and in-person lessons. Carlos needs to book home visits while accounting for travel time. They need a system where an AI agent can read their availability, negotiate with clients via chat, handle rescheduling automatically, and collect deposits upfront—all natively from their mobile phones, without jumping into an external calendar app.

  ## Research Report
  **Competitor Systems Audit:**
  - **Acuity / Calendly:** Powerful scheduling tools, but they exist as separate islands. Users must constantly sync them with their main CRM and payment processors.
  - **Wix Bookings / Squarespace Scheduling:** Built into the platform, but mostly rely on static booking pages where the customer picks a slot. They lack proactive AI agents that can negotiate times via conversational interfaces (like Instagram DMs or SMS).
  - **Shopify:** Primarily built for physical products; bookings require third-party apps which often break the native mobile experience and introduce extra monthly fees.

  **Gaps Identified:**
  OHC lacks a native, temporal, and capacity-constrained scheduling engine. To fully support service-based and booking-based solopreneurs, we need an architecture where time slots are treated as dynamic inventory. Furthermore, this system must integrate deeply with the Omni-Channel AI Inbox so AI agents can propose times, handle rescheduling, and enforce deposit requirements autonomously.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> CalendarUI[Glassmorphism Calendar & Booking UI];
          CalendarUI --> LocalCRDT[(Local SQLite Calendar Cache)];
      end

      App -- "Sync Booking State" --> Gateway[OHC API Gateway];

      Gateway --> SchedulingEngine[Temporal Scheduling Engine];
      SchedulingEngine --> MainDB[(Cloud Postgres Ledger)];
      SchedulingEngine <--> ExternalCalendars[Google/Apple Calendar Sync];

      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> SalesAgent[Sales: Propose Times & Collect Deposits];
          Agents --> CSAgent[Customer Success: Handle Rescheduling];
          Agents --> OpsAgent[Ops: Manage Capacity & Buffer Times];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Availability Setup:** Leo opens the app and sets his working hours and lesson durations with a few taps. The UI uses clean, Translucent Glass cards.
  2. **AI Negotiation:** A student messages Leo on Instagram asking for a lesson. The AI Sales Agent checks Leo's real-time availability and replies: "Leo is available Tuesday at 4 PM or Wednesday at 10 AM. Which works best for you?"
  3. **Booking Confirmation:** The student replies "Tuesday". The AI instantly locks the slot, sends a secure payment link for the deposit via the Instant Localized Invoicing engine, and updates Leo's calendar.
  4. **The Calendar View:** Leo opens the "Calendar" tab on his app. He sees his day clearly laid out with color-coded slots, showing which appointments are confirmed, pending deposit, or blocked for travel time. He can swipe right on a booking to view customer details.

  ### AI Agent Integration Points
  - **Sales Agent:** Has read access to the master calendar to propose available slots to leads in chat channels. Initiates the invoice flow for deposits.
  - **Customer Success (CS) Agent:** Handles requests like "Can we move my lesson to next week?" by checking the schedule, releasing the old slot, and booking the new one, all without bothering the human owner.
  - **Operations Agent:** Monitors buffer times and travel times. If Carlos books a job across town, the agent automatically blocks out the necessary travel time before and after the appointment.

  ### Key Design Decisions & Security
  - **Zero-Trust Tenant Isolation:** Calendar data is highly sensitive. All queries to the Scheduling Engine must be strictly scoped by the SPIFFE tenant identity.
  - **Offline-First Synchronization:** The calendar must use CRDTs so Carlos can check his schedule while in a client's basement without cell service.
  - **Unified Inventory:** A time slot is conceptually identical to a physical product in our system. It has stock (1 slot), price, and variants (duration).

  ## Implementation Prompt
  Implement the Autonomous Smart Scheduling & Resource Booking Engine.
  - **User-Facing Outcome:** Users can manage their availability, and AI agents can proactively negotiate and book appointments with customers over chat channels, collecting deposits securely. The calendar is perfectly synced and available offline on the mobile app.
  - **CUJ:** A customer messages the business. The AI reads the schedule, proposes times, and the customer agrees. The AI books the slot and sends an invoice for the deposit. The business owner opens their app and sees the new booking cleanly integrated into their daily agenda.
  - **Acceptance Criteria:**
    - Build a mobile-first Calendar UI adhering to the 375px glassmorphism design system.
    - Implement a backend scheduling engine that handles time zones, buffer times, and double-booking prevention.
    - Integrate the schedule state with the AI Swarm so agents can query availability and create bookings.
    - Support offline access to the schedule via local caching.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: "P0"
issue_category: "research"
issue_type: "task"
issue_label:
  - "agent-report"
assignees: []
