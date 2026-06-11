issue_title: "Implement Autonomous Agentic Booking & Resource Management System"
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
      A[Customer UI] --> B(OHC API Layer)
      B --> C[Operations Agent]
      B --> D[Sales/CS Agent]
      C --> E{Calendar & Availability}
      D --> F[Tenant CRM]
      B --> G[(Unified Ledger/Stripe)]
      C -.-> H[Push Notifications to Owner]
  ```

  ### Data Model Overview
  - The system requires managing services offered, the time blocks a resource is available, and the appointments booked by customers.
  - State transitions for a booking must be rigorously tracked (e.g., from pending deposit to confirmed).

  ### AI Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests via natural language processing, and manages dynamic availability based on existing blocks and external calendar sync (e.g. Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets), and proceed to a deposit payment flow via Stripe.
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions.

  ### Key Design Decisions and Why
  - **Native Integration vs Third Party**: Built natively into OHC rather than relying on external app integrations to ensure a unified customer record and seamless deposit flows.
  - **AI Proactivity**: Instead of just being a passive calendar, AI agents actively manage dormant users to drive revenue, adhering to the "AI Does Useful Work" core value.
  - **Mobile-First**: The booking flow is prioritized for 375px viewports because many owners (like Carlos) operate entirely from their phones, meaning large touch targets and offline-tolerance are essential.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Target Persona**: Leo the Music Tutor
  **Outcome**: Leo can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Next Actions**:
  1. Design and implement the necessary data structures and relationships to support services, availability, and bookings with strict multi-tenant isolation. Do not use a pre-determined schema; design it based on best practices for PostgreSQL.
  2. Develop the Customer Booking Flow UI (mobile-first calendar and slot selection) and integrate it with the existing Stripe payment system for deposits.
  3. Create the Operations Agent capability to parse natural language rescheduling requests and manage calendar availability.
  4. Develop the Owner Dashboard view to manage bookings and view AI-suggested follow-ups.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
