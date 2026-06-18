issue_title: "Implement OHC Native Agentic Booking System"
issue_description: |
  # Research Report: Autonomous Appointment Booking & Resource Management System

  ## 1. Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ## 2. Market Mapping & Competitor Discovery
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ## 3. Deep Dive Architecture Design
  ### Data Model (PostgreSQL)
  - `Service`: The type of appointment (duration, price, deposit required). Row-level tenant isolation using `tenant_id`.
  - `Resource`: The provider (e.g., Leo) or physical space. Row-level tenant isolation using `tenant_id`.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available. Row-level tenant isolation using `tenant_id`.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled). Row-level tenant isolation using `tenant_id`.

  ### Architecture Diagram
  ```mermaid
  erDiagram
    TENANT ||--o{ SERVICE : offers
    TENANT ||--o{ RESOURCE : employs
    TENANT ||--o{ CUSTOMER : serves
    SERVICE ||--o{ BOOKING : defines
    RESOURCE ||--o{ BOOKING : assigned_to
    CUSTOMER ||--o{ BOOKING : makes
    RESOURCE ||--o{ AVAILABILITY_BLOCK : has
  ```

  ### AI Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar). Triggers on incoming webhooks or scheduled intervals.
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link. Integrated with the Customer Assistant to draft replies for Instagram DMs and emails.

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets, 44x44px minimum), and proceed to a deposit payment flow (Stripe Checkout Sessions).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions in a clean Apple/Ubiquiti-style hierarchy.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Objective**: Build a native scheduling system integrated with OHC's backend that allows non-technical owners to manage service bookings, resource availability, and deposit payments without third-party apps, entirely from a 375px mobile screen.

  **Estimated Scope**: Large

  **Critical User Journey (CUJ)**:
  1. The owner (e.g., Leo) opens the app and sets up a new service ("1-Hour Drum Lesson") and their weekly availability.
  2. A customer visits the public OHC storefront (on mobile), selects the drum lesson, picks an available time slot, and pays the deposit via Stripe.
  3. The Operations Agent automatically creates the `Booking` record, blocks the time on the calendar, and sends a confirmation to the customer.
  4. The owner's unified feed updates with the new booking.

  **Acceptance Criteria**:
  - Implement full backend CRUD for Services, Resources, AvailabilityBlocks, and Bookings with row-level multi-tenant isolation.
  - Implement the Stripe Checkout integration for taking deposits during the booking flow.
  - Create the 375px-first mobile frontend for both the owner setup flow and the public customer booking flow.
  - Integrate the Operations Agent to listen for new bookings and update the calendar.
  - Ensure 100% unit test coverage and at least 5 Playwright E2E tests covering the complete CUJ.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
