issue_title: "Agentic Autonomous Native Booking & Resource Management System"
issue_description: |
  ## Mission Queue Protocol Brief

  ### 1. Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ### 2. Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ### 3. Design Doc
  #### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer UI] -->|Selects Slot| B(Availability API)
      B --> C{Operations Agent}
      C -->|Checks| D[(Central Ledger/Postgres)]
      C -->|Reserves| E[(Redis Redlock)]
      D --> F[Sales/CS Agent]
      F -->|Follow-ups| G[Notification/Communication Layer]
  ```

  #### Data Model & Invariants
  - `Service`: The type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., Leo) or physical space.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled).
  - **Multi-Tenant Rule**: Row-level isolation using `tenant_id`. Lock key pattern `ohc:lock:{tenant_id}:booking:{resource_id}`.

  #### AI Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  #### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets, at least 44x44px), and proceed to a deposit payment flow via Stripe Checkout.
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions in a clean Apple/Ubiquiti-style interface.

  ### 4. Implementation Prompt
  **Goal:** Build the Native Booking System backend and frontend to support seamless appointment booking and AI-driven re-engagement for service-based SMBs.
  **CUJ:**
  1. Owner configures a Service and sets their Availability.
  2. Customer visits the mobile-first storefront, selects a time block, and completes the deposit payment.
  3. The Operations Agent correctly processes the reservation and notifies the owner.
  4. The Sales Agent successfully identifies an overdue follow-up and suggests a drafted re-engagement message.
  **Acceptance Criteria:**
  - Robust schema with Row-Level Security for `tenant_id`.
  - Redis distributed locking correctly prevents double-booking for the same `resource_id` at the same time.
  - E2E Playwright test simulating a customer booking and owner verifying the appointment on a 375px viewport.
  - 100% backend unit test coverage for new domains.
  - All UI elements follow the translucent glass design system.

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []