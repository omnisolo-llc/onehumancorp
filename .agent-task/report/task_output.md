issue_title: "Unified Mobile Tap-to-Pay & Omnichannel Cart Architecture"
issue_description: |
  ## Problem Statement
  Priya (Boutique Operator, 35) runs a clothing shop where she manages both in-store and online operations. Currently, many small businesses have fragmented systems: one for online commerce (e.g., a website cart) and a separate POS (Point of Sale) terminal for in-store purchases. When Priya wants to quickly ring up a customer in-store using her phone via Tap-to-Pay, she lacks an integrated flow that shares the same inventory, discounts, and customer memory as her online store. This disconnect results in out-of-sync inventory, manual end-of-day reconciliations, and disjointed customer relationships. The owner needs an integrated, mobile-first Tap-to-Pay and omnichannel cart system that unifies in-person and digital sales effortlessly.

  ## Research Report
  ### Market Findings & Competitor Analysis
  - **Shopify POS:** Offers excellent multi-channel integration, but is heavily app-dependent and has a separate, sometimes complex, POS app experience that feels distinct from the web admin.
  - **Square:** The gold standard for mobile POS and Tap-to-Pay, but its online store integration is sometimes seen as secondary or less customizable compared to dedicated e-commerce builders.
  - **Wix/Squarespace:** Both offer some POS integrations, typically partnering with Stripe Terminal or Square, but the UX on mobile for the owner is often clunky.
  - **Stripe Terminal (Tap-to-Pay on iPhone/Android):** Stripe has recently introduced Tap-to-Pay directly on mobile devices without external hardware. This presents a massive opportunity for OHC to offer zero-hardware in-person sales directly from the OHC mobile assistant app.

  ### The Opportunity for OHC
  By integrating Stripe's Tap-to-Pay SDK directly into the OHC Flutter app, Priya can build a cart using the exact same catalog as her online store and accept payment in seconds on her phone. Furthermore, the "Finance & Decision Assistant" agent can immediately factor this transaction into daily summaries, and the "Customer & Relationship Assistant" can automatically text/email a receipt to the customer if they are a known CRM contact.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Omnichannel Cart Initiation:** From the OHC Assistant home screen, Priya taps a persistent FAB or quick-action icon: "New Sale".
  2. **Product Selection:** A fast, search-first interface to add products (with variant selection like Size/Color). An AI helper suggests items based on current trending in-store purchases or known customer preferences if Priya links a profile.
  3. **Payment Method:** The checkout screen shows total, tax, and a large button: "Tap to Pay".
  4. **Tap-to-Pay Execution:** The screen transitions to the native iOS/Android Tap-to-Pay UI (via Stripe SDK). The customer taps their card/phone.
  5. **Post-Sale:** Payment succeeds. The app asks if the customer wants a receipt via text/email. Inventory is instantly decremented. The Assistant feed updates with a "Sale completed" card.

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Priya (Owner)
      participant OHC_App as OHC Flutter App (Mobile)
      participant Stripe_SDK as Stripe Terminal SDK
      participant OHC_API as OHC Backend (Rust)
      participant DB as PostgreSQL
      participant AI_Agent as Finance/Customer Agent

      Priya->>OHC_App: Tap "New Sale", Add Items
      OHC_App->>OHC_API: Create Omnichannel Cart (Tenant ID)
      OHC_API->>DB: Lock Inventory (Row-Level Security)
      OHC_API-->>OHC_App: Cart Created & Ready
      Priya->>OHC_App: Tap "Tap to Pay"
      OHC_App->>OHC_API: Request Stripe ConnectionToken
      OHC_API-->>OHC_App: Return ConnectionToken
      OHC_App->>Stripe_SDK: Initialize & Collect Payment
      Stripe_SDK-->>OHC_App: Payment Intent Confirmed
      OHC_App->>OHC_API: Capture Payment Intent
      OHC_API->>DB: Deduct Inventory, Record Sale Ledger
      OHC_API->>AI_Agent: Event: In-Store Sale Completed
      AI_Agent-->>Priya: Feed Update & Receipt Prompt
  ```

  ### AI Agent Integration Points
  - **Work Triage:** Converts the Tap-to-Pay success event into a summary feed card instead of a disruptive alert.
  - **Customer Assistant:** Automatically identifies the customer if they use a known payment method (Stripe Link) or asks Priya to attach a profile to send a receipt.
  - **Decision Assistant:** Factors the real-time sale into the daily performance summary, explaining to Priya how in-store is performing vs. online.

  ### Key Design Decisions
  - **Zero Hardware:** Rely purely on iOS/Android native Tap-to-Pay via Stripe Terminal SDK to reduce the barrier to entry for small businesses.
  - **Unified Ledger:** The database schema must treat in-store and online carts symmetrically (`channel: 'in_store' | 'online'`) so inventory logic is identical.
  - **Tenant Isolation:** All Stripe API calls and cart operations must be strictly scoped to the `tenant_id` via Postgres RLS.

  ## Implementation Prompt
  **Outcome:** Implement the backend foundation for an Omnichannel Cart and Stripe Terminal Connection for Tap-to-Pay.
  **CUJ:** Priya opens the app, creates a manual cart containing an in-stock variant of a dress, and requests to collect payment via Tap-to-Pay. The system provisions a Stripe connection token and returns the finalized cart state.
  **Acceptance Criteria:**
  - Create a unified `Cart` and `CartItem` data model that supports an `in_store` channel flag.
  - Expose API endpoints to create a cart, add items (checking inventory constraints), and transition the cart to a pending payment state.
  - Integrate a Stripe service module to generate a Terminal ConnectionToken for the tenant.
  - Ensure all database interactions utilize the established multi-tenant RLS context.
  - Include E2E tests simulating the cart creation and connection token generation for an authenticated mobile user.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []