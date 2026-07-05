issue_title: "Unified Agentic Booking & Quoting Engine for Service Businesses"
issue_description: |
  # Mission: Unified Agentic Booking & Quoting Engine for Service Businesses

  ## Problem Statement
  Service-based small business owners (e.g., Carlos the Handyman, Leo the Music Tutor, Nora the Agency Principal) do not sell simple "Add to Cart" products. Their business relies on a multi-step flow: Inquiry -> Scoping -> Quoting -> Deposit -> Scheduling. Current platforms (Shopify, Wix) require stringing together disparate third-party apps (forms, Calendly, invoicing) to achieve this. Non-technical users resort to managing this chaos manually via Instagram DMs, Venmo, and personal calendars, leading to missed leads, no-shows, and delayed payments. They need a single, mobile-first engine that handles everything from initial inquiry to final payment seamlessly.

  ## Research Report
  - **Competitive Baseline (Shopify/Wix)**: These platforms are heavily optimized for SKU-based physical retail. Service booking requires plugins (e.g., Calendly integrations, custom form builders) which break the unified UI experience, add monthly costs, and fail to provide holistic AI orchestration.
  - **Specialized Tools (Jobber/Housecall Pro/Square)**: Strong in field service or appointments, but often too complex for simple tutors or independent contractors, and lacking in proactive, automated AI follow-ups.
  - **The OHC Opportunity**: By integrating booking and quoting natively alongside e-commerce and powering it with the AI Agents (Salesperson/Operations), OHC can eliminate the "app tax" and provide a genuinely proactive, zero-friction booking experience.
  - **User Pain**: "I spend 2 hours every evening just replying to DMs to figure out what kind of cake they want and when they need it, then tracking down deposits."

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ SERVICE : offers
      SERVICE ||--o{ INQUIRY : receives
      INQUIRY ||--o{ QUOTE : generates
      TENANT ||--o{ BOOKING : manages
      QUOTE ||--|{ BOOKING : transitions_to
      BOOKING ||--o{ INVOICE : requires
      INVOICE ||--|{ PAYMENT : records

      TENANT {
          string id PK
          string name
          string timezone
      }
      SERVICE {
          string id PK
          string tenant_id FK
          string name
          string type "Fixed | Custom Quote"
          int default_duration_minutes
      }
      INQUIRY {
          string id PK
          string tenant_id FK
          string customer_id FK
          string service_id FK
          text description
          string status "Pending | Quoted | Declined"
      }
      QUOTE {
          string id PK
          string tenant_id FK
          string inquiry_id FK
          string status "Draft | Sent | Accepted"
          float total_amount
          float required_deposit
          datetime expires_at
      }
      BOOKING {
          string id PK
          string tenant_id FK
          string quote_id FK
          datetime start_time
          datetime end_time
          string status "Pending Deposit | Confirmed | Completed"
      }
  ```

  ### High-Level Flow & Mobile UX Flow (375px First)
  1. **Customer View**: A seamless "Request a Service" or "Book an Appointment" form on the OHC storefront. Supports natural language descriptions and photo uploads.
  2. **AI Agent Processing ("The Salesperson")**:
     - The OHC backend intercepts the inquiry.
     - The AI analyzes the text/image context against the Business Profile.
     - The Agent drafts a proposed Quote (Price + Scope) and extracts available times from the "Operations" Agent's calendar.
  3. **Owner View (The 1-Tap Approval)**:
     - The owner (e.g., Carlos) receives a push notification on his OHC mobile app.
     - UI shows a Glassmorphism card: "New Inquiry: Leaky Faucet. Suggested Quote: $150. Suggested Time: Tue 2 PM."
     - Action buttons: [Approve & Send] / [Edit] / [Decline].
  4. **Customer Conversion**: Customer receives a unified OHC link to view the proposal, pick/confirm the time, and pay the deposit via Stripe to secure the booking.

  ### AI Agent Integration Points
  - **The Salesperson Agent**: Analyzes `Inquiry` payloads and proposes `Quote` drafts based on the tenant's pricing rules and past jobs.
  - **Operations Agent**: Monitors the calendar, generates dynamic availability blocks, handles rescheduling requests, and flags conflicts.
  - **Customer Success Agent**: Automatically identifies pending quotes or overdue bookings and drafts re-engagement messages.

  ## Implementation Prompt
  **Objective**: Implement the end-to-end "Agentic Booking & Quoting Engine" for service businesses.

  **Target Persona**: Carlos the Handyman / Leo the Music Tutor.

  **Critical User Journey (CUJ)**:
  1. As a Customer, I submit a custom service request via the storefront.
  2. As the System, the AI Salesperson agent automatically generates a draft quote and proposed times based on the request.
  3. As the Business Owner, I open the mobile-optimized dashboard, see the pending draft quote card, and tap "Approve".
  4. As a Customer, I receive the approved quote, confirm the booking time, and can pay the deposit.

  **Acceptance Criteria**:
  - Implement the core Data Models (`Inquiry`, `Quote`, `Booking`, `Service`) in PostgreSQL with strict multi-tenant row-level isolation.
  - Implement the backend logic and agent queue integration to route the inquiry to the AI agent for drafting the quote.
  - Build the UI components for the customer inquiry form and the owner approval card (must be perfectly usable at 375px width, utilizing OHC Premium Tokens).
  - Include full E2E Playwright tests covering this exact CUJ, from submitting the inquiry to owner approval and customer payment. For AI generation in E2E tests, use the official test-mode credentials or repository-provided local service adapters; never mock internal OHC network calls, and ZERO mock data may appear in the UI.
  - Do not prescribe the exact database migrations or API routes; let the implementer design the precise schemas to satisfy the UI and Agent state transitions.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
