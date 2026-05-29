issue_title: "Implement Multi-Tenant Deposit Ledger & Unified Booking Architecture"
issue_description: |
  # Multi-Tenant Deposit Ledger & Unified Booking Architecture

  ## Problem Statement
  Small business owners, especially those offering services (like Carlos, the handyman) or custom orders (like Maya, the baker, and Leo, the music tutor), struggle with collecting and managing deposits. Currently, OneHumanCorp (OHC) lacks a native, unified architecture for taking a partial payment (deposit) to secure a booking or an order, tracking the outstanding balance, and automatically invoicing for the remainder upon completion. This forces users to juggle multiple tools (e.g., Venmo for deposit, a separate calendar app, and manual messaging), breaking the seamless 10-minute zero-to-live promise. We need a core structural capability to tie a booking/order entity to a multi-tenant ledger that supports split payments, deposits, and automated follow-ups.

  ## Research Report
  ### Market Context & Competitor Analysis
  - **Shopify:** Uses third-party apps for robust deposit management, though native partial payments exist. The UX can be disjointed and requires manual setup.
  - **Wix:** Has built-in deposit functionality for its Booking app, but it's tightly coupled to appointments, limiting use cases for physical custom orders (like cakes).
  - **Stripe:** Offers robust PaymentIntents and partial captures, but requires significant engineering to map this to a specific business domain entity (like a booking or a custom physical product).
  - **Square:** Handles deposits well for in-person and online services, but their UI is heavily skewed towards physical POS.

  ### Findings for OHC
  Our core persona needs a system that is fundamentally agnostic to *what* is being sold (a cake, a plumbing fix, a music lesson) but strictly enforces *how* it's paid for over time. The gap in OHC is a unified `Ledger` that seamlessly integrates with the `Booking/Order` system, allowing AI agents to understand the state of a transaction ("deposit paid, pending fulfillment, final invoice sent").

  ## Design Doc

  ### Key Design Decisions
  1.  **Unified Ledger Model:** Treat both physical custom orders and time-based bookings as `Commitments`. A `Commitment` has a lifecycle and is linked to a `LedgerAccount` for the specific tenant.
  2.  **AI Invoicing Automaton:** The system will not rely on the business owner to remember to send the final invoice. An AI Finance Agent will automatically trigger the final payment request based on the state of the `Commitment` (e.g., date of lesson passed, cake marked as picked up).
  3.  **Zero Trust Multi-Tenancy:** Each `LedgerAccount` is strictly isolated by TenantID. All operations on the ledger must include the TenantID and validate via the SPIFFE/SPIRE identity framework.

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ COMMITMENT : manages
      TENANT ||--o{ LEDGER_ACCOUNT : owns
      COMMITMENT ||--o{ LEDGER_ENTRY : generates
      LEDGER_ACCOUNT ||--o{ LEDGER_ENTRY : contains

      TENANT {
          string id PK
          string name
      }
      COMMITMENT {
          string id PK
          string tenant_id FK
          string type "booking | custom_order"
          string status "pending | deposit_paid | fulfilled | fully_paid"
          float total_amount
          float deposit_amount
          datetime due_date
      }
      LEDGER_ACCOUNT {
          string id PK
          string tenant_id FK
          float balance
      }
      LEDGER_ENTRY {
          string id PK
          string account_id FK
          string commitment_id FK
          float amount
          string type "deposit | final_payment | refund"
          datetime timestamp
      }
  ```

  ### AI Agent Integration Points
  -   **Finance & Operations Agent:** Monitors the `COMMITMENT` table. When a status changes to `fulfilled` and the balance is > 0, it automatically drafts and sends a final invoice via SMS/Email to the customer.
  -   **Customer Success Agent (CS):** Reads the `LedgerAccount` and `Commitment` state to answer customer inquiries like, "Did you get my deposit?" or "How much do I still owe?"

  ### Mobile-First UX Flow & UI Wireframes (375px viewport)
  **Screen 1: Create Commitment (e.g., Custom Cake Order)**
  -   **Top Bar:** Translucent glass effect, "New Order" title, Back arrow.
  -   **Body:** Clean, Ubiquiti UniFi style modular cards.
      -   *Card 1 (Details):* Customer Name, Description (e.g., "Vegan Chocolate Cake").
      -   *Card 2 (Pricing):* Total Price Input (large typography).
      -   *Card 3 (Deposit):* Toggle switch "Require Deposit". If ON, reveals a slider or input for percentage/flat amount. Defaults to 50%.
  -   **Bottom Action Bar:** Sticky, full-width primary button "Send Quote & Request Deposit".

  **Screen 2: Customer Payment Link (Web View)**
  -   **Body:** Business logo top center. Large text: "Deposit for Custom Cake".
  -   **Details:** "Total: $100. Due Now: $50."
  -   **Action:** Apple Pay / Google Pay instant checkout button.

  **Screen 3: Owner Dashboard (Post-Deposit)**
  -   **Body:** List of active commitments.
  -   **Item Card:** "Vegan Chocolate Cake - Maya". Status chip: "Deposit Paid". Secondary text: "$50 remaining".
  -   **Action:** Swipe right to mark "Completed & Request Final Payment".

  ## Implementation Prompt
  **Context:** We are implementing the Multi-Tenant Deposit Ledger to allow businesses to require partial upfront payments for services or custom goods.

  **Task:** Implement the backend domain logic and API endpoints for the `Commitment` and `LedgerEntry` entities as described in the architectural design.
  1.  **Endpoints:** Create REST/gRPC endpoints (or equivalent in our Rust backend) to create a Commitment with a required deposit, record a deposit payment (LedgerEntry), and calculate the outstanding balance.
  2.  **Multi-Tenancy:** Ensure all data access is scoped to the `tenant_id` of the authenticated request.
  3.  **AI Trigger:** Implement an event hook (e.g., publishing to a message queue or calling an internal service) when a `Commitment` is marked 'fulfilled' so the Finance Agent can issue the final invoice.
  4.  **Acceptance Criteria:**
      - A user can create an order for $200 with a $50 deposit requirement.
      - The system correctly reports the balance as $150 after the deposit is paid.
      - The tenant data is isolated; Tenant A cannot access Tenant B's ledger.
      - Do *not* implement the actual third-party payment gateway integration (Stripe/PayPal) in this task; mock the successful payment callback for now.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
