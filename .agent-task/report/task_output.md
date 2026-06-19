issue_title: "Implement Autonomous AI Booking System"
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
      A[Customer Web/Mobile UI 375px] -->|Initiate Booking| B(OHC API Gateway)
      B --> C{Booking Service}
      C --> D[PostgreSQL: Check Availability]
      C --> E[Stripe: Process Deposit]
      D --> F[Reserve Slot]
      E --> G[Confirm Payment]
      F --> H(Finalize Booking)
      G --> H
      H --> I[Operations Agent: Update Owner Calendar]
      H --> J[Sales Agent: Schedule Follow-up]
  ```

  ### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ SERVICE : offers
      TENANT ||--o{ RESOURCE : has
      TENANT ||--o{ CUSTOMER : serves
      SERVICE ||--o{ BOOKING : defines
      RESOURCE ||--o{ AVAILABILITY_BLOCK : "is available during"
      CUSTOMER ||--o{ BOOKING : makes
      RESOURCE ||--o{ BOOKING : "assigned to"

      SERVICE {
          uuid id PK
          uuid tenant_id FK
          string name
          int duration_minutes
          decimal price
          boolean requires_deposit
      }
      RESOURCE {
          uuid id PK
          uuid tenant_id FK
          string name
          string type
      }
      AVAILABILITY_BLOCK {
          uuid id PK
          uuid resource_id FK
          uuid tenant_id FK
          timestamp start_time
          timestamp end_time
          boolean is_recurring
      }
      BOOKING {
          uuid id PK
          uuid tenant_id FK
          uuid customer_id FK
          uuid service_id FK
          uuid resource_id FK
          timestamp start_time
          timestamp end_time
          string status
      }
      CUSTOMER {
          uuid id PK
          uuid tenant_id FK
          string name
          string email
          string phone
      }
  ```

  ### Data Model Summary (PostgreSQL)
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

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  **Target Persona**: Leo the Music Tutor
  **Outcome**: Leo can offer monthly lesson packages with an integrated booking calendar. The system handles deposit payments, syncs with his personal calendar, and automatically follows up with students who haven't booked in a while.

  **Next Actions**:
  1. Implement the core Data Models (`Service`, `AvailabilityBlock`, `Booking`) with strict multi-tenant isolation.
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
