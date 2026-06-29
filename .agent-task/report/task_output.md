issue_title: "[Feature] Implement Universal Offline-First Hardware & Thermal Print Mesh"
issue_description: |
  ## Problem Statement
  Small business owners operating in challenging network conditions—such as Fatima managing her food cart in areas with patchy 4G, or Carlos repairing plumbing in a customer's basement with zero reception—face critical operational friction when their point-of-sale (POS), booking, or inventory management tools fail offline. Existing solutions either prevent actions completely without a connection or show generic error states, leading to lost sales and poor customer experiences. We need a robust, offline-first mobile architecture that allows the business to function seamlessly when disconnected, capturing orders, taking cash/offline payments, and securely syncing state back to the OHC backend once connectivity is restored.

  Specifically, businesses need to be able to accept in-person tap-to-pay directly on their smartphones without dongles, and keep the app functional even when the network drops, ensuring their digital and physical storefronts are unified effortlessly.

  ## Research Report
  **Market Competitive Analysis:**
  - **Square POS:** The industry leader in offline mode. Square allows merchants to process cash and offline card payments (with explicit risk disclaimers) seamlessly when the network drops. Transactions are queued locally and synced within 24-72 hours.
  - **Shopify POS:** Offers offline capabilities primarily for cash and custom payment types. Card payments require an active connection. Inventory is cached locally, but true optimistic sync for complex catalog updates is limited.
  - **Wix/Squarespace:** Primarily online-dependent. While they offer mobile apps, robust offline-first POS and inventory management are not deeply integrated at the core edge layer, requiring a solid connection for most management tasks.

  **Our Opportunity:**
  OneHumanCorp can differentiate by treating offline resilience not as a bolt-on feature, but as a core architectural primitive. By employing an Optimistic Mutation Engine with Conflict-Free Replicated Data Types (CRDTs) or a robust local action queue (Local-First architecture), OHC will guarantee that a user (like Maya or Fatima) can manage inventory, process local cash/tap-to-pay offline operations, and rely on the AI Operations Department to resolve state conflicts gracefully in the background without overwhelming the user with technical "sync error" jargon.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as OHC Mobile App (Edge)
      participant SDK as Payment SDK / Local DB
      participant Sync as Background Sync Engine
      participant Cloud as OHC Cloud Platform
      participant AI as Finance Agent

      User->>SDK: Process Tap-to-Pay (Offline)
      SDK-->>User: Success (Payment Queued)
      Note over SDK: Encrypted transaction stored locally

      loop Background Process
          Sync->>SDK: Check for connectivity
          alt Internet Restored
              SDK->>Cloud: Batch upload queued transactions
              Cloud-->>SDK: Ack sync
              Cloud->>AI: Trigger reconciliation
              AI-->>Cloud: Process potential declines
              Cloud->>User: Push notification (Sync Complete)
          end
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
