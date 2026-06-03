issue_title: "[Architecture] Edge-Native Offline-First Local Storefront Discovery Engine"
issue_description: |
  # Edge-Native Offline-First Local Storefront Discovery Engine

  ## Problem Statement
  Small businesses like Fatima's food cart or Carlos's handyman service often operate in areas with poor or intermittent network connectivity (e.g., crowded festivals, basements, or remote job sites). Their customers also face similar connectivity issues. Currently, OHC's storefront relies on cloud-based API calls. If the network drops, customers cannot view menus or book services, and merchants cannot receive or process orders, leading to lost revenue and frustration. They need a system that remains fully functional, fast, and reliable regardless of internet connectivity.

  ## Research Report
  ### The Offline Reliability Gap in SMB Platforms
  - **Shopify**: Offers a robust POS system for in-person sales but its online storefronts and customer-facing apps are highly dependent on continuous internet access. Offline mode is limited primarily to POS terminal queuing.
  - **Square**: Has offline mode for card processing, but the merchant catalog and customer booking/ordering interfaces require connectivity to function seamlessly.
  - **Wix/Squarespace**: Completely cloud-dependent. Storefronts fail to load without an active connection.

  ### OneHumanCorp Differentiation
  OHC will architect an **Edge-Native Offline-First Discovery and Ordering Engine**. By leveraging Progressive Web App (PWA) technologies, local caching (IndexedDB), and background sync via Service Workers, the OHC storefront will load instantly from the edge and remain functional offline. AI agents will manage intelligent data syncing when connectivity is restored, ensuring no data loss and seamless conflict resolution.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      CUSTOMER_APP ||--o{ LOCAL_CACHE : "reads/writes"
      LOCAL_CACHE ||--o{ SERVICE_WORKER : "managed by"
      SERVICE_WORKER ||--o{ OHC_EDGE_NODE : "syncs when online"
      OHC_EDGE_NODE ||--o{ CORE_DATABASE : "persists to"

      %% AI Coordination
      OPS_AGENT ||--o{ OHC_EDGE_NODE : "resolves sync conflicts"
      FINANCE_AGENT ||--o{ OHC_EDGE_NODE : "processes queued payments"
  ```

  ```mermaid
  sequenceDiagram
      autonumber
      actor Customer
      participant Browser as Customer Browser (PWA)
      participant SW as Service Worker
      participant LocalDB as IndexedDB
      participant Edge as OHC Edge Node
      participant Agent as AI Operations Agent

      Note over Customer, SW: Offline State
      Customer->>Browser: Open Fatima's Menu
      Browser->>SW: Fetch Menu Assets
      SW->>LocalDB: Retrieve Cached Menu
      LocalDB-->>Browser: Return Menu (Instant)
      Customer->>Browser: Place Order ($15)
      Browser->>SW: Submit Order
      SW->>LocalDB: Queue Order & Payment Intent
      LocalDB-->>Browser: Show "Order Saved" Success

      Note over Customer, Edge: Online State Restored
      SW->>Edge: Background Sync: Push Queued Orders
      Edge->>Agent: Validate Order Inventory
      Agent-->>Edge: Confirm Order
      Edge-->>SW: Sync Success Acknowledgment
      SW->>Browser: Push Notification "Order Confirmed!"
  ```

  ### Mobile UX Flow (375px First)
  1. **Instant Load**: When a customer opens a storefront link, the Service Worker immediately serves the cached UI and catalog. A subtle "Offline Mode" indicator appears if the network is disconnected.
  2. **Seamless Browsing**: The customer browses the catalog, views images (cached via WebP), and selects variants without any loading spinners.
  3. **Offline Checkout**: Customer adds items to the cart and completes the checkout flow using vaulted payment methods (Apple Pay/Google Pay). The app displays: "Order saved! We'll process it the moment you reconnect."
  4. **Background Sync**: Once the device regains connectivity, the Service Worker automatically pushes the transaction. The customer receives a push notification confirming the order.
  5. **Merchant View**: Fatima's mobile dashboard shows a "Pending Sync" queue. Once online, AI agents automatically process the orders and update inventory.

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors the sync queue. When devices reconnect, it resolves inventory conflicts (e.g., if two offline users ordered the last item, it intelligently assigns it based on timestamp or VIP status and refunds the other with an apology).
  - **Finance Agent**: Safely processes queued offline payment intents and manages any declines via the standard Dunning Protocol once back online.
  - **Business Advisory Agent**: Analyzes offline interaction patterns and suggests optimizations (e.g., "Your customers frequently drop offline at 2 PM; consider pre-caching the lunch menu earlier").

  ### Key Design Decisions
  - **Local-First Data Model**: All reads and writes happen against the local device cache (IndexedDB) first. The network is treated as an asynchronous enhancement, not a dependency.
  - **Optimistic UI Updates**: The UI immediately reflects user actions (adding to cart, booking a slot) without waiting for server confirmation, creating a zero-latency feel.
  - **Conflict-Free Replicated Data Types (CRDTs)**: Implement CRDT-like structures for critical shared state (like inventory counts) to allow safe, deterministic merging of offline actions when syncing to the cloud.

  ## Implementation Prompt
  **To Implementer Agent:**
  Implement the Edge-Native Offline-First Local Storefront architecture.
  - **User Journey (CUJ)**: A customer must be able to load a merchant's storefront, browse products, and submit an order while their device is completely disconnected from the internet. When the device reconnects, the order must seamlessly sync to the OHC backend without user intervention.
  - **Acceptance Criteria**:
    1. Implement a Service Worker strategy that caches the core storefront application shell, product catalog data, and images.
    2. Implement a local IndexedDB queue for capturing offline actions (orders, bookings).
    3. Implement Background Sync API functionality to automatically flush the local queue to the backend upon network restoration.
    4. Provide UI components (toast notifications, offline badges) that clearly but unobtrusively communicate network state and pending syncs to the user.
    5. The backend must include conflict resolution logic to handle out-of-sync inventory updates safely.
  - **Constraint**: Ensure the solution adheres to strict mobile-first design principles on a 375px viewport and maintains premium Translucent Glass aesthetics even in offline mode.

  ## Priority
  P1 (High)
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
