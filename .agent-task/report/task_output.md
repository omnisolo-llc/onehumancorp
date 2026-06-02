issue_title: "[Architecture] Zero-Config AI-Powered Subscription & Membership Billing Engine"
issue_description: |
  # Problem Statement
  Small business owners like Leo (music tutor) and Priya (boutique owner) need a way to offer recurring subscriptions and memberships without relying on complex, expensive third-party apps. Existing platforms (Shopify, Wix, Squarespace) often require technical setup, convoluted third-party app integration, and do not provide simple ways for customers to manage subscriptions passwordlessly. Owners struggle with churn, failed payments (dunning), and complex configurations.

  # Research Report
  - **Market Gap:** Current platforms require expensive add-ons (e.g., Recharge, Skio) for basic recurring billing.
  - **OHC Advantage:** OHC can offer an invisible, zero-config subscription engine where merchants simply toggle a product/service to "recurring" and let AI agents (Finance, CRM, Operations) handle the rest.
  - **Competitors:**
    - Shopify: Relies on apps.
    - Wix: Complex setup.
    - Squarespace: Only available on high tiers, mobile management lacking.
    - GoDaddy: Basic options, no AI churn management.

  # Design Doc
  - **Architecture Diagram:** Multi-tenant subscription engine tracking `SUBSCRIPTION`, `INVOICE`, and `LEDGER_ENTRY`. AI departments coordinate for failed payment recovery (Finance Agent) and proactive churn management (CRM Agent).
  - **UI/UX Flow:**
    - Merchant toggles "Make this a recurring subscription" on any product, sets frequency (Weekly, Monthly, Yearly).
    - Customer uses 1-tap Apple Pay/Google Pay checkout.
    - Passwordless magic links sent via SMS for customers to pause, cancel, or update cards.
  - **AI Integration:** Finance AI manages dunning automatically. CRM AI handles proactive notifications and win-back campaigns.
  - **Invariants:** Universal support for physical, digital, and service items. Strict multi-tenant isolation. Zero configuration for the merchant (no webhook setups, no manual dunning schedules).

  # Implementation Prompt
  Implement the core Zero-Configuration Subscription & Membership Billing Engine.
  - **CUJ:** A non-technical merchant can toggle "recurring" on a product/service. Customers subscribe via 1-tap checkout and manage via magic link. AI handles dunning and churn notifications automatically.
  - **Acceptance Criteria:**
    1. Support recurring billing intent creation for any asset.
    2. Finance AI automatically intercepts failed renewals and triggers dunning protocol via SMS/Email.
    3. Passwordless customer management via magic link.
    4. Seamless integration with OHC Storefront and Hybrid Event Mesh.
    5. Mobile-first 375px UI for merchant setup and customer checkout.
    6. Strict multi-tenant isolation via RLS.
    7. No configuration required from merchants for webhooks or retry schedules.

  # Known Issues
  During the initial codebase scan, several compilation errors were discovered regarding missing dependencies across various workspace members. Specifically:
  - `src/agents/builtin/Cargo.toml` is completely empty of its necessary dependencies (`serde`, `tokio`, `prost`, etc.), leading to massive compilation errors in the `ohc_builtin_agent` crate.
  - Resolving those dependencies exposes further missing workspace dependencies inside `server_auth` (e.g., `sqlx`, `jsonwebtoken`, `bcrypt`, `chrono`).
  - These dependency cascades currently prevent successful builds via `cargo build`.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
