issue_title: "Architectural Gap: Autonomous AI-Agentic Quoting, Estimating & Deposit Architecture"
issue_description: |
  # Title: Autonomous AI-Agentic Quoting, Estimating & Deposit Architecture

  ## Problem Statement
  For non-technical owners running service-based or custom-order businesses (e.g., Carlos the handyman, Nora the agency principal, Maya the custom baker), the gap between receiving an inquiry and securing a commitment (a paid deposit) is highly manual, disjointed, and prone to leakage. Today, these owners juggle DMs, emails, phone calls, separate invoicing tools, and calendar checks to formulate a quote. There is no unified, zero-config architecture in OneHumanCorp that allows an AI Sales Assistant to automatically draft a contextual quote based on prior jobs, verify availability, send a localized estimate to the customer, and securely process a multi-tenant deposit, all while maintaining seamless 375px mobile-first usability.

  ## Research Report
  ### Competitive Analysis & Market Findings
  - **Square Invoices & Estimates:** Offers robust estimating with one-click conversion to invoices. However, it requires manual data entry and lacks autonomous conversational context (the owner has to build the quote manually).
  - **Shopify & Shopify Sidekick:** Primarily built for static e-commerce (SKUs). Custom quotes and B2B wholesale require expensive third-party apps, breaking the native unified experience for small service operators.
  - **Durable:** Generates generic lead-capture forms, but falls short on dynamic quoting and deposit handling based on complex conversational context.
  - **Jobber / ServiceTitan:** Excellent at field service dispatch and quoting, but overwhelmingly complex for micro-operators (Maya, Leo) and require significant upfront configuration and training.
  - **Gap in Market:** An AI-native, zero-setup quoting engine that reads conversational intent (from unified inbox), references a tenant's historical pricing memory, and auto-drafts an actionable Estimate/Deposit link natively integrated with Stripe Checkout.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ QUOTE : generates
      QUOTE ||--o{ QUOTE_LINE_ITEM : contains
      QUOTE ||--|| CONVERSATION : contextualizes
      QUOTE ||--|| STRIPE_PAYMENT_INTENT : secures_deposit
      QUOTE {
          uuid id
          uuid tenant_id
          uuid customer_id
          string status "draft, sent, accepted, rejected, expired"
          decimal total_amount
          decimal required_deposit
          datetime expires_at
      }
      QUOTE_LINE_ITEM {
          uuid id
          uuid quote_id
          string description
          decimal unit_price
          int quantity
      }
      STRIPE_PAYMENT_INTENT {
          uuid id
          uuid quote_id
          string stripe_pi_id
          string status
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **Work Triage Feed (Home):** The owner sees an AI-flagged actionable item: "New Inquiry: Carlos needs a roof repair estimate. AI has drafted a $450 quote based on similar past jobs."
  2. **Quote Review Screen:** Tapping the item opens a 375px optimized card layout. A translucent glass-styled panel displays the drafted quote line items. The owner can tap any line to adjust the price via a native numeric keypad.
  3. **One-Tap Approval:** A sticky bottom action bar (44x44px touch target) contains a primary "Approve & Send Deposit Link" button.
  4. **Customer Experience:** The customer receives an SMS/Email link, opening a mobile-responsive, edge-cached dynamic storefront page to review the quote and seamlessly pay the deposit via Apple Pay/Google Pay.
  5. **Status Update:** Once paid, the quote transitions to `accepted`, the calendar is blocked, and the Work Triage Feed notifies the owner.

  ### AI Agent Integration Points
  - **Customer & Relationship Assistant:** Extracts intent, constraints, and requested services from incoming inbound messages.
  - **Sales & Revenue Assistant (The Quoter):** Triggered via background job (PostgreSQL SKIP LOCKED queue). Queries the tenant's historical job ledger and pricing vectors to draft a `QUOTE` record.
  - **Operations Assistant:** Verifies tenant calendar availability before proposing dates in the quote.

  ### Key Design Decisions
  - **Row-Level Security (RLS):** All `QUOTE` and `QUOTE_LINE_ITEM` tables will strictly enforce PostgreSQL RLS using `tenant_id` to guarantee cross-tenant data isolation.
  - **Idempotency & Stripe Integration:** Quote-to-deposit payment links must utilize idempotency keys to prevent double-charging on flaky mobile networks (crucial for field workers like Carlos).
  - **Offline-Tolerant Reads:** The mobile application must cache drafted quotes locally using a unified mobile state management approach, allowing operators to review quotes even in low-connectivity areas.

  ## Implementation Prompt
  **Role:** Implementer Agent
  **Objective:** Implement the backend domain logic, gRPC/REST APIs, and Flutter frontend components for the Autonomous Quoting & Deposit feature.

  **Critical User Journey (CUJ):**
  1. The owner navigates to the Work Triage screen on a 375px viewport.
  2. The owner views an AI-drafted Quote containing at least one line item and a calculated deposit amount.
  3. The owner adjusts the price of a line item using native mobile inputs.
  4. The owner taps "Approve & Send", which generates a shareable payment link securely tied to the tenant's Stripe configuration.

  **Acceptance Criteria:**
  - Define the `Quote` and `QuoteLineItem` data models with strict tenant isolation (RLS).
  - Implement a backend service layer with 100% test coverage that handles quote drafting, updating, and state transitions (`draft` -> `sent` -> `accepted`).
  - Create the Flutter UI using the OHC Premium Token library (macOS Translucent Glass styles). The layout MUST not require horizontal scrolling on a 375px device.
  - Ensure all interactive elements (buttons, inputs) are at least 44x44px.
  - Build at least one comprehensive Playwright E2E test verifying the owner can review, edit, and approve a drafted quote using real (non-mocked) backend data.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
