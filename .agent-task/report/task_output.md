issue_title: "Implement Autonomous Booking & Calendar Management"
issue_description: |
  # Mission Queue Protocol: Autonomous Booking System

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
  ### Architecture & Data Model (PostgreSQL)
  ```mermaid
  erDiagram
      Tenant ||--o{ Service : offers
      Tenant ||--o{ Resource : has
      Service ||--o{ Booking : creates
      Resource ||--o{ Booking : assigned_to
      Resource ||--o{ AvailabilityBlock : has
      Customer ||--o{ Booking : makes
  ```
  - `Service`: The type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., Leo) or physical space.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled).

  ### AI Integration Points
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets, min 44x44px), and proceed to a deposit payment flow (Stripe).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions in their Agent Feed.

  ## Implementation Prompt
  **Outcome**: A functional, mobile-first booking module integrated directly into the OHC platform, removing the need for external scheduling tools.
  **CUJ**:
  1. Carlos (owner) creates a "Plumbing Diagnostic" service (1 hr, $50 deposit) via natural language chat with the OHC Agent.
  2. Carlos sets his working hours (Mon-Fri, 9am-5pm) and connects his Google Calendar to block personal events.
  3. A customer visits Carlos's OHC site on their phone, selects a slot, and pays the deposit via Stripe.
  4. The Booking is confirmed, and the AI agent automatically drafts a "Pre-arrival checklist" email to the customer.

  **Acceptance Criteria**:
  - Implement full-stack booking logic: gRPC/Proto definitions, Go backend with PostgreSQL `Booking`, `Service`, `Resource`, `AvailabilityBlock` tables (with `tenant_id` RLS).
  - Mobile-responsive frontend UI in Flutter/PWA with translucent glass design tokens.
  - AI Agent integration: The agent must be able to draft a follow-up message when a booking is created or updated.
  - End-to-end tests covering the entire CUJ.

  ## Priority
  P1

  ## Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
