issue_title: "[Architecture] Instant Localized Invoicing Ledger Implementation"
issue_description: |
  # Instant Localized Invoicing Ledger

  ## 1. Problem Statement
  Service-based and non-standard small businesses (like Carlos, the handyman, or Maya, the baker, dealing with custom orders) face significant friction when collecting payments. Generating professional invoices is often a desktop-first, multi-step process in traditional tools (e.g., Quickbooks, Xero). Managing deposits, chasing unpaid invoices, and handling localized tax and currency requirements manually consumes hours and creates cash flow anxiety. A non-technical small business owner needs a way to generate a localized, professional invoice in seconds from their phone, with an AI agent taking over the entire follow-up and collection process invisibly.

  **Market Gap:** There is no platform that allows a user to say "Send a $200 invoice to John for the plumbing fix" into their phone and have a perfectly formatted, legally compliant, localized invoice generated, sent via SMS/WhatsApp, and automatically chased by an AI agent until paid.

  ## 2. Research Report
  - **Competitor Analysis:**
    - **Shopify:** Primarily designed for physical product checkout. Invoicing (draft orders) is clunky and not optimized for mobile-first service businesses.
    - **Wix:** Has basic invoicing, but it feels disconnected from the core CRM and lacks robust autonomous follow-up capabilities.
    - **Square / Quickbooks:** Powerful but overwhelming. UI is highly technical, and the mobile experience often hides core features behind complex menus. They lack native, integrated AI for proactive collections.
    - **Stripe Invoicing:** Developer-first. The Dashboard is not designed for a "grandmother test" level user like Fatima or Carlos.
  - **OHC Differentiation:** The Finance & Payments agent handles this invisibly. It observes an order or a quote approval, determines the customer's location, applies the correct local tax rules (VAT, GST, State Sales Tax), and instantly generates a localized invoice with a payment link, all recorded seamlessly into the underlying multi-tenant ledger.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ CUSTOMER : "serves"
      MERCHANT ||--o{ INVOICE : "issues"
      CUSTOMER ||--o{ INVOICE : "receives"
      INVOICE ||--o{ INVOICE_LINE_ITEM : "contains"
      INVOICE ||--o{ PAYMENT_EVENT : "tracks"
      INVOICE {
          string id
          string merchant_id
          string customer_id
          string status "DRAFT, SENT, PARTIAL, PAID, OVERDUE"
          float total_amount
          string currency
          datetime due_date
      }
      PAYMENT_EVENT {
          string id
          string invoice_id
          string type "DEPOSIT, FULL, REFUND"
          float amount
          datetime timestamp
      }
  ```
  ```mermaid
  sequenceDiagram
      participant Merchant (Mobile App)
      participant OHC Gateway
      participant AI Finance Agent
      participant Ledger / Tenant DB
      participant Customer (SMS/WhatsApp)

      Merchant (Mobile App)->>OHC Gateway: "Create invoice for John: $150 Sink Repair"
      OHC Gateway->>AI Finance Agent: Parse intent, draft invoice
      AI Finance Agent->>Ledger / Tenant DB: Fetch tax rules & customer details
      AI Finance Agent-->>OHC Gateway: Formatted Invoice & Payment Link
      OHC Gateway-->>Merchant (Mobile App): Preview UI (375px)
      Merchant (Mobile App)->>OHC Gateway: Tap "Send"
      OHC Gateway->>Customer (SMS/WhatsApp): Deliver localized payment link

      loop Autonomous Collection
          AI Finance Agent->>Ledger / Tenant DB: Check invoice status daily
          alt Overdue & Unpaid
              AI Finance Agent->>Customer (SMS/WhatsApp): Send polite reminder
          end
      end
  ```

  ### Core Components
  1.  **AI Finance Agent:** Formats invoice, applies local taxes, generates web-based payment link.
  2.  **Ledger/Tenant DB:** Handles strict tenant isolation, storing invoice status, tracking payment events, and executing state transitions (Draft -> Sent -> Paid -> Overdue).
  3.  **Autonomous Collection loop:** Checks invoice status daily, and sends polite reminders on overdue payments.

  ### Mobile UX Flow (375px First)
  - **Screen 1:** Translucent glass card labeled "Get Paid".
  - **Screen 2:** Natural Language Input. "Who is this for?", "What did you do?", "How much?".
  - **Screen 3:** Magic Preview of a beautiful, professional invoice. "Send via WhatsApp".
  - **Screen 4:** Collection Dashboard. Unified view of "Money Out" and "Money In" with status tags.

  ## 4. Implementation Prompt
  **To the Implementer Swarm:**
  Implement the backend framework for the Instant Localized Invoicing Ledger.
  - **Outcome:** A secure, multi-tenant capable API for creating and managing invoices and line items. A state machine handling invoice transitions. Integration hooks for the AI Finance Agent to begin autonomous collections.
  - **CUJ:** Leo navigates to Finance tab, taps "New Invoice", selects customer and service, system calculates tax and generates preview, he taps "Send". Invoice is recorded in the ledger and sent to customer.
  - **Acceptance Criteria:**
    1.  Create `src/server/domain/finance.rs` modeling the Invoice, Line Item, and Payment entities.
    2.  Write migration scripts (e.g. `src/server/db/migrations/015_finance_ledger.sql`) to define the tables for invoices, line items, and payment events.
    3.  Create API endpoints in `src/server/api/finance.rs` to support creating, reading, and transitioning invoice state.
    4.  Ensure strict multi-tenant isolation.
    5.  Implement comprehensive unit testing, and E2E testing using Playwright to mimic Leo's user journey.

  ## 5. Priority & Scope
  - **Priority:** P0
  - **Estimated Scope:** Medium

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
