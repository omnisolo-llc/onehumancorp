issue_title: "Implement Autonomous Agentic Booking & Resource Management System"
issue_description: |
  # Research Report: Autonomous Appointment Booking & Resource Management System

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
  The booking architecture needs to be fully integrated with our native multitenant data model.

  - `Service`: The type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., Leo) or physical space.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled).

  ```mermaid
  graph TD
      A[Service Inquiry] --> B(Operations Agent Triage)
      B --> C{Calendar Sync}
      C --> D[Availability Lookup]
      D --> E[Booking Proposal]
      E --> F(Deposit Flow)
      F --> G[Booking Confirmed]
      G --> H[Followup Scheduler]
      H --> I[Customer Success Agent Re-engagement]
  ```

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view optimized for 375px screens. Customers select a date, see available slots (large touch targets >= 44px), and proceed to a integrated deposit payment flow (via Stripe).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive glassmorphism-styled push notifications for new bookings or AI-drafted follow-up suggestions in their Triage Feed.
  3. **Approval UX**: When an appointment is requested or needs rescheduling, the owner receives an Agent Proposal card with a single "Approve" button.

  ### AI Agent Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (e.g., Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ## Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Target Persona**: Leo the Music Tutor

  **User-facing Outcome**: Leo can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Critical User Journey (CUJ)**:
  1. Customer views Leo's service offering on their mobile browser and taps "Book Lesson".
  2. A native touch-optimized calendar appears; customer selects an available time block.
  3. Customer pays the required deposit using Stripe Elements integration.
  4. The booking state is finalized in the database and Leo receives a triage feed notification: "New Booking: Piano Lesson on Thursday at 4 PM".
  5. If the customer does not re-book after 10 days, the Customer Success Agent drafts a message: "Hi, it's been a while since your last lesson! Would you like to book another session?" and presents it to Leo for approval.

  **Acceptance Criteria**:
  - Implement core Data Models (`Service`, `AvailabilityBlock`, `Booking`) with strict multi-tenant RLS.
  - Develop the Customer Booking Flow UI (mobile-first calendar and slot selection) and integrate it with the existing Stripe payment system for deposits.
  - Extend the Operations Agent capability to parse natural language rescheduling requests and manage calendar availability.
  - Implement E2E Playwright tests simulating a full booking and payment flow.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
