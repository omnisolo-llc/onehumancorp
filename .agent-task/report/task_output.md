issue_title: "OHC AI-Native Autonomous Appointment Booking & Resource Management System"
issue_description: |
  # Research Report: AI-Native Autonomous Appointment Booking & Resource Management

  ## Problem Statement
  Service-based small business owners (e.g., Leo the Music Tutor, Carlos the Field Service Owner) struggle with fragmented booking systems. They typically have to bolt on third-party tools (like Calendly or specialized Shopify apps) to their main website, which leads to a disconnected experience: separate customer records, complicated deposit payment flows, and manual follow-ups. Existing platforms do not offer an integrated AI that actively manages the calendar, handles rescheduling via natural language, and proactively re-engages dormant clients.

  ## Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for robust booking, often adding $15-$30/month to the subscription cost and fracturing the user experience. Wix and Squarespace offer native booking but lack proactive, agent-driven management. They wait for the user to configure availability and for the customer to initiate the booking.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with the Operations and Sales AI Agents, OHC can eliminate the "app tax" and provide a genuinely proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings are treated as products via apps; poor native calendar management.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from the primary business storefront and CRM.

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD;
      Customer[Customer / Web] -->|Books Slot| BookingEngine[Booking Engine API];
      BookingEngine --> Redis[(Redis Redlock - ohc:lock:tenant_id:resource_id)];
      Redis --> Postgres[(PostgreSQL Central Ledger)];
      OperationsAgent[Operations Agent] -->|Reads/Updates| Postgres;
      OperationsAgent -->|Generates Availability| BookingEngine;
      SalesAgent[Sales/Customer Success Agent] -->|Re-engages dormant clients| Push[Mobile Push / SMS];
  ```
  ### Data Model (PostgreSQL)
  - `Service`: The type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., Leo) or physical space.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled). All tables enforce multi-tenant isolation via `tenant_id`.

  ### Mobile UX Flow (375px First)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (>= 44x44px touch targets), and proceed to a deposit payment flow via Stripe Checkout.
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions directly on their 375px mobile shell.

  ### AI Agent Integration
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests via natural language (e.g., "Move Leo's lesson to 3 PM"), and manages availability blocks.
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a student missing a week) and drafts a re-engagement message with a direct booking link for owner approval.

  ## Implementation Prompt
  **Target Persona**: Leo the Music Tutor & Carlos the Field Service Owner
  **Critical User Journey (CUJ)**:
  1. Customer visits Leo's OHC site on mobile and books a 1-hour music lesson, paying a deposit via Stripe.
  2. The system applies a Redis Redlock to prevent double-booking the slot.
  3. Booking is finalized in the PostgreSQL ledger; Leo receives a push notification on his OHC mobile app.
  4. Later, the Operations Agent identifies a scheduling conflict and drafts a reschedule message for Leo to approve.
  5. The Customer Success Agent notices a past student hasn't booked in 3 weeks and drafts a follow-up text with a re-booking link.

  **Next Actions for Engineering**:
  - **Step 1:** Implement core Data Models (`Service`, `Resource`, `AvailabilityBlock`, `Booking`) with strict `tenant_id` multi-tenant isolation.
  - **Step 2:** Develop the Customer Booking Flow UI optimized for 375px viewports and integrate it with Stripe payment intents for deposits.
  - **Step 3:** Create the Operations Agent capability to parse natural language rescheduling requests and manage calendar availability dynamically.
  - **Step 4:** Add E2E Playwright tests verifying the end-to-end booking flow without mocked data.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
