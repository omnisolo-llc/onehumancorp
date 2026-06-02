issue_title: "[Architecture] Unified Multi-Tenant Booking & Availability Engine"
issue_description: |
  # [Architecture] Unified Multi-Tenant Booking & Availability Engine

  ## Problem Statement
  Currently, OneHumanCorp (OHC) lacks a universal scheduling and availability engine. Many of our core personas require robust, time-based booking features to run their businesses effectively:
  - **Carlos (Handyman)** needs time slots for service appointments.
  - **Leo (Music Tutor)** requires synchronized lesson booking with Google Calendar integration.
  - **Fatima (Food Cart Operator)** needs pre-order pickup time slots.

  Without a centralized, multi-tenant booking architecture, we risk fragmenting the data model across different business types (e.g., service appointments vs. food pickups), leading to technical debt, inconsistent UI on mobile, and complex AI Agent integrations. We need a unified engine where "time" and "availability" are generic, bookable resources isolated by `tenant_id`.

  ## Research Report
  **Competitor Analysis:**
  - **Shopify**: Booking is typically handled via third-party apps (e.g., Sesami, Appointo). This creates a disjointed user experience and additional costs.
  - **Wix**: Offers Wix Bookings, which is integrated but notoriously complex to configure for simple use cases like a food cart pre-order.
  - **Squarespace**: Acquired Acuity Scheduling. Powerful, but feels like a separate product and is not mobile-first.

  **Our Approach (OHC Differentiation):**
  A native, transparent availability engine seamlessly embedded into the platform. AI Operations Department manages the calendar autonomously. Real-time availability checks using PostgreSQL `EXCLUDE USING gist` for overlapping intervals, ensuring strict multi-tenant isolation.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ BOOKABLE_RESOURCE : owns
      BOOKABLE_RESOURCE ||--o{ AVAILABILITY_SLOT : has
      BOOKABLE_RESOURCE ||--o{ BOOKING : receives
      CUSTOMER ||--o{ BOOKING : places
      BOOKING {
          uuid id PK
          uuid tenant_id FK
          uuid resource_id FK
          uuid customer_id FK
          timestamp start_time
          timestamp end_time
          string status
          jsonb metadata
      }
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  - **Screen 1: Service Selection (Customer View)**
    - Glassmorphism card detailing the service (e.g., "Guitar Lesson").
    - Large, tappable touch targets (≥ 44x44px) for selecting standard durations.
  - **Screen 2: Date & Time Picker**
    - Horizontal scrolling date chips (7 days ahead).
    - Vertical list of available time slots (e.g., 9:00 AM, 10:30 AM).
    - "Sold Out" state clearly marked with low opacity.
  - **Screen 3: Deposit & Confirmation**
    - Native numeric keypad for deposit entry if applicable.
    - Apple Pay / Google Pay integration via Stripe Terminal.
    - Post-booking confirmation screen.

  ### AI Agent Integration Points
  - **Operations ("The Manager")**: Automatically confirms bookings, blocks out time for maintenance, and manages capacity limits.
  - **Sales ("The Salesperson")**: Follows up with customers who viewed a booking slot but didn't complete the deposit.
  - **Customer Success ("The Ambassador")**: Sends SMS/Email reminders 24 hours before the appointment.

  ### Key Design Decisions
  - **PostgreSQL Range Types**: Utilizing `tsrange` and GiST indexes to prevent double-booking at the database level.
  - **Zero Trust Multi-Tenancy**: Every table must enforce `tenant_id` via Row Level Security (RLS).
  - **Stateless Read Replicas**: High-frequency availability checks will be routed to read replicas, cached aggressively via Redis.

  ## Implementation Prompt
  **Objective**: Implement the backend and frontend for the Unified Multi-Tenant Booking Engine.

  **CUJ (Critical User Journey)**:
  As Leo (Music Tutor), I want to create a new bookable resource "1-Hour Guitar Lesson", set my availability to Mon-Fri 9 AM - 5 PM, and see a mobile-optimized public booking link. As a customer, I want to open the link, select Tuesday at 2 PM, and pay a deposit.

  **Acceptance Criteria**:
  1. Define the multi-tenant database schema (PostgreSQL) for resources, availability, and bookings.
  2. Implement the gRPC/REST APIs for creating resources and querying availability.
  3. Build the Flutter UI (375px optimized) with Glassmorphism styling for both the Owner Dashboard (setting availability) and the Public Booking flow.
  4. Integrate the Operations AI agent to listen to booking events and update the calendar state.
  5. Minimum 5 Playwright E2E tests validating the end-to-end booking flow from the customer's perspective.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
