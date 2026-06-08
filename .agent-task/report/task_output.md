issue_title: "Implement AI-Native Autonomous Booking & Scheduling System"
issue_description: |
  # Research Report: AI-Native Autonomous Booking & Scheduling System

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  The current market for scheduling software (Calendly, Acuity, Square Appointments, Booking.com) is heavily reliant on user-initiated actions. While they offer integrations and API access, they function as passive tools. The business owner must set up the rules, availability, buffers, and manual follow-ups. For service-based small businesses (Carlos the Handyman) and independent professionals (Leo the Music Tutor), this creates administrative overhead. The gap is not the lack of booking tools, but the lack of an *agent* that actively manages the calendar, negotiates times with clients, collects deposits, and follows up on incomplete bookings.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:**
    - **Carlos (Field Service Owner):** Needs to quote, book, and collect deposits from his Android phone while on a job. Current tools require him to open a separate app, generate a link, and send it, hoping the customer books.
    - **Leo (Creator and Tutor):** Needs to manage recurring lesson packages, handle reschedules gracefully without manual back-and-forth, and chase inactive students.
  - **The Gap:** OHC currently lacks an autonomous scheduling engine. While we may integrate with calendars, we need a native Operations Agent capable of bidirectional negotiation (e.g., "I'm free Tuesday at 2 PM or Wednesday morning. Do any of those work?") and full-cycle booking management (quote -> schedule -> deposit -> reminder) completely invisibly to the owner, unless an exception requires approval.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Architecture Diagram
  ```mermaid
  graph TD;
      Client[Customer Message / Web Form] --> Triage[Work Triage Agent];
      Triage --> |Identifies Booking Intent| BookingAgent[Operations Booking Agent];
      BookingAgent --> CalendarSync[Bi-directional Calendar Sync];
      BookingAgent --> |Check Availability| Ledger[(PostgreSQL Availability Ledger)];
      BookingAgent --> |Generate Options| ClientResponse[Draft/Send Reply via Integration];
      ClientResponse --> |Customer Selects Time| Lock[Redis Redlock Time Slot Reservation];
      Lock --> PaymentAgent[Sales & Revenue Agent - Deposit Request];
      PaymentAgent --> |Payment Confirmed| Confirm[Booking Finalized & Synced];
  ```

  ### Data Model & Invariants
  - **Availability Ledger:** A normalized PostgreSQL schema managing available slots, blocked times, and travel buffers (critical for Carlos).
  - **Redis Redlock:** Used during the booking negotiation phase to temporarily hold a slot (e.g., 5 minutes) while the customer completes the deposit.
  - **Agent State Machine:** The Booking Agent must track state (Negotiating, Pending Deposit, Confirmed, Rescheduling).

  ### Mobile UX Flow (375px)
  1.  **The "Action Needed" Card:** Carlos opens the OHC app. He sees a card: *"Agent tentatively booked a roof repair estimate for Sarah on Tuesday 2 PM. Pending $50 deposit. No action needed."*
  2.  **The "Approval" Card:** Leo opens the app. Card: *"Mark requested to reschedule his 4 PM lesson to 5 PM today. You have a conflict. Suggest tomorrow at 4 PM?" [Approve] [Edit] [Deny]*.

  ### AI Agent Integration
  - **Operations Assistant:** Handles the logic of finding open slots, calculating travel time (if applicable), and holding the inventory of time.
  - **Customer Assistant:** Generates the natural language text to negotiate the time with the client via SMS, WhatsApp, or Instagram DM.

  ## 4. Implementation Prompt (For Engineering Swarm)
  **Feature Name:** Autonomous Booking & Scheduling Engine

  **Target Persona:** Carlos (Field Service) & Leo (Tutor)

  **Outcome:** Provide a native booking engine where the Operations Agent can negotiate times, hold slots via Redis, request deposits, and finalize bookings via natural language or simple booking links, reducing owner administrative time to zero for standard bookings.

  **Critical User Journey (CUJ):**
  1.  A new lead texts Carlos: "Can you come look at my sink sometime next week?"
  2.  The Customer Assistant drafts a reply, querying the Operations Agent for availability: "I can stop by next Tuesday at 10 AM or Thursday at 2 PM. There is a $50 diagnostic fee. Which works for you?"
  3.  The Lead replies: "Tuesday at 10 AM is great."
  4.  The Operations Agent places a Redis lock on Tuesday at 10 AM and the Sales Agent generates a Stripe Payment Link.
  5.  Customer Assistant replies: "Great! Please pay the $50 diagnostic fee here to lock in the time: [Link]"
  6.  Upon payment webhook, the booking is finalized in the PostgreSQL ledger and Carlos receives a push notification on his Android phone: "New Booking: Sink Repair, Tuesday 10 AM. $50 deposit collected."

  **Acceptance Criteria:**
  - Implement the PostgreSQL Availability Ledger and Redis lock mechanism.
  - Agent must be able to parse relative time requests ("next week") and propose concrete slots.
  - End-to-end Playwright tests verifying the agent negotiation and deposit flow.
  - Mobile UI must display clear, non-technical Action Cards for the owner.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
