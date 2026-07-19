issue_title: "Architecture: Universal Smart Deposits & Milestone Payments Engine"
issue_description: |
  ## Mission Queue Protocol Brief
  This report defines the architecture and implementation plan for the **Universal Smart Deposits & Milestone Payments Engine**, a critical capability for service-oriented and custom-product business owners using OneHumanCorp (OHC).

  ## Problem Statement
  Small business owners like Maya (Custom Baker), Carlos (Field Service), and Leo (Tutor) require upfront financial commitment to secure bookings and begin custom work. Standard e-commerce platforms default to full-payment checkouts, forcing owners to use clunky third-party invoicing or manual tracking for deposits.

  OHC needs a native, mobile-first smart deposit system that allows owners to seamlessly request partial payments. This system must integrate instantly with bookings, AI agent drafts, and final invoicing, without requiring any technical configuration or complex accounting from the owner.

  ## Research Report
  **Market & Competitive Analysis:**
  - **Shopify:** Requires expensive third-party apps (e.g., Globo, Zapiet) for robust deposit and partial payment functionality, adding to the "app tax" and creating a disjointed mobile experience.
  - **Square Appointments:** Handles deposits well but is isolated from general physical product commerce (like Maya's cakes).
  - **Wix:** Offers basic deposit features, but the configuration interface is desktop-centric and complex.

  **OHC's Opportunity:**
  Make partial payments, milestones, and deposits a core architectural primitive deeply integrated with the OHC AI Agent Swarm. Instead of the owner manually remembering to collect the final balance, the **Finance Agent** will automatically monitor the order timeline and push an actionable card to the owner's feed (e.g., "Request final $50 from Carlos before tomorrow's delivery?").

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Mobile App (375px)
          QuoteUI[Quote/Booking UI]
          ActionFeed[Owner Action Feed]
      end
      subgraph Backend (Go)
          OrderSvc[Order & Booking Service]
          PaymentSvc[Payment & Ledger Engine]
          Stripe[Stripe Payment Intents]
      end
      subgraph AI Agent Swarm
          SalesAgent[Sales & Revenue Assistant]
          FinanceAgent[Finance & Decision Assistant]
      end

      QuoteUI -->|Initiate Deposit Quote| OrderSvc
      OrderSvc -->|Create PaymentIntent| PaymentSvc
      PaymentSvc <--> Stripe
      OrderSvc -->|State Change: Deposit Paid| SalesAgent
      SalesAgent -->|Draft Thank You / Next Steps| ActionFeed
      PaymentSvc -.->|Schedule Remainder Collection| FinanceAgent
      FinanceAgent -->|Push 'Request Balance' Card| ActionFeed
  ```

  ### Mobile UX Flow (375px First)
  1. **Quote Creation:** When Maya creates a custom quote from an Instagram DM conversation, a clean bottom sheet appears. A toggle switch says "Require Deposit". She can select "50%" or enter a fixed amount using the native mobile numpad.
  2. **Customer View:** The customer receives a responsive link clearly showing: "Total: $100 | Due Now: $50". The checkout natively supports Apple Pay / Google Pay for friction-free conversion.
  3. **Owner Feed (Triage):** Once paid, Maya's unified feed shows a premium green status card: "Sarah paid $50 deposit. Order #123 moved to Production."
  4. **Automated Balance Collection:** 24 hours before the scheduled delivery or service, the Finance Agent pushes an action card to Maya's feed: "Send final $50 payment link to Sarah?" with a 1-tap "Send" button.

  ### AI Agent Integration Points
  - **Sales & Revenue Assistant:** Intercepts quote creation and suggests a deposit amount based on the business type and transaction size (e.g., suggesting a 50% deposit for custom cakes over $100).
  - **Finance & Decision Assistant:** Monitors the ledger and the timeline of deposit-backed orders, automatically scheduling and drafting balance collection reminders.

  ### Key Design Decisions
  - **Core Primitive:** Treat "Deposit" and "Milestone" as core properties of an `Order` or `Booking` entity in PostgreSQL, not as separate "dummy" products.
  - **Idempotent API:** All payment state mutations must use robust idempotency keys to handle flaky cellular networks gracefully.
  - **Zero-Config Flow:** The user should never see terms like "Payment Intents" or "Webhooks".

  ## Implementation Prompt
  **Role:** Implementer Agent

  **Goal:** Implement the Universal Smart Deposits Engine in the Go backend and Flutter frontend, allowing owners to require partial upfront payments for quotes and bookings effortlessly.

  **Acceptance Criteria:**
  1. Extend the `Order` and `Payment` gRPC protobuf schemas to support `deposit_amount`, `remaining_balance`, and `milestone_schedule`.
  2. Update the PostgreSQL schema with necessary multi-tenant (`tenant_id`) and Row-Level Security (RLS) constraints for partial payments.
  3. Implement a 375px-optimized bottom sheet in the Flutter app for adding a deposit requirement to a quote or booking.
  4. Integrate the Finance Agent (via the AI Job Queue) to monitor deposit-backed orders and trigger an actionable card in the unified feed for remaining balance collection.
  5. Provide a Playwright E2E test verifying the complete Critical User Journey (CUJ): An owner logs in, creates a quote with a 50% deposit, the simulated customer pays, the owner receives a feed notification, and the remaining balance action card appears.

  ## Priority
  P1

  ## Estimated Scope
  Medium-Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
