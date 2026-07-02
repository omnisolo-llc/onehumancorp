issue_title: "[research] Build Mobile Tap-to-Pay & Offline-First POS Engine"
issue_description: |
  # [Mobile Tap-to-Pay & Offline-First POS] Unified Physical and Digital Retail

  ## Problem Statement
  For businesses operating in the real world—like Fatima running a food cart in areas with spotty cell service, or Priya selling clothes at her boutique and at a weekend pop-up market—relying purely on online checkout or external point-of-sale (POS) hardware dongles introduces immense friction. If Fatima loses connectivity, she can't process payments or view her active orders. If Priya has to switch between an online store platform and a separate POS app for in-store sales, her inventory goes out of sync and she has a fragmented view of her business. They need to be able to accept in-person tap-to-pay directly on their smartphones without dongles, and keep the app functional even when the network drops, ensuring their digital and physical storefronts are unified effortlessly.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Offers Shopify POS, which works well but requires a separate app. Tap-to-Pay on iPhone/Android is supported. Offline mode exists but is somewhat restricted depending on the payment terminal used.
  - **Square:** The dominant player for physical POS. Excellent hardware ecosystem and Tap-to-Pay support. However, its e-commerce website builder is often seen as clunky compared to pure digital platforms.
  - **Wix:** Has a POS system and supports Tap-to-Pay via the Wix Owner app, but offline capabilities are limited.

  **Market Needs:**
  The modern solopreneur expects their smartphone to be their entire business operating system. Apple's "Tap to Pay on iPhone" and Google's equivalent for Android have commoditized the payment terminal. By integrating this deeply with an offline-first data synchronization model, OHC can replace Square and Shopify POS entirely for small businesses, without requiring any external hardware.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Device
          App[OHC Mobile App 375px] --> LocalDB[(Local SQLite / CRDT)];
          App --> TapToPay[Native Tap-to-Pay SDK];
          LocalDB --> SyncEngine[Offline Sync Engine];
          TapToPay --> LocalDB: Record Encrypted Payment Intent;
      end

      SyncEngine -- Network Restored --> Gateway[OHC API Gateway];
      Gateway --> Stripe[Stripe Terminal API];
      Gateway --> MainDB[(Cloud Postgres)];
      Gateway --> Agents[AI Agent Swarm];

      subgraph Agent Departments
          Agents --> OpsAgent[Ops: Update Inventory];
          Agents --> FinanceAgent[Finance: Reconcile Ledger];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Dashboard:** Priya opens the OHC app. A prominent "New Sale" FAB (Floating Action Button) is visible, designed using macOS-style Translucent Glass materials.
  2. **Cart Building:** She taps products from her visual catalog. The app responds instantly (sub-50ms) because it reads from the local CRDT store.
  3. **Checkout:** She taps "Charge $45.00". A bottom sheet slides up asking "Tap to Pay" or "Cash".
  4. **Payment:** She selects "Tap to Pay". The native iOS/Android Tap-to-Pay system UI appears. The customer taps their card against Priya's phone.
  5. **Offline Mode:** If Fatima is offline, the app displays a subtle "Offline - Syncing later" indicator. Cash sales and pre-orders are logged locally. (Tap-to-pay requires network for authorization, but inventory and order management remain fully functional offline).

  ### AI Agent Integration Points
  - **Operations Agent:** Detects when an item is sold out in-person and automatically updates the online storefront to prevent double-selling.
  - **Finance Agent:** Reconciles offline cash transactions with processed Tap-to-Pay transactions to give an accurate daily ledger, summarized in a notification.
  - **Marketing Agent:** If the customer provides an email for a receipt, the agent automatically drafts a personalized "Thank you for visiting my stall!" email to send later.

  ### Key Design Decisions
  - **Offline-First CRDTs:** We must use a local-first database (like SQLite with CRDTs) so the app never blocks on network requests. This solves Fatima's bad connectivity problem.
  - **Dongle-less Payments:** We rely strictly on Apple/Google native Tap-to-Pay SDKs to remove the friction and cost of buying external hardware.
  - **Unified Inventory:** Physical and digital sales hit the same underlying ledger. There is no separate "POS" inventory.

  ## Implementation Prompt
  Implement the Tap-to-Pay and Offline-First POS capabilities.
  - **User-Facing Outcome:** Users can open the mobile app, add items to a cart, and process a payment by having a customer tap a credit card directly on the merchant's phone. The app must remain responsive and allow inventory management even with airplane mode on.
  - **CUJ (Critical User Journey):**
    1. User adds item to cart in the mobile app.
    2. User selects "Tap to Pay".
    3. Customer taps card.
    4. Payment succeeds and inventory decrements.
    5. (Alternative) User is offline, logs a cash sale, and the app syncs the sale to the cloud when reconnected.
  - **Acceptance Criteria:**
    - Native Tap-to-Pay flow is triggered on iOS/Android.
    - App state (inventory, orders) is readable and writable when offline, syncing automatically upon network restoration.
    - No developer jargon (CRDT, sync) is visible to the user.
    - UI strictly adheres to the glassmorphism and card-based design system.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
