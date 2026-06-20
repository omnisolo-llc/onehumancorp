issue_title: "Implement Unified AI Agentic Booking & Scheduling Engine with Edge-Sync"
issue_description: |
  ## Problem Statement
  For service-based owners like Leo (music tutor) and Carlos (field service), scheduling is their core product, yet existing SMB tools handle booking as a disconnected add-on. Competitors like Shopify require third-party apps for appointments, while Wix and Squarespace offer rigid, siloed calendars. These legacy systems fail to coordinate seamlessly with inventory, route planning, agentic customer communication (e.g., automated follow-ups), and multi-tenant conflict resolution. The result is double-booking, complex manual setup, and poor mobile experiences.

  ## Research Report
  - **Market Context:** Our competitive audit of standard SaaS booking solutions reveals a heavy reliance on manual administrative setup. Users must explicitly define schedules, services, and exceptions.
  - **The "Shopify/Wix" Tax:** Existing platforms treat time-slots merely as digital products. They do not automatically cross-reference constraints like staff availability or travel time.
  - **OHC Opportunity:** By introducing an AI Agentic Booking Engine, OHC can dynamically handle complex scheduling constraints. The "Operations Agent" can negotiate time slots directly with customers via chat, manage Edge-Sync offline reservations (similar to Redis Redlock for inventory), and intelligently optimize routes.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Owner View:** The OHC home screen shows a unified "Today's Work Feed".
  2. **Booking Capture:** The AI Customer Assistant intercepts an Instagram DM ("Can you fix my sink tomorrow at 2 PM?") and automatically generates a proposed booking card with estimated travel time.
  3. **Approval:** The owner taps a single "Approve & Send Deposit Link" button (minimum 44x44px touch target). No manual calendar entry is required.
  4. **Customer View:** The customer receives a localized, edge-cached confirmation page with a seamless Stripe checkout session for the deposit.

  ### AI Agent Integration
  - **Customer Success Agent:** Translates informal booking requests (DMs, emails) into structured `BookingIntent` events.
  - **Operations Agent ("The Dispatcher"):** Evaluates `BookingIntent` against the PostgreSQL central ledger, calculating transit times, staff schedules, and existing commitments.
  - **Finance Agent:** Issues deposit payment links and tracks fulfillment upon job completion.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant CustomerAgent as Customer Success Agent
      participant OpsAgent as Operations Agent
      participant Redis as Redis (Edge Lock)
      participant DB as PostgreSQL (Ledger)
      participant Owner as Owner (Mobile UI)

      Customer->>CustomerAgent: "I need a lesson Tuesday morning."
      CustomerAgent->>OpsAgent: Parse Intent & Query Availability
      OpsAgent->>DB: Check Calendar & Constraints
      OpsAgent->>Redis: Acquire Provisional Lock (10 mins)
      OpsAgent->>Owner: "Draft Booking: Tuesday 10 AM. Approve?"
      Owner->>OpsAgent: Taps "Approve"
      OpsAgent->>Customer: Sends Deposit Link
  ```

  ## Implementation Prompt
  **Target Persona:** Leo the Music Tutor & Carlos the Field Service Owner
  **Objective:** Implement the backend architecture and mobile-first UI for the Unified Agentic Booking Engine.
  **Requirements:**
  1. Define the `Booking` and `ScheduleConstraint` data schemas in PostgreSQL with strict `tenant_id` row-level security.
  2. Implement a Redis-backed distributed lock mechanism (`ohc:lock:{tenant_id}:booking:{time_slot}`) to prevent double-booking during agent negotiations.
  3. Create the "Operations Agent" department logic to evaluate availability, taking into account overlapping events and buffer times.
  4. Build the mobile-first (375px) "Draft Booking Approval" card for the owner's unified feed, ensuring all touch targets meet the 44x44px minimum and it utilizes the macOS-style Translucent Glass design tokens.

  ## Priority & Scope
  - **Priority:** P1 (High)
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
