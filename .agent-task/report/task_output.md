issue_title: "Implement Centralized Agentic Booking & Resource Management System"
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
  ### Data Model (PostgreSQL)
  - `services`: The type of appointment (duration, price, deposit required).
  - `booking_resources`: The provider (e.g., Leo) or physical space.
  - `availability_schedules`: Recurring or specific time blocks when the resource is available.
  - `availability_blocks`: A generated granular snapshot of specific start and end times for availability over a period.
  - `bookings`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled).
  - `booking_slots`: The slots reserved, linked to bookings and specific tenant operations.

  ### Multi-Tenancy & Security
  - Every table enforces Row-Level Security (RLS) tightly restricted to the authenticated `tenant_id`.
  - Operations Agents use SPIFFE/SPIRE for safe authenticated backend job queues without directly escalating outside tenant bounds.

  ### AI Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link. This is exposed directly into the owner's `agent_feed_items`.

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets, 44x44px min), and proceed to a deposit payment flow (Stripe).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests injected into their action cards. They receive push notifications for new bookings or AI-drafted follow-up suggestions, utilizing macOS Translucent Glass and UniFi layout styles.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Target Persona**: Leo the Music Tutor
  **Outcome**: Leo can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Next Actions**:
  1. Ensure the core Data Models (`availability_schedules`, `availability_blocks`, `booking_resources`, `bookings`, `booking_slots`) are fully realized across Rust server domain layers.
  2. Map incoming requests from the API layer correctly to the PostgreSQL agent_feed schema. Specifically, update old APIs that attempt to insert into deprecated `agent_feed` or `agent_approvals` tables to use the target `agent_feed_items` schema.
  3. Develop the Customer Booking Flow UI (mobile-first calendar and slot selection) utilizing 375px breakpoint constraints and integrate it with the existing Stripe payment system for deposits.
  4. Expand the Operations Agent capability to parse natural language rescheduling requests and manage calendar availability proactively based on the new Unified Booking structures.
  5. Develop the Owner Dashboard view to manage bookings and view AI-suggested follow-ups. Use standard Playwright E2E testing using local stacks to confirm all links and buttons.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
