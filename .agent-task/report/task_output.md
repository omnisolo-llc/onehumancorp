issue_title: "[Architecture] Zero-Touch Subscription & Membership Engine"
issue_description: |
  # Architect and Implement Zero-Touch Subscription & Membership Engine

  ## Problem Statement
  Small business owners like Leo (music tutor offering weekly lesson packages) and Priya (boutique owner launching a monthly VIP style box) need a way to offer recurring subscriptions and memberships. However, they are forced to navigate complex third-party app stores, figure out Stripe webhooks, manage failed payment retry logic, and deal with "subscription hell" where their platform fee balloons from $29 to $200+ just to add basic recurring billing functionality. They need a system where they just toggle "Make this a monthly subscription" on their phone and the system handles the billing, failed payment retries, customer portal, and access rights automatically.

  ## Research Report
  ### The Small Business "Subscription Gap"
  Recurring revenue is the holy grail for small businesses, but offering subscriptions remains highly technical.
  - **Shopify**: Does not support native subscriptions out of the box without external apps (Recharge, Appstle, Skio) which add $99/mo+ overhead.
  - **Wix**: Offers native subscriptions, but the setup is buried in complex desktop-first dashboard settings and struggles with hybrid models.
  - **Squarespace**: Supports subscriptions only on the highest-tier Advanced Commerce plan. Mobile management is severely lacking.
  - **Patreon / Substack**: Built only for digital creators.

  ### OHC Differentiation
  Our platform must provide an **invisible, native subscription engine**. A merchant should be able to toggle a switch on any product, service, or digital good to make it recurring. The AI Finance and Operations departments handle all backend complexity (dunning, customer notifications, automated pause/resume via SMS, ledger reconciliation) with absolutely zero configuration from the user.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      MERCHANT ||--o{ SUBSCRIPTION_PRODUCT : "creates"
      CUSTOMER ||--o{ SUBSCRIPTION : "subscribes to"
      SUBSCRIPTION_PRODUCT ||--o{ SUBSCRIPTION : "templates"
      SUBSCRIPTION ||--o{ INVOICE : "generates"
      SUBSCRIPTION ||--o{ LEDGER_ENTRY : "records"

      FINANCE_AGENT ||--o{ SUBSCRIPTION : "monitors for dunning/renewal"
      OPS_AGENT ||--o{ SUBSCRIPTION : "triggers fulfillment/booking"
      CRM_AGENT ||--o{ SUBSCRIPTION : "manages churn & loyalty"
  ```

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      participant Storefront as OHC Storefront Edge
      participant SubEngine as Zero-Config Subscription Engine
      participant FinanceAgent as AI Finance Dept
      participant CRM_Agent as AI CRM Dept

      Customer->>Storefront: Taps "Subscribe Monthly" ($50/mo)
      Storefront->>SubEngine: Create Subscription Intent
      SubEngine-->>Customer: Collect Vaulted Payment
      Customer->>SubEngine: Confirm Payment
      SubEngine->>FinanceAgent: Activate Subscription & Vault Token
      FinanceAgent->>SubEngine: Schedule Next Billing Cycle
      SubEngine->>CRM_Agent: Trigger Welcome Flow
      CRM_Agent-->>Customer: SMS: "Welcome to the VIP club! Manage your sub here."

      Note over SubEngine, FinanceAgent: 30 Days Later (Charge Fails)
      SubEngine->>FinanceAgent: Attempt Charge Cycle 2
      FinanceAgent-->>SubEngine: Charge Failed
      FinanceAgent->>CRM_Agent: Trigger Dunning Protocol
      CRM_Agent-->>Customer: SMS: "Hey! Your payment failed. Tap to update your card."
  ```

  ### Mobile UX Flow (375px First)
  1. **Creation**: Merchant taps a single toggle on a product to make it recurring and sets frequency.
  2. **Checkout**: Customer views product on edge-cached storefront. Uses Apple/Google Pay for 1-tap checkout.
  3. **Management (Customer)**: Customers receive SMS notifications before renewals with a passwordless magic link to skip/pause/update.
  4. **Management (Merchant)**: Dashboard shows a simple MRR widget and active subscriber count.

  ### Key Design Decisions
  - **Zero-Config Dunning**: Finance AI optimizes retry days automatically. Merchants are not asked to define retry schedules.
  - **Passwordless Customer Management**: Customers manage subscriptions via SMS magic links, eliminating friction.
  - **Universal Subscription Protocol**: Treats physical goods, digital downloads, and booked services identically at the core data layer.
  - **Strict Multi-Tenant Isolation**: Payment tokens and subscription ledgers are strictly isolated per tenant using Zero-Trust principles.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the core Zero-Configuration Subscription & Membership Billing Engine.
  - **User Journey (CUJ)**: A merchant must be able to toggle "recurring" on any product and define a billing interval. A customer must be able to subscribe via 1-tap checkout and manage their subscription via a passwordless magic link.
  - **Acceptance Criteria**:
    1. The engine supports creating recurring billing intents for any asset type.
    2. Finance AI automatically intercepts failed renewals and triggers dunning protocol.
    3. Customer management (pause/resume/cancel) must be accessible via passwordless magic links.
    4. Provide a 375px mobile-first UI for merchant setup and customer checkout.
    5. Ensure strict multi-tenant isolation.
  - **Constraint**: Do not expose any configuration for webhooks, API keys, or manual retry schedules to the merchant. The system must be "Zero-Config".

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
