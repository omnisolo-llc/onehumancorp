issue_title: "[booking] Service Booking Engine Native Support Gap"
issue_description: |
  # Research Report: Service Booking Engine Capability Gap

  ## Problem Statement
  Currently, OHC relies on generic external calendar links (e.g., Cal.com, Zoom) for "Services & Bookings" businesses (like Carlos the Handyman and Leo the Music Tutor). This creates a disjointed UX where the user leaves the OHC mobile app flow, disrupting the "No code. No servers. No jargon" promise and making it impossible for the "Operations" AI agent to natively manage deposits, calendar conflicts, and rescheduling.

  ## Research Report
  - **Competitor Analysis:**
    - **Wix Bookings** provides native scheduling, staff management, and deposit handling in a single interface.
    - **Squarespace Scheduling (Acuity)** handles complex time slots but feels bolted on.
    - **Shopify** requires expensive third-party apps for bookings, a major pain point for service businesses.
  - **OHC's Opportunity:** By building a native, multi-tenant booking engine in PostgreSQL with AI agent orchestration, OHC can own the entire flow (Acquisition -> Booking -> Deposit -> Fulfillment).

  ## Design Doc
  - **Architecture Diagram:**
    ```mermaid
    graph TD
        A[Mobile App] -->|gRPC/REST| B[BookingEngine Rust Service]
        B --> C[(PostgreSQL: bookings table with RLS)]
        B --> D[Redis: Concurrency Locks]
        B --> E[Stripe API: Pre-payment Deposits]
        F[Operations Agent] -->|Queries| B
    ```
  - **Architecture:**
    - New `BookingEngine` Rust module interfacing with the `bookings` PostgreSQL table.
    - Redis-backed concurrency locks (`ohc:lock:{tenant_id}:timeslot:{start_time}`) to prevent double-booking.
    - Stripe integration for pre-payment deposits.
  - **Mobile UX Flow:**
    - Clean, 375px-optimized glassmorphism calendar view.
    - Native time-slot picker (avoiding web-view popouts).
  - **AI Agent Integration:**
    - "The Manager" (Operations) reads available slots and auto-suggests times via DM/Chat.

  ## Implementation Prompt
  Implement the core `BookingEngine` Rust service. It must support checking availability, reserving a time slot with a Redis lock, and integrating with Stripe for deposit holds. Ensure all PostgreSQL queries enforce `tenant_id` row-level security. Expose this via a gRPC/REST endpoint for the mobile app.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
