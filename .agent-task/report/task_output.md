issue_title: "Implement Autonomous Unified Booking & Revenue Engine"
issue_description: |
  # Architecture Gap: Lack of Unified Booking & Revenue Engine

  ## 1. Problem Statement
  Currently, OneHumanCorp (OHC) lacks a comprehensive, unified booking and resource management engine with built-in revenue coordination and autonomous agent capabilities. While individual features might have rudimentary time/date selection, there is no centralized system to handle complex scheduling logic, resource allocation, and automated revenue recovery (e.g. dormant customer follow-up).

  From a non-technical owner/operator's perspective:
  - **Carlos (Handyman):** Needs to schedule jobs that require his time AND specific equipment, preventing double-booking of either.
  - **Leo (Tutor):** Needs a booking system that syncs with his personal calendar, handles timezone conversions for online students, and manages recurring lesson packages.
  - **Priya (Boutique):** Might want to schedule personal styling appointments in-store, requiring both a staff member's time and a fitting room.
  - **Nora (Agency):** Needs to schedule client review meetings based on the availability of multiple team members.

  Without a unified engine, owners face double-bookings, manual calendar coordination, and lost revenue from scheduling friction.

  ## 2. Research Report
  - **Competitor Analysis:**
    - **Acuity Scheduling / Calendly:** Excel at calendar syncing and basic availability but lack deep integration with physical resources (equipment, rooms) and commerce (deposits, inventory).
    - **Shopify:** Primarily product-focused; booking requires third-party apps, creating a disjointed experience for service businesses.
    - **Mindbody:** Comprehensive for fitness/wellness (staff + rooms) but overly complex and expensive for general small businesses.
  - **OHC Requirement:** We need a system that combines the simplicity of Calendly for simple meetings with the robustness of Mindbody for resource allocation, fully integrated into the OHC commerce and AI assistant ecosystem, leveraging agents to trigger "dormant customer" follow-up.

  ## 3. Design Doc: Unified Booking Engine Architecture

  ### 3.1 Core Concepts
  - **Resource:** Anything that can be scheduled (Staff Member, Room, Equipment, Vehicle).
  - **Service/Event:** What the customer is booking (e.g., "Plumbing Repair", "Guitar Lesson"). Requires one or more Resources.
  - **Availability:** When a Resource is bookable (Working hours, minus existing bookings and external calendar events).
  - **Booking:** A confirmed reservation of a Service, locking the required Resources for a specific time slot.

  ### 3.2 Architecture Diagram (Mermaid)

  #### Sequence Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant OwnerApp as Owner (Mobile App)
      participant OHC as OHC Booking Engine
      participant OpsAgent as Operations Agent
      participant CSAgent as Customer Success Agent

      Customer->>OHC: Books Service & Pays Deposit
      OHC->>OpsAgent: Update Calendar & State
      OpsAgent->>OpsAgent: Run Nightly Dormant Analysis
      OpsAgent->>CSAgent: Trigger: "Sarah missed regular slot"
      CSAgent-->>OHC: Draft check-in message & magic link
      OHC->>OwnerApp: Push Notification: "Approve check-in for Sarah?"
      OwnerApp->>OHC: Tap "Approve"
      OHC->>Customer: Send SMS/Email
  ```

  #### ER Diagram
  ```mermaid
  erDiagram
      TENANTS ||--o{ CUSTOMERS : "owns"
      TENANTS ||--o{ SERVICES : "offers"
      TENANTS ||--o{ AVAILABILITY_BLOCKS : "defines"
      TENANTS ||--o{ BOOKINGS : "manages"
      TENANTS ||--o{ OHC_UNIVERSAL_LEDGER : "records"

      CUSTOMERS ||--o{ BOOKINGS : "makes"
      SERVICES ||--o{ AVAILABILITY_BLOCKS : "has"
      PRODUCTS ||--o{ BOOKINGS : "reserved_via"

      BOOKINGS {
          string id PK
          string tenant_id FK
          string customer_id FK
          string product_id FK
          timestamp start_time
          timestamp end_time
          string status "pending, pending_payment, confirmed, completed, cancelled"
          string payment_intent_id
      }

      SERVICES {
          string id PK
          string tenant_id FK
          string title
          string description
          bigint price_cents
      }

      AVAILABILITY_BLOCKS {
          string id PK
          string tenant_id FK
          string product_id FK
          timestamp start_time
          timestamp end_time
          boolean is_available
      }
  ```

  ### 3.3 Mobile UX Flow (Carlos - Handyman Booking)
  1. **Customer View (375px):** Clicks "Book Repair" link. Sees a clean, touch-friendly calendar. Only dates/times where Carlos AND his required equipment are available are shown.
  2. **Selection:** Customer taps a time slot -> simple form (name, issue description) -> Stripe payment (deposit).
  3. **Owner View (Carlos):** Receives push notification. The booking appears on his OHC daily agenda view. If he manually adds a personal appointment, those slots instantly disappear from the public booking link.

  ### 3.4 AI Agent Integration
  - **Operations Assistant:** Continuously monitors the booking schedule. If Carlos is booked for a job across town, the agent automatically blocks out travel time on either side based on Google Maps estimates.
  - **Customer Assistant:** Drafts confirmation and reminder messages based on the booking details. Handles rescheduling requests via natural language (e.g., Customer texts "Can we move to Tuesday?").

  ### 3.5 Key design decisions and why
  - Strict multi-tenancy isolation using `tenant_id` combined with row-level security ensuring safe operations in a single Postgres cluster.
  - Utilizing `skip locked` for robust execution of Agent background tasks like nightly dormany analysis to avoid concurrency race conditions.

  ## 4. Implementation Prompt
  **Goal:** Implement the backend foundation and core APIs for the Unified Booking Engine.
  **CUJ:** A business owner (e.g., Carlos) defines a Service ("Plumbing Repair") that requires 2 hours of time. A customer queries available time slots for that service. The system returns slots based on the owner's defined working hours, omitting times already booked. The customer successfully creates a Booking, and the slot is no longer available in subsequent queries.
  **Acceptance Criteria:**
  - Create robust data models for Resources, Services, and Bookings with strict tenant isolation.
  - Implement an efficient availability calculation engine (handling overlapping schedules and existing bookings).
  - Provide REST/gRPC endpoints to query availability and create bookings.
  - Ensure operations are atomic to prevent double-booking race conditions (e.g., using PostgreSQL transactional locks).

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
