issue_title: "[Research] OHC Autonomous Booking System Architecture & Implementation"
issue_description: |
  # Research Report: Autonomous Appointment Booking & Resource Management System

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
  ### Architecture Diagram
  ```mermaid
  graph TD
      MobileUI[Mobile-First UI 375px] --> API[Go API Server]
      API --> OpsAgent[Operations Agent]
      API --> SalesAgent[Sales Agent]
      OpsAgent --> DB[(PostgreSQL)]
      SalesAgent --> DB
      DB --> Services[Services Table]
      DB --> Blocks[Availability Blocks]
      DB --> Bookings[Bookings Table]
      OpsAgent -.-> ExtCal[External Calendar Sync]
      MobileUI -.-> Stripe[Stripe Checkout]
  ```

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets >= 44x44px), and proceed to a deposit payment flow (Stripe).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions.
  3. **Agent Interaction**: Users can use voice/text commands to manage schedule: "Block off next Tuesday afternoon."

  ### AI Agent Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests via natural language parsing, and generates dynamic availability based on existing blocks and external calendar sync.
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Target Persona**: Leo the Music Tutor
  **Outcome**: Leo can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Critical User Journey (CUJ)**:
  1. Leo opens OHC App, sets up a new service "1hr Piano Lesson".
  2. Customer visits Leo's OHC link, sees availability calendar.
  3. Customer selects time slot and pays deposit via Stripe.
  4. Booking appears in Leo's unified feed.
  5. 2 weeks later, Sales Agent notices customer hasn't re-booked, drafts follow up text for Leo to approve.

  **Next Actions / Acceptance Criteria**:
  1. Implement the API endpoints to expose the PostgreSQL schemas (`services`, `availability_blocks`, `bookings`, `booking_resources`). Ensure strict multi-tenant isolation (RLS is already configured in DB, needs API validation).
  2. Develop the Customer Booking Flow UI (mobile-first calendar and slot selection) and integrate it with the existing Stripe payment system for deposits. Validate 44x44px touch targets.
  3. Create the Operations Agent capability to parse natural language rescheduling requests and manage calendar availability.
  4. Develop the Owner Dashboard view to manage bookings and view AI-suggested follow-ups.
  5. Include E2E Playwright tests simulating the complete flow.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
