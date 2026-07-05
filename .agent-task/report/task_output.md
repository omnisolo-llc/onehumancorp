issue_title: "Implement Native Agentic Booking System for Services"
issue_description: |
  # Native Agentic Booking System for Services

  ## Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website. This leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ## Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer/Leo UI 375px] --> B[API Layer]
      B --> C[PostgreSQL Ledger]
      B --> D[Event Mesh / Redis PubSub]
      C --> E{Data Models: Service, Resource, AvailabilityBlock, Booking}
      D --> F[Operations Agent]
      D --> G[Customer Success Agent]
      F --> |Manage Rescheduling/Sync| C
      G --> |Draft Re-engagement| A
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large 44x44px touch targets), and proceed to a deposit payment flow (Stripe).
  - **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests via natural language (e.g. from DMs), and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ### Key Design Decisions
  - Natively embedded into OHC to eliminate 3rd party apps and allow unified customer memory.
  - Mobile-first approach for booking and management.
  - Agentic automation instead of static scheduling rules.

  ## Implementation Prompt
  **User-Facing Outcome:** Leo the Music Tutor can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Critical User Journey & Acceptance Criteria:**
  1.  Create Data Models: `Service`, `Resource`, `AvailabilityBlock`, `Booking` with row-level tenant isolation in Postgres.
  2.  Develop Customer Booking Flow UI (mobile-first 375px) that integrates deposit payments via Stripe.
  3.  Extend Operations Agent: It should intercept rescheduling intents and output `Booking` updates.
  4.  Extend Customer Success Agent: Run a scheduled job to identify dormant customers and draft re-engagement messages in the owner's feed.
  5.  Write E2E Playwright tests covering a customer booking an appointment, and Leo approving a drafted re-engagement message.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
