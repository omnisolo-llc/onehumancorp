issue_title: "Implement OHC Unified Multi-Channel Inventory Sync & POS Terminal"
issue_description: |
  # Mission Queue Protocol Brief: OHC Unified Multi-Channel Inventory Sync & POS

  ## Problem Statement
  Currently, OneHumanCorp (OHC) handles online payments efficiently via Stripe Checkout. However, our real-world personas, like Priya (the boutique owner) and Carlos (the handyman), frequently transact in person. Priya needs to seamlessly ring up customers in her store using her phone (Tap to Pay) or a dedicated card reader, while Carlos needs to accept final payments on-site after a repair. The lack of a native in-person Point of Sale (POS) solution forces users to rely on disjointed third-party systems, breaking the "all-in-one" OHC promise and muddying financial reporting. More importantly, without centralized real-time inventory locking, double-booking occurs when simultaneous online and offline purchases happen.

  ## Research Report
  - **OHC Gaps:** A review of `src/server/integrations/stripe/` reveals implementations for Checkout, Subscription, and Routing, but zero support for an integrated Stripe Terminal POS flow tied to global inventory.
  - **Competitor Landscape:**
    - *Shopify:* Offers a robust POS system with Tap to Pay on iPhone/Android, seamlessly syncing online and offline inventory.
    - *Square:* Dominates offline payments but lacks agentic workflow automation to unify the business operations effortlessly.
  - **Opportunity:** Integrating Stripe Terminal with a Redis Redlock-backed centralized inventory will enable OHC users to accept in-person payments directly within the OHC app, preventing double-booking and consolidating financial reporting.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
    Client[OHC Mobile App - POS View] -->|Request Connection Token| API[OHC Backend API]
    API -->|Create ConnectionToken| Stripe[Stripe API]
    Stripe -->|Token| API
    API -->|Token| Client
    Client -->|Initialize Terminal SDK| Reader[Stripe Reader / Tap to Pay]
    Reader -->|Read Card Data| Client
    Client -->|Create PaymentIntent| API
    API -->|Reserve Inventory Redlock| Redis[(Redis)]
    API -->|Confirm PaymentIntent| Stripe
    Stripe -->|Capture Success| API
    API -->|Update Ledger & Release Lock| DB[(PostgreSQL)]
    API -->|Notify Success| Client
  ```

  ### Mobile UX Flow (375px Mobile First)
  - **Entry:** A "POS" tab in the Operations dashboard on the mobile app.
  - **Selection:** A clean, calculator-like keypad or product selection grid synced with the master catalog.
  - **Action:** A "Charge $X.XX" button triggers the Terminal SDK overlay (Tap to Pay) or connects to a paired Bluetooth reader.
  - **Visuals:** Translucent Glassmorphism overlays and clear typography adhering to OHC's visual standards. Touch targets must be >= 44x44px.

  ### AI Agent Integration Points
  - *Operations ("The Manager"):* Actively monitors stock levels across all channels. It tracks incoming orders, triggers low-stock alerts, coordinates with the sync mechanism to reconcile conflicts, and suggests restock plans.
  - *Finance ("The Accountant"):* Automatically reconciles in-person transactions with online sales, generating unified daily reports.

  ## Implementation Prompt
  As an implementer, your task is to fully integrate Stripe Terminal and centralized inventory locking into OHC.
  1. Add necessary handlers in `src/server/integrations/stripe/terminal.rs` and `src/server/api/terminal_api.rs` to support `ConnectionToken` generation and in-person `PaymentIntent` capture.
  2. Implement Redis Redlock in the inventory reservation service to prevent double-booking during the POS checkout flow.
  3. Ensure the `PaymentIntent` creation flow reserves inventory, and the successful capture webhook deducts it from PostgreSQL.
  4. Build the mobile-first POS UI in the Flutter/Tauri frontend, ensuring all touch targets are >= 44px and layouts are optimized for 375px screens.
  5. The end result should allow a user (like Priya) to open the OHC app, select an item, and successfully prompt a "Tap to Pay" interaction, resulting in a recorded, unified transaction in her dashboard with correctly adjusted global inventory.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
