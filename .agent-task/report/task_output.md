issue_title: "Implement OHC Native Agentic Booking System"
issue_description: |
  # Mission Queue Protocol Brief: OHC Native Agentic Booking System

  ## 1. Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ## 3. Design Doc
  ### High-Level Architecture
  The booking capability must natively integrate into the core multi-tenant backend without bolting on external disjointed datastores. It needs to provide a unified scheduling surface connecting customers to physical resources or staff members, directly wired into the main ledger for deposits and the central inventory system for slot limits.

  ```mermaid
  erDiagram
      Tenant ||--o{ BookingService : offers
      Tenant ||--o{ SchedulableResource : manages
      BookingService ||--o{ Appointment : creates
      SchedulableResource ||--o{ Appointment : booked_for
      Customer ||--o{ Appointment : requests
      Appointment ||--|| DepositInvoice : triggers
  ```

  ### AI Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets), and proceed to a deposit payment flow (Stripe).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Objective**: Design and implement the backend models, secure API endpoints, and core agent capabilities for the native booking system, ensuring robust multi-tenant RLS isolation.
  **CUJ**: A service owner creates a new service offering with availability rules. A customer visits the mobile web interface, selects a time slot, and confirms a booking. The AI assistant schedules the booking, verifies no conflicts, and generates a follow-up action.
  **Acceptance Criteria**:
  - The persistence layer accurately tracks schedulable capacity, resource constraints, and service bookings, all strictly scoped by `tenant_id`.
  - A comprehensive set of API operations allows owners to manage capacity and customers to securely book time.
  - The Operations Agent is equipped with the necessary tools to read schedules and generate booking operations natively.
  - 100% unit test coverage for the new capability module.
  - Playwright E2E tests validating the booking creation flow from the browser UI as a regular user, verifying the result in the owner dashboard.

  ## 5. Priority & Scope
  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
