issue_title: "Implement Autonomous Appointment Booking & Resource Management System"
issue_description: |
  ## Issue Brief: Autonomous Appointment Booking & Resource Management System

  ### Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Handyman) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Crucially, existing platforms don't offer an integrated AI that actively manages the calendar and re-engages dormant clients.

  ### Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and customer relationship management.

  ### Design Doc
  - **Architecture diagram (Mermaid.js)**:
    ```mermaid
    erDiagram
        Tenant ||--o{ Service : offers
        Tenant ||--o{ Resource : owns
        Tenant ||--o{ Customer : serves
        Service ||--o{ Booking : includes
        Resource ||--o{ Booking : assigned_to
        Resource ||--o{ AvailabilityBlock : has
        Customer ||--o{ Booking : makes
    ```
  - **UI Wireframes / Screen Flow (375px first)**:
    - **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets), and proceed to a deposit payment flow (Stripe).
    - **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions.
  - **Mobile UX Flow**:
    1. **Customer Booking**: Customer lands on the storefront link, taps 'Book Now', selects a service, picks an available slot, pays the deposit via Stripe Terminal/Checkout, and gets a confirmation.
    2. **Owner Management**: The Operations Agent receives the booking, syncs it with the owner's Google Calendar, and pushes a notification card to the owner's 375px OHC feed.
  - **AI Agent Integration Points**:
    - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
    - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.
  - **Key Design Decisions and Why**:
    - *Native Integration vs. App Tax*: We build booking directly into the OHC core data model (`Service`, `AvailabilityBlock`, `Booking`) to prevent fragmentation and avoid charging users for "basic" features.
    - *Proactive Re-engagement*: The Sales Agent is integrated deeply with the Booking entities to recognize dormant customers, driving revenue passively.

  ### Implementation Prompt
  **User-facing Outcome**: Leo (Music Tutor) can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Critical User Journey (CUJ)**:
  1. Customer visits Leo's OHC site on mobile.
  2. Taps "Book Lesson", selects a time block from an agent-managed calendar.
  3. Completes payment.
  4. Leo receives a notification and sees the booking in his centralized Feed.
  5. One week later, if the student hasn't re-booked, the Sales Agent drafts a message: "Hey, time for your next lesson? Here's a quick link to book." Leo taps "Approve" from his feed.

  **Acceptance Criteria**:
  - The booking flow is completely usable on a 375px viewport with native mobile keyboards and touch targets >= 44x44px.
  - Core models for `Service`, `AvailabilityBlock`, and `Booking` are implemented with multi-tenant row-level isolation.
  - E2E Playwright tests must verify the full customer booking flow and the owner's feed notification.
  - Operations and Sales Agents must successfully interact with booking data.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
