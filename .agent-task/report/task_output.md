issue_title: "Architecture Design Suite for OHC Core Systems"
issue_description: |
  # Research Report: Complete Architecture Design Suite for OHC Core Systems

  ## 1. Problem Statement
  The OHC platform must serve diverse small business owners (Maya, Carlos, Priya, Leo, Fatima) seamlessly. The current journeys lack a unified architectural view, risking friction during critical phases. The gap we discovered is that there is no Unified Resource & Inventory Scheduling Matrix that handles both physical stock and time-based availability gracefully in a unified ledger, which is necessary for cross-channel business journeys (e.g. boutique + tutoring).

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify require third-party apps for booking. Wix and Squarespace offer native booking but lack proactive agent-driven management. They wait for the user to configure availability.
  - **The OHC Opportunity**: By integrating booking natively alongside e-commerce and powering it with AI Agents, OHC can eliminate the "app tax" and provide a proactive booking experience.
  - **Competitor Gaps**:
    - *Shopify*: Bookings treated as products via apps.
    - *Wix*: Complex setup; passive system.
    - *Calendly*: Excellent scheduling but detached from main storefront.
  - **Architectural Gap**: No single primitive unifies "Stock" (products) and "Time" (services/bookings) with distributed locking.

  ### Track 1: Architectural Gap & Scaling Discovery
  The most critical missing capability is a Unified Resource Scheduling Matrix. This is needed so that Carlos (handyman) and Leo (music tutor) can schedule services natively alongside deposits, and Priya (boutique operator) can lock inventory online while tap-to-pay processes offline without race conditions.

  ### Track 2: Selected Architecture Deep Dive
  **Unified Resource Scheduling Architecture**
  - **Business Journey Mapping**:
    - *Acquisition & Onboarding*: Maya creates a service (Cake Consultation) and a product (Vegan Cake).
    - *Activation*: Customer books a slot or buys an item. The Operations agent reserves the resource via Redis Redlock.
    - *Retention*: Sales agent re-engages dormant customers.
  - **Data Model & Invariants**:
    - `Resource`: Abstract entity representing Time (Leo's hours) or Stock (Priya's dresses).
    - `Ledger` (PostgreSQL): Absolute truth for all resource availability, using RLS per tenant.
    - `Lock` (Redis Redlock): Temporary reservation during checkout to prevent double-booking or overselling. Key pattern: `ohc:lock:{tenant_id}:{resource_type}:{resource_id}`.
  - **AI Department Coordination**:
    - *Operations Agent*: Monitors calendar/inventory and handles rescheduling/restocking.
    - *Sales Agent*: Re-engages dormant customers.

  ### Track 3: Technical Integrity & Mobile-First Review
  - **Mobile-First UX Flow (375px)**: Touch-friendly availability grid (44x44px targets). Seamless integrated Stripe checkout for deposits/purchases. Clean Apple/Ubiquiti-style hierarchy.
  - **Performance & Security**: Row-level tenant isolation in PostgreSQL. Redis Redlock for temporary consistency.

  ## 3. Design Doc
  **Architecture Diagram**:
  ```mermaid
  graph TD
      Client[Mobile/Web Client] --> Edge[Edge Cache]
      Edge --> API[API Gateway]
      API --> Queue[AI Job Queue]
      Queue --> OpsAgent[Operations Agent]
      Queue --> SalesAgent[Sales Agent]
      API --> Redis[Redis Redlock - Reservations]
      API --> DB[(PostgreSQL Ledger)]
      Redis --> DB
      OpsAgent --> DB
      SalesAgent --> DB
  ```

  ## 4. Implementation Prompt
  **Feature Name**: OHC Unified Resource Scheduling Matrix

  **Description**: Implement the foundational data models and background agent workflows for the Unified Resource Scheduling matrix. This must handle both time-based bookings and physical inventory stock consistently.

  **Acceptance Criteria**:
  1. Define PostgreSQL schema for `Resource` and `Ledger` with RLS.
  2. Implement Redis Redlock mechanism for temporary reservations.
  3. Integrate the Operations Agent to monitor resource availability and generate alerts.
  4. Build the Mobile-First (375px) Owner Unified Feed UI showing resource events.
  5. E2E Playwright verification.

  ## 5. Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
