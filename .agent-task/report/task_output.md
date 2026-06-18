issue_title: "Implement Autonomous Agentic Booking System"
issue_description: |
  ## Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ## Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ## Design Doc
  ### Data Model (PostgreSQL)
  - `Service`: The type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., Leo) or physical space.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled).

  ### Architecture Flow Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      participant OHC_App as OHC Mobile Booking UI
      participant API as Backend (Go/gRPC)
      participant Operations_Agent as Operations Agent
      participant DB as PostgreSQL (Ledger)
      participant Payment as Stripe (Deposit)

      Customer->>OHC_App: Views available times
      OHC_App->>API: Fetch available slots
      API->>Operations_Agent: Request availability (Agent calculates dynamic slots)
      Operations_Agent->>DB: Query `AvailabilityBlock`
      DB-->>Operations_Agent: Returns blocks
      Operations_Agent-->>API: Returns optimized slots
      API-->>OHC_App: Displays slots
      Customer->>OHC_App: Selects slot & pays deposit
      OHC_App->>Payment: Process deposit
      Payment-->>OHC_App: Payment success
      OHC_App->>API: Confirm booking
      API->>DB: Save `Booking` state (Confirmed)
      API->>Operations_Agent: Notify booking created
      Operations_Agent->>Customer: Auto-send confirmation & details
  ```

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets ≥ 44x44px), and proceed to a deposit payment flow via Stripe.
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions (Action Cards).

  ### AI Agent Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests via natural language (intent classification), and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link. Sent for owner approval in the feed.

  ## Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Target Persona**: Leo the Music Tutor

  **User-Facing Outcome**: Leo can offer monthly lesson packages with an integrated booking calendar directly on his OHC profile. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Critical User Journey (CUJ)**:
  1. Leo opens the OHC mobile app and enables the Booking capability for his 'Piano Lesson' service.
  2. A student visits Leo's OHC link, selects a 45-minute slot from the mobile-optimized calendar, and pays a $20 deposit via Stripe.
  3. The booking is instantly confirmed, and the Operations Agent sends a calendar invite and zoom link to the student.
  4. The following week, the student forgets to book. The Sales Agent drafts a message: "Hi! Ready for our next lesson? Tap here to book your slot."
  5. Leo sees the drafted message as an Action Card in his mobile feed. He taps "Approve & Send".

  **Acceptance Criteria**:
  - The customer booking interface must be perfectly responsive and fully usable on a 375px viewport (no horizontal scroll, large touch targets).
  - Multi-tenant data isolation must be enforced on all new database models (`Service`, `AvailabilityBlock`, `Booking`).
  - Booking creation must be atomic, securely handling Stripe payment intent finalization and database insertion.
  - Implement full E2E Playwright tests covering the booking and deposit flow from the customer's perspective.
  - The feature must adhere to the Premium Token design system.

  **Priority**: P1
  **Estimated Scope**: Large

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
