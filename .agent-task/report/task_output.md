issue_title: "[Research] OHC Autonomous Unified Booking & Revenue Engine"
issue_description: |
  # OHC Autonomous Unified Booking & Revenue Engine

  ## 1. Problem Statement (The Owner Perspective)
  Service-based and hybrid owners like Leo (music tutor) or Carlos (handyman) struggle with fragmented toolchains. They are forced to bolt third-party booking tools (Calendly, Acuity) onto generic site builders, resulting in disconnected customer records, clunky deposit payment flows, and massive operational overhead. Crucially, existing tools are passive—they wait for the customer to book. When a regular student misses a week, or a bi-annual HVAC maintenance is due, the owner has to remember to follow up manually, leading to lost revenue. Owners need a unified, autonomous system that not only accepts bookings seamlessly but proactively manages the calendar and re-engages dormant clients to drive recurring revenue.

  ## 2. Research Report
  ### 2.1 Competitive Analysis
  - **Shopify/Wix**: Treat bookings as "products" via clumsy add-ons. Poor native calendar management and zero proactive engagement.
  - **Calendly/Acuity**: Excellent scheduling but entirely detached from the core CRM and e-commerce storefront. They don't know if a booked user is a VIP, a first-time lead, or someone who bought a physical product last month.
  - **Vertical SaaS (Mindbody, Jobber)**: Powerful but overly complex, expensive, and lacking autonomous AI features for non-technical micro-businesses.

  ### 2.2 The OHC Opportunity
  By natively integrating a booking engine into the core OHC data model alongside products and customers, we unlock unified commerce. The true differentiator is the "Autonomous Operations Agent." It doesn't just display a calendar; it actively analyzes booking patterns, anticipates cancellations, and drafts personalized re-engagement messages, transforming a passive calendar into an active revenue driver.

  ## 3. Design Doc (Architecture & UX Flow)

  ### 3.1 Mobile UX Flow (The "Zero-Touch" Owner Experience)
  1.  **Passive Booking**: A customer visits Leo's OHC mobile site (375px), selects a "30-min Guitar Lesson," picks a slot, and pays a $20 Stripe deposit seamlessly.
  2.  **Autonomous Analysis**: The Operations Agent notes that a regular student, Sarah, missed her usual Tuesday slot.
  3.  **Proactive Re-engagement**: The OHC app pushes a notification to Leo: "Sarah missed her usual Tuesday lesson. Operations Agent drafted a check-in message with a direct booking link for next week. Approve?"
  4.  **1-Tap Approval**: Leo taps "Approve." The message is sent via SMS/WhatsApp.
  5.  **Unified Feed**: The new booking and the sent message appear in Leo's central Agent Feed.

  ### 3.2 AI Agent Integration Points
  -   **Operations Agent**: Manages calendar state, resolves double-booking conflicts, and identifies deviations in recurring customer behavior (the "dormant client" trigger).
  -   **Customer Success Agent**: Drafts contextual, personalized follow-up messages based on the customer's history and the Operations Agent's triggers.

  ### 3.3 Architecture Diagram

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

  ### 3.4 Key Design Decisions
  -   **Native Entity**: Bookings must be a first-class entity in the database, directly linked to `Tenant`, `Customer`, and a `LedgerTransaction` (for deposits), ensuring strict row-level security.
  -   **Actionable Feed over Dashboards**: Avoid complex calendar grid views as the primary interface for the owner. Prioritize the Agent Feed for upcoming critical events and suggested actions.
  -   **Stripe Intent Integration**: Deposits must utilize Stripe PaymentIntents securely tied to the booking state machine to handle refunds automatically on cancellation.

  ## 4. Implementation Prompt (For Implementer Agent)
  **Objective**: Architect the backend data model and core service layer for the "Autonomous Unified Booking Engine."

  **Requirements**:
  1.  **Data Schema**: Design the PostgreSQL schema for `Service`, `AvailabilityBlock`, and `Booking`. Ensure strong foreign keys to the existing `Customer` and multi-tenant `Tenant` tables. Include fields necessary for Stripe deposit tracking and state management (e.g., pending, confirmed, completed, cancelled).
  2.  **Dormancy Trigger Logic**: Design the query or logic that the `Operations Agent` will use to identify "dormant" recurring customers. Define how this analysis will be scheduled (e.g., cron job via the AI Job Queue).
  3.  **State Machine**: Define the state transitions for a `Booking` and the associated side-effects (e.g., triggering a confirmation email when moving from `pending` to `confirmed`).
  4.  **Zero UI Assumptions**: Focus purely on the backend systems, data models, and agent workflows. Do not build frontend UI, but ensure the APIs support the mobile-first approval flow.

  **Acceptance Criteria**:
  -   Entity-Relationship (ER) diagram for the Booking subsystem.
  -   Detailed schema definitions (SQL or ORM pseudo-code).
  -   A sequence diagram outlining the state transitions of a booking from creation to completion.

  ## 5. Metadata
  **Priority**: P0 (Critical - unlocks the massive services market segment).
  **Estimated Scope**: Large.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
