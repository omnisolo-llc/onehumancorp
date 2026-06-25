issue_title: "Implement native agentic booking system for service business owners"
issue_description: |
  # Mission Queue Protocol: Agentic Booking System

  ## 1. Title
  Implement native agentic booking system for service business owners

  ## 2. Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ## 3. Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ## 4. Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      RESOURCE ||--o{ AVAILABILITY_BLOCK : has
      SERVICE ||--o{ BOOKING : defines
      RESOURCE ||--o{ BOOKING : assigned_to
      CUSTOMER ||--o{ BOOKING : makes
      BOOKING }|--|| PAYMENT_DEPOSIT : requires
  ```

  ### Data Model (PostgreSQL)
  - `Service`: The type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., Leo) or physical space.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled).

  ### AI Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets), and proceed to a deposit payment flow (Stripe).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions.

  ## 5. Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Objective**: Build an end-to-end booking flow from the customer-facing scheduling UI to the owner's dashboard view, driven by agentic scheduling intelligence.
  **CUJ**: A service business owner configures an hourly "Consultation" service, a customer books a slot on mobile and pays a deposit, and the owner sees the new booking in their daily unified feed.
  **Acceptance Criteria**:
  - The booking UI must be fully functional on a 375px viewport with 44px minimum touch targets.
  - It must support zero-mock data (read/write directly from API to DB).
  - It must follow the OHC Premium Token visual standards (Translucent Glass materials, 16px corner radius).

  ## 6. Priority
  P1 (High)

  ## 7. Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
