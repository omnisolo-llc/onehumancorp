issue_title: "Implement Autonomous Booking & Resource Management System"
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
  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      Tenant ||--o{ Service : offers
      Tenant ||--o{ Resource : owns
      Service ||--o{ Booking : associated_with
      Resource ||--o{ AvailabilityBlock : has
      Resource ||--o{ Booking : assigned_to
      Customer ||--o{ Booking : makes

      Service {
          uuid id PK
          string tenant_id FK
          string name
          int duration_minutes
          decimal price
          decimal deposit_required
      }
      Resource {
          uuid id PK
          string tenant_id FK
          string name
          string type
      }
      AvailabilityBlock {
          uuid id PK
          string tenant_id FK
          uuid resource_id FK
          timestamp start_time
          timestamp end_time
      }
      Booking {
          uuid id PK
          string tenant_id FK
          uuid customer_id FK
          uuid service_id FK
          uuid resource_id FK
          timestamp start_time
          timestamp end_time
          string status
      }
      Customer {
          uuid id PK
          string tenant_id FK
          string name
          string email
          string phone
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Customer
      actor Owner
      participant Storefront UI
      participant Booking API
      participant Operations Agent
      participant Stripe
      participant DB

      Customer->>Storefront UI: Selects Date & Service
      Storefront UI->>Booking API: Request availability
      Booking API->>DB: Query AvailabilityBlock
      DB-->>Storefront UI: Return open slots
      Customer->>Storefront UI: Selects slot & proceeds to pay
      Storefront UI->>Stripe: Generate Deposit Session
      Customer->>Stripe: Completes Payment
      Stripe-->>Booking API: Webhook (payment_intent.succeeded)
      Booking API->>DB: Insert Confirmed Booking
      Booking API->>Operations Agent: Trigger "New Booking" event
      Operations Agent->>Owner: Push notification (Booking Confirmed)
  ```

  ### Data Model (PostgreSQL)
  - `Service`: The type of appointment (duration, price, deposit required).
  - `Resource`: The provider (e.g., Leo) or physical space.
  - `AvailabilityBlock`: Recurring or specific time blocks when the resource is available.
  - `Booking`: The actual appointment, linked to a Customer, Service, and Resource, with state (pending, confirmed, completed, cancelled). All tables must include `tenant_id` and have Row Level Security enabled.

  ### AI Integration Points
  - **Operations Agent**: Monitors the calendar, handles rescheduling requests, and generates dynamic availability based on existing blocks and external calendar sync (Google Calendar).
  - **Sales/Customer Success Agent**: Automatically identifies customers who haven't booked a follow-up (e.g., a music student missing a week) and drafts a re-engagement message with a direct booking link.

  ### Mobile UX Flow (375px)
  1. **Customer View**: A clean, touch-friendly calendar view. Customers select a date, see available slots (large touch targets, >44px), and proceed to a deposit payment flow (Stripe).
  2. **Owner View (Dashboard)**: The owner sees a unified feed of upcoming bookings and new requests. They receive push notifications for new bookings or AI-drafted follow-up suggestions.

  ### Key Design Decisions and Why
  - **Native Integration vs App Ecosystem**: We built booking natively to avoid the "App Tax" common on Shopify, providing a unified customer record.
  - **Agentic Proactivity**: We use the Sales Agent to automatically re-engage dormant customers, a feature Wix/Squarespace lack, directly driving revenue.
  - **Tenant Isolation**: Row-Level Security on all booking tables ensures strict data isolation for the multi-tenant architecture.

  ## 4. Implementation Prompt
  **Feature Name**: OHC Native Agentic Booking System
  Implement a booking model in Postgres that links directly to customers. Design the mobile booking checkout flow (375px native first) allowing deposits via Stripe Checkout. Implement the Operations Agent trigger that responds to scheduling requests. Develop the Dashboard view where owners can see upcoming slots and manage them intuitively.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
