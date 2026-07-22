issue_title: "Implement Automated Agentic Service Booking & Calendar Synchronization Engine"
issue_description: |
  # Research Report: Agentic Service Booking & Calendar Synchronization

  ## Title
  Implement Automated Agentic Service Booking & Calendar Synchronization Engine

  ## Problem Statement
  Service-based owners like Carlos (Handyman) and Leo (Music Tutor) lose time and money juggling disjointed booking systems, manual calendar updates, and text messages. While e-commerce platforms like Shopify focus on physical goods, service booking requires a "Tetris-like" coordination of time, travel, and availability. Existing solutions (Calendly, Acuity, Wix Bookings) require owners to manually set up complex rules and are often separate from their main payment or customer relationship systems. The owner just wants an assistant to handle the scheduling ping-pong, take a deposit, and put the confirmed job on their calendar seamlessly from their phone.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Primarily for physical goods; service bookings require clunky 3rd-party apps (e.g., Sesami, BookThatApp) that feel bolted on and break the native checkout flow.
  - **Wix/Squarespace:** They have built-in booking engines (Wix Bookings, Squarespace Scheduling/Acuity), but they require the owner to navigate complex desktop admin panels to manage staff availability, buffer times, and service rules.
  - **Calendly:** Great for standalone scheduling, but lacks deep integration with unified point-of-sale, cart, and full CRM history.
  - **OHC Opportunity:** Create an *agentic* booking engine where "The Operations Agent" acts as a virtual receptionist. It natively integrates with the Unified Calendar, understands travel time or buffer rules contextually without complex user setup, and coordinates directly with the customer via chat/inbox or a smart widget, requiring only a simple "Approve Booking" tap from the owner on a 375px mobile feed.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request via Web/DM] --> B(Omnichannel Gateway)
      B --> C{Operations Agent - The Manager}
      C -->|Query Availability| D[(Unified Postgres Calendar DB)]
      C -->|Draft Quote/Time| E[Action Required Queue]
      D -->|Check Lock| F[Redis Distributed Locks]
      E --> G[Mobile App Feed 375px]
      G -->|Owner Taps Approve| H[Booking Confirmed State]
      H --> I[Stripe Deposit/Payment Intent]
      H --> J[Customer Success Agent - The Ambassador]
      J -->|Send Confirmation/Reminder| B
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Home Feed (Mobile):** Top card shows "Booking Request: Leo's Guitar Lesson (Tues 4pm) from Sarah".
  - **Interaction:** Tapping the card expands the request. It shows the customer's history (Sarah is a returning student) and an AI-suggested response ("Tues 4pm works, I have added a 15m buffer before my next lesson. Total $50").
  - **Action:** A prominent primary button "Approve & Send Deposit Link" and a secondary "Propose New Time". Touch targets are 44x44px.
  - **Visual Design:** Translucent Glassmorphism cards on the feed, clean Ubiquiti-style modular layout. No desktop-only complex grid calendars on the main feed; time is presented linearly and relationally.

  ### AI Agent Integration Points
  - **Operations Agent (The Manager):** Triggered by an incoming scheduling request. It queries the PostgreSQL ledger for available time slots, applying implicit logic (like travel time or prep buffers) learned from the owner's past preferences rather than static form inputs.
  - **Customer Success Agent (The Ambassador):** Once the owner approves, this agent drafts and sends the deposit payment link and the calendar invite to the customer, and schedules a follow-up reminder 24 hours before the service.

  ### Key Design Decisions
  - **Agentic Configuration over Manual Setup:** Instead of a complex UI where Carlos defines "15 min travel time", the Agent manages buffers dynamically based on service location.
  - **Pessimistic Redis Locking for Time Slots:** Similar to inventory, a proposed time slot is temporarily locked in Redis (`ohc:lock:{tenant_id}:calendar:{time_block}`) when a quote is sent, preventing double booking during the negotiation phase.
  - **Unified Ledger:** Booking is treated as an "Inventory of Time", seamlessly mapping to the existing PostgreSQL multi-tenant architecture.

  ## Implementation Prompt
  **User-Facing Outcome:** When a customer requests a quote and time for a service (e.g., a home repair), the Operations Agent instantly checks the calendar, temporarily reserves the slot, and presents a drafted response with a deposit link in the owner's mobile feed. The owner taps "Approve" and the booking is finalized.
  **CUJ & Acceptance Criteria:**
  1. A scheduling request is ingested (via mocked API or webhook).
  2. The Operations Agent queries the Postgres database for availability and creates a temporary reservation lock in Redis.
  3. A drafted response and booking proposal appears as an action card on the 375px mobile UI.
  4. The owner taps "Approve" -> The state changes to confirmed, the Redis lock is consumed/converted into a permanent DB record, and a mocked Stripe payment link is generated.
  5. Provide Playwright E2E tests: A user logs in, sees the pending booking card, taps "Approve", and the system verifies the calendar DB is updated and a confirmation message is queued.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
