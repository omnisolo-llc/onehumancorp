issue_title: "[Architecture] Autonomous Yield Management Engine"
issue_description: |
  # Autonomous Yield Management Engine

  ## Problem Statement
  Small business owners in the services and bookings sector (e.g., Leo the Music Tutor, fitness studios, salons) often struggle with empty calendar slots and sub-optimal pricing. They lack the time and data analysis skills to dynamically adjust prices based on demand, seasonality, or last-minute availability. As a result, they leave significant revenue on the table and face inefficient capacity utilization. From a non-technical user's perspective, they just see "empty slots" and don't know how to fill them without manually emailing customers or dropping prices across the board, which devalues their service.

  ## Research Report
  - **Market Context**: Airlines and hotels use sophisticated yield management to maximize revenue per available unit. SMBs currently have no access to such tools.
  - **Competitor Analysis**:
    - **Shopify/Wix**: Focus on static product pricing. Any dynamic pricing requires complex third-party apps (e.g., Bold Custom Pricing) that are not tailored for services/bookings.
    - **Acuity/Calendly**: Allow for coupons but do not autonomously adjust prices based on real-time availability.
  - **OHC Opportunity**: OHC can differentiate by embedding an *Invisible AI Yield Manager* that autonomously adjusts service prices and triggers targeted promotions to fill unused capacity, requiring zero configuration from the business owner.

  ## Design Doc
  - **Architecture**:
    - **Data Model**: Extend the existing calendar/booking schema to track utilization rates per service type and time slot.
    - **AI Yield Agent**: A background worker (part of the Finance & Payments / Operations department) that continuously monitors booking velocity and upcoming availability.
    - **Pricing Rules Engine**: Simple, LLM-driven heuristics (e.g., "If tomorrow has >50% empty slots, offer a 15% discount to past customers who haven't booked in 30 days").
    - **Integration**: Plugs into the existing quoting and booking system.
  - **Mobile UX**:
    - 375px view: A single card on the dashboard showing "Yield Opportunities."
    - "Leo, you have 3 empty guitar slots tomorrow. Tap to send a 20% discount offer to your waitlist." (1-tap approval).
  - **Multi-Tenant Isolation**: Ensure yield rules and utilization data are strictly scoped per `tenant_id`.

  ## Implementation Prompt
  **Outcome**: Implement the backend logic and mobile-first UI for the Autonomous Yield Management Engine.
  **CUJ**:
  1. Leo (Music Tutor) logs into the OHC app.
  2. The dashboard displays a notification: "3 empty slots tomorrow. Send a 20% discount offer to 15 past students?"
  3. Leo taps "Approve."
  4. The system updates the price for those specific slots and dispatches notifications via the Ambassador Agent.
  **Acceptance Criteria**:
  - Backend worker capable of identifying low-utilization periods.
  - UI card component (375px optimized) for 1-tap approval.
  - E2E Playwright test verifying the approval flow and subsequent price adjustment.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
