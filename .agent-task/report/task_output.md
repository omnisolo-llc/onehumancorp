issue_title: "Research: Automated Appointment Booking System Architecture"
issue_description: |
  # Automated Appointment Booking System Design

  ## Problem Statement
  Service-based businesses (e.g., Leo the Music Tutor, Carlos the Handyman) currently rely on disparate third-party booking tools that are not deeply integrated into their main business operations and customer records. The process involves manual calendar management, complex deposit collection logic, and separate platforms for automated re-engagement.

  ## Research Report
  - Competitor Analysis: Platforms like Shopify typically require integration with specialized third-party booking applications (like Calendly or Acuity), leading to increased subscription costs and a disjointed user experience. Native tools on Wix or Squarespace lack proactive agentic features.
  - The OHC Value Add: Integrating appointment booking natively alongside physical/digital commerce features will reduce costs and improve UX for both merchants and customers, avoiding the "app tax". Integrating AI directly enables automated calendar management, real-time rescheduling, and proactive re-engagement.

  ## Design Doc
  ### Architectural Data Schema
  The existing base schema is partially present (as seen in `services`, `bookings`, `availability_blocks`, `booking_resources`), but must be strictly enforced via multi-tenant Row Level Security (RLS) constraints:
  - `services`: Represents the types of appointments offered (duration, cost, title).
  - `booking_resources`: The individual provider or physical resource being booked (e.g., a specific music room, a particular staff member).
  - `availability_blocks`: Denotes explicitly marked availability intervals.
  - `bookings`: Associates a Customer, Service, and Resource with specific timestamps, tracking the status (e.g., pending_payment, confirmed, cancelled).

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ SERVICE : offers
      TENANT ||--o{ BOOKING_RESOURCE : has
      TENANT ||--o{ CUSTOMER : serves
      SERVICE ||--o{ AVAILABILITY_BLOCK : scheduled_via
      SERVICE ||--o{ BOOKING : reserved_as
      BOOKING_RESOURCE ||--o{ BOOKING : assigned_to
      CUSTOMER ||--o{ BOOKING : makes

      BOOKING {
          uuid id PK
          string tenant_id FK
          string status
          timestamptz start_time
          timestamptz end_time
      }
      SERVICE {
          string id PK
          string tenant_id FK
          bigint price_cents
      }
  ```

  ### AI Integration Points
  - **Operations Agent**: Automatically parses natural language to handle rescheduling requests and manages the calendar based on synced external sources (e.g., Google Calendar integration).
  - **Sales/Customer Success Agent**: Analyzes booking history to proactively identify dormant customers (e.g., missed their usual monthly lesson) and drafts re-engagement follow-up messages with direct booking links.

  ### Mobile UX Flow (375px First)
  - **Customer Flow**: A clean, touch-friendly calendar view with prominent selection targets (>44px). The flow moves from date selection -> available time slot -> checkout/deposit.
  - **Owner Dashboard**: Displays a unified chronological feed of upcoming appointments, highlighting pending requests or AI-suggested actions in a scannable format.

  ## Implementation Prompt
  Implement the "Autonomous Booking System" as described for Leo the Music Tutor.
  1. Ensure the core database models (Service, AvailabilityBlock, Booking, BookingResource) exist, have strict `tenant_id` RLS policies applied, and are properly exposed in the data access layer.
  2. Implement the backend API endpoints necessary to query available time slots and create a new booking with a required deposit flow (integrating with the existing Stripe payment system logic).
  3. Create the mobile-first (375px) customer booking UI in the frontend, focusing on a robust, responsive calendar and slot selection component.
  4. Build the owner dashboard view for managing bookings, ensuring that it adheres to the macOS Translucent Glass styling.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
