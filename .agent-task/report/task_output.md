issue_title: "[Architecture] Autonomous AI Scheduling & Booking Agent"
issue_description: |
  # [Architecture] Autonomous AI Scheduling & Booking Agent

  ## Problem Statement
  For service-based owners like **Leo (Music Tutor)** or **Carlos (Handyman)**, scheduling is the most high-friction part of their day.
  Currently, they must answer a text message or email, manually check their calendar, propose 3 times, wait for the customer to reply, manually block the calendar, and manually send a deposit link. If a customer reschedules, the whole painful loop starts again.

  Competitors like Calendly require customers to click a link, view a grid of times, and fill out a form, which often breaks the conversational flow for small businesses operating via Instagram DMs or SMS. Owners want the OHC AI assistant to just handle the back-and-forth conversation, negotiate a time based on their availability, block the calendar, and collect the deposit—invisibly.

  ## Research Report
  ### Competitive Landscape
  *   **Calendly / Acuity:** Standard link-based booking. Functional but sterile. Requires context switching for the customer (from DM to web browser).
  *   **Shopify Bookings Apps:** Clunky, bolted-on experiences not native to the core platform.
  *   **OHC Opportunity:** "Conversational Scheduling." Because OHC already has AI agents that read inbound messages (Work Triage), we can give the agent direct access to read the owner's availability and write calendar holds. The AI acts exactly like a human receptionist negotiating a time over chat, and only sends a link when it's time to pay the deposit.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OHC_Inbox
      participant CustomerServiceAgent
      participant OpsAgent
      participant CalendarDB
      participant Stripe

      Customer->>OHC_Inbox: "Do you have time to fix my sink this week?"
      OHC_Inbox->>CustomerServiceAgent: New inbound message
      CustomerServiceAgent->>OpsAgent: Request availability for Service (Sink Repair, 2 hours)
      OpsAgent->>CalendarDB: Query free slots for Carlos
      CalendarDB-->>OpsAgent: Returns [Tue 2pm, Wed 10am]
      OpsAgent-->>CustomerServiceAgent: Slots available
      CustomerServiceAgent->>Customer: "Carlos can come by Tuesday at 2pm or Wednesday at 10am. Do either work?"
      Customer->>OHC_Inbox: "Tuesday at 2pm is perfect."
      OHC_Inbox->>CustomerServiceAgent: Parse confirmation
      CustomerServiceAgent->>OpsAgent: Book Tue 2pm
      OpsAgent->>CalendarDB: Create HOLD (Pending Deposit)
      OpsAgent->>Stripe: Generate Payment Link for $50 Deposit
      Stripe-->>CustomerServiceAgent: Link URL
      CustomerServiceAgent->>Customer: "Great! I've held that spot. Please pay the deposit here to confirm: [Link]"
  ```

  ### Mobile UX Flow (375px First)
  **For the Owner (Carlos):**
  1. **Daily View:** Carlos opens OHC. His "Today" view shows a clean chronological feed.
  2. **Pending Actions:** A translucent card shows: "The AI scheduled a Sink Repair for Tuesday 2pm with John. Waiting on $50 deposit."
  3. **Calendar Setup:** In Settings, Carlos simply toggles his "Working Hours" and connects his Google/Apple calendar. No complex rule building required.

  **For the Customer:**
  1. The entire interaction happens inside their preferred channel (SMS, Instagram DM, Web Chat).
  2. The AI uses natural language. No forms or grids unless they explicitly ask for a calendar link.

  ### AI Agent Integration Points
  *   **Customer Service Agent:** Handles the natural language understanding. Needs a new tool: `CheckAvailability(service_id, date_range)`.
  *   **Operations Agent:** Manages the underlying calendar ledger. Needs tools: `CreateHold(service_id, time_slot)` and `ConfirmBooking(hold_id)`.

  ### Key Design Decisions
  *   **Holds vs. Confirmed:** The system must support "Holds". A time slot is held when the customer agrees, but only transitions to "Confirmed" when the deposit is paid via Stripe. The AI will automatically release holds after 24 hours if unpaid and follow up with the customer.
  *   **Unified Calendar Ledger:** The internal OHC calendar (`CalendarDB` in PostgreSQL) must be the source of truth, syncing bidirectionally with external calendars (Google/Apple) to prevent double-booking.

  ## Implementation Prompt
  **To the Implementer:**
  Implement the backend core for the "Conversational Scheduling Engine".

  1. Create a `calendar` service module.
  2. Define the database schema for `AvailabilityBlocks`, `ServiceDurations`, and `CalendarEvents` (with states: `Hold`, `Confirmed`, `Cancelled`). Ensure strict multi-tenant isolation.
  3. Implement the API endpoints/gRPC methods for the AI Agents to query availability (`GET /v1/calendar/availability`) and create holds (`POST /v1/calendar/hold`).
  4. Implement the logic to automatically release unconfirmed holds after a configured timeout (e.g., via the Hybrid Task Scheduler).
  5. Provide 100% unit test coverage for the scheduling conflict resolution logic. Do not build the frontend UI yet.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
