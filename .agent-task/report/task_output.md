issue_title: "Implement Intelligent Tap-to-Pay Visibility and Cash/External Logging for Physical Retail (Priya)"
issue_description: |
  # Architectural Deep Dive: Tap-to-Pay Visibility & Omni-channel Analytics for Physical Retail (The "Priya" Gap)

  ## 1. Problem Statement
  Priya, a boutique operator (Target Persona), needs to manage both online demand and in-store operations. She currently lacks visibility into in-person transactions (Tap-to-Pay, Cash, External Terminals) alongside her online sales within the OHC platform. She needs a unified view of her revenue, inventory deduction based on physical sales, and an easy way to log physical transactions without needing an external POS system. Currently, OHC heavily indexes on digital commerce (cart recovery, digital storefronts) but lacks the first-class physical POS/Cash primitives required for true omni-channel operation by SMBs.

  ## 2. Research Report
  - **The Gap:** OHC's current data model and agent workflows are optimized for digital orders (e.g., Abandoned Cart Recovery, SEO Agent). Physical transactions require different handling: instant clearing (no shipping), immediate inventory deduction without fulfillment flows, and distinct payment methods (Cash, Tap-to-Pay via Stripe Terminal SDK, External Card Readers).
  - **Competitive Analysis:**
    - *Shopify POS:* Offers a robust separate app, but it's disconnected from the core mobile app unless the user pays for higher tiers.
    - *Square:* Built for this, but lacks the advanced AI agent workflows (like the Promoter Agent) that OHC offers.
  - **The OHC Advantage:** By integrating physical transactions directly into the OHC mobile-first interface, Priya gets a single dashboard for *all* sales. The "Finance & Decision Assistant" can then summarize total daily performance (online + offline) automatically.

  ## 3. Design Doc (Architecture)

  ### Architecture & Data Model Adjustments
  - **New Entity / Schema Expansion:** Expand the `orders` or `transactions` table to support `sales_channel` (`ONLINE`, `IN_STORE`) and `payment_method` (`CREDIT_CARD_ONLINE`, `TAP_TO_PAY`, `CASH`, `EXTERNAL_TERMINAL`).
  - **API Layer:** New endpoints to log in-store transactions securely, bypassing traditional checkout/shipping flows.
  - **Agent Integration:**
    - *Finance & Decision Assistant:* Needs to ingest these physical transactions for the "plain-language daily performance summaries".
    - *Operations Assistant:* Needs to immediately deduct inventory for `IN_STORE` sales without creating fulfillment tasks.

  ### Mobile UX Flow (375px First)
  1. **Quick Action Fab:** On the OHC home screen, a prominent "New Sale" FAB (Floating Action Button).
  2. **Cart/Amount Screen:** Priya enters a quick amount or selects from the catalog (with variants like size/color).
  3. **Payment Method:** Large, touch-friendly 44x44px buttons for "Tap to Pay" (integrates with Stripe Terminal if configured), "Cash", or "External Reader".
  4. **Confirmation:** A quick success screen, and the transaction immediately appears in the daily revenue summary.

  ### Zero Trust & Security
  - All physical transaction logging must validate the tenant context via SPIFFE/SPIRE (backend) and ensure row-level security (`tenant_id`) is strictly enforced so cash/tap logs cannot leak across tenants.

  ## 4. Implementation Prompt
  **Role:** Implementer

  **Objective:** Build the unified omni-channel transaction logging flow for physical retail (Priya's Use Case).

  **User Journey:**
  1. Priya opens the OHC mobile app (browser/PWA).
  2. She taps "New Sale" from the unified dashboard.
  3. She selects a product variant (e.g., "Red Dress - Size M") which adds it to a quick cart.
  4. She selects "Cash" as the payment method.
  5. The sale is recorded, inventory is instantly deducted, and the daily summary reflects the new revenue.

  **Acceptance Criteria:**
  - Create/Update database schemas to support in-store channels and payment types (Cash, External).
  - Build the backend API to process these immediate-clearing transactions.
  - Implement the mobile-first (375px) UI for the "New Sale" flow using the OHC Premium Token library (translucent materials, clean hierarchy).
  - Ensure the Operations Agent immediately updates inventory without generating shipping/fulfillment tasks.
  - Ensure the Finance Agent includes these sales in the daily summary.
  - Write at least 5 Playwright E2E tests verifying the "Cash Sale" flow from the home screen to the updated daily summary.
  - 100% Unit test coverage on new backend and frontend code.
  - Verify layout responsiveness down to 375px.

  ## 5. Priority & Scope
  - **Priority:** P1
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
