issue_title: "Offline-Tolerant Operations Feed & Pre-Order System for Low-Bandwidth Mobile"
issue_description: |
  # Research Report: Offline-Tolerant Operations Feed & Pre-Order System for Low-Bandwidth Mobile

  ## Problem Statement
  Food cart operators and local vendors in high-density or emerging areas (like Fatima the Food Cart Operator) frequently experience slow or unstable 3G/4G networks while actively serving customers. Current mobile web applications for order management, including standard Shopify or Wix setups, require continuous network connectivity. When the connection drops, operators lose access to incoming pre-orders, fail to see payment confirmations, and the UI often hangs or resets, causing severe operational disruptions and lost revenue.

  ## Research Report
  - **The "Fatima" Persona:** Fatima handles pre-orders, pickup timing, daily menus, language barriers, and slow mobile data. She needs a simple order list, availability toggles, customer notifications, and clear offline-tolerant flows. She works in an environment where speed and reliability are paramount.
  - **Competitive Landscape:**
    - **Square POS:** Strong offline capability, but requires their hardware or specific apps, and is less integrated with a unified AI assistant for broader workflows.
    - **Shopify/Wix:** Primarily web-based dashboards that fail gracefully but are unusable offline. E-commerce focused rather than localized pre-order focused.
  - **OHC Opportunity:** By building an offline-tolerant, local-first Service Worker (PWA) architecture for the mobile UI, combined with a background sync mechanism, OHC can ensure operators never lose track of an order. The AI Operations Agent can seamlessly queue actions (like "Mark order ready" or "Notify customer of delay") and sync them the moment network connectivity returns.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Browser / PWA 375px] -->|Reads/Writes| B(IndexedDB / Local Cache)
      A -->|Service Worker Intercepts| C{Network Status}
      C -->|Online| D[OHC Backend API gRPC/REST]
      C -->|Offline| E[Background Sync Queue]
      E -.->|Connection Restored| D
      D --> F[PostgreSQL / Redis]
      D --> G[Operations Agent]
      G -->|Push Notification| A
  ```

  ### Mobile UX Flow (375px)
  1. **Dashboard:** Operator opens the app. The "Today's Orders" list loads instantly from `IndexedDB`. A clear indicator shows "Online" or "Offline Mode".
  2. **Incoming Order:** A pre-order arrives while online. It is stored locally.
  3. **Offline Action:** The connection drops. Fatima taps "Mark as Ready". The UI instantly updates to show it's ready (Optimistic UI), and the action is queued in the Service Worker.
  4. **Restoration:** When the network returns, the Service Worker syncs the "Mark as Ready" event to the backend. The Operations Agent then automatically sends an SMS/Push notification to the customer to pick up the food.
  5. **Menu Toggles:** Fatima toggles "Spicy Chicken" to "Sold Out". This is queued and synced similarly, preventing online double-booking even if she temporarily lost signal.

  ### AI Agent Integration
  - **Operations Assistant ("The Manager"):** Intercepts queued offline actions upon sync. It acts as the reconciliation layer—if a customer tried to cancel an order while the operator was offline, the Agent alerts the operator immediately upon sync and drafts a resolution message.
  - **Customer Assistant ("The Ambassador"):** If an order is delayed due to offline sync issues, the Agent can proactively message the customer: "Your order is being prepared, but we're experiencing a network delay. We'll update you momentarily."

  ### Key Design Decisions
  - **Local-First PWA:** Rely heavily on Flutter's web support or native Service Worker caching to store the active order state.
  - **Optimistic UI:** Never block the user with a spinner if the network is flaky. Accept the action locally and sync in the background.

  ## Implementation Prompt
  Implement an offline-tolerant Operations Feed for mobile devices (375px viewport).
  1. Create a `LocalOrderCache` mechanism (using IndexedDB or equivalent) that stores the day's active pre-orders.
  2. Implement an optimistic UI update for order state transitions (e.g., Pending -> Preparing -> Ready).
  3. Create a background sync queue that captures these state changes when offline and flushes them to the `OHC Backend` when online.
  4. Integrate the Operations Agent to handle the synced events and trigger customer notifications (e.g., "Food is ready for pickup").
  5. Ensure the UI clearly indicates the current network status (Online/Offline) and pending sync count to the operator without being intrusive.

  ## Priority
  P0

  ## Estimated Scope
  Medium
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []