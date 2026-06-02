issue_title: "[Architecture] Zero-Config AI-Powered Subscription & Membership Billing Engine"
issue_description: |
  # Zero-Configuration AI-Powered Subscription & Membership Billing Engine

  ## Problem Statement
  Small business owners like Leo (music tutor) and Priya (boutique owner) need a way to offer recurring subscriptions and memberships. However, they hit a wall when trying to set this up on existing platforms. They are forced to navigate complex third-party app stores, figure out Stripe webhooks, manage failed payment retry logic, and deal with "subscription hell." They just want to tap "Make this a monthly subscription" on their phone and have the system handle everything invisibly.

  ## Research Report
  - **Shopify**: Does not support native subscriptions out of the box without relying on external apps (like Recharge, Appstle, or Skio), which add monthly fees and require complex configuration.
  - **Wix**: Offers native subscriptions, but the setup is buried in complex desktop-first dashboard settings. It struggles with hybrid models (e.g., booking a service + a product subscription).
  - **Squarespace**: Supports subscriptions, but only on the highest-tier Advanced Commerce plan. The mobile management for merchants is severely lacking.
  - **GoDaddy**: Basic subscription options exist, but lack AI-driven churn management or flexible pause/resume flows for customers.

  **Opportunity**: Our platform must provide an **invisible, native subscription engine**. A merchant should be able to toggle a switch on any product, service, or digital good to make it recurring. The AI Finance and Operations departments handle all the backend complexity: dunning (failed payment retries), customer notifications, automated pause/resume actions via SMS, and ledger reconciliation—with absolutely zero configuration from the user.

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
      SubEngine-->>Customer: Collect Vaulted Payment (Apple Pay / GPay)
      Customer->>SubEngine: Confirm Payment
      SubEngine->>FinanceAgent: Activate Subscription & Vault Token
      FinanceAgent->>SubEngine: Schedule Next Billing Cycle
      SubEngine->>CRM_Agent: Trigger Welcome Flow
      CRM_Agent-->>Customer: SMS: "Welcome to the VIP club! Manage your sub here."
  ```

  ### Mobile UX Flow & AI Integration
  1. **Creation**: Merchant creates a product/service and taps a single toggle to make it recurring. They set the frequency. Done.
  2. **Checkout**: Customer views the product on the edge-cached storefront. They use digital wallets (Apple/Google Pay) for frictionless vaulting.
  3. **Management**: Customers receive SMS notifications before renewals with a passwordless magic link to skip a month, pause, or update payment methods.
  4. **AI Dunning**: Finance AI automatically intercepts failed renewals and triggers the dunning protocol.

  ## Implementation Prompt
  Implement the core Zero-Configuration Subscription & Membership Billing Engine.
  - **User Journey (CUJ)**: A merchant (non-technical) must be able to toggle "recurring" on any product, service, or digital good and define a billing interval (e.g., monthly). A customer must be able to subscribe via a 1-tap checkout and manage their subscription via a passwordless magic link.
  - **Acceptance Criteria**:
    1. The engine supports creating recurring billing intents for any asset type.
    2. The Finance AI automatically intercepts failed renewals and triggers the dunning protocol.
    3. All customer management (pause, resume, cancel, update card) must be accessible via passwordless magic links.
    4. The solution must integrate seamlessly with our edge storefront and hybrid event mesh, enforcing strict multi-tenant data isolation.
    5. Provide a 375px mobile-first UI for both merchant setup and customer checkout.
  - **Constraint**: Do not expose any configuration for webhooks, API keys, or manual retry schedules to the merchant. The system must be "Zero-Config".

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
