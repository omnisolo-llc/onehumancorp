issue_title: "[Architecture] Real-Time Multi-Tenant WebSocket Sync Engine for OHC Point-of-Sale (POS) and Storefront"
issue_description: |
  # Research Report: Real-Time Multi-Tenant WebSocket Sync Engine for OHC Point-of-Sale (POS) and Storefront

  ## 1. Problem Statement
  Small business owners who operate across multiple physical and digital locations (like Priya, the boutique owner, or Jun, the location manager) face severe challenges when their Point-of-Sale (POS) systems, e-commerce storefronts, and back-office dashboards do not instantly reflect inventory changes or sales. When a customer buys the last item in-store, the online storefront must instantly reflect it as "out of stock" to prevent double-booking. The current architecture in OHC lacks a unified, multi-tenant real-time sync layer that can push state changes (inventory, orders, agent feeds) reliably and concurrently to mobile POS apps, consumer web storefronts, and owner dashboards.

  ## 2. Research Report
  - **Market Context**: Platforms like Shopify and Square excel in unified commerce because they maintain strict, low-latency synchronization between their backend, POS terminals, and online storefronts. Shopify relies on robust WebSockets and polling fallbacks, while Square utilizes a localized sync protocol that reconciles with the cloud.
  - **The OHC Opportunity**: Implementing a real-time sync engine using WebSockets—backed by Redis Pub/Sub for horizontal scalability—will allow OHC to push real-time updates seamlessly. This will empower the Operations Agent and Finance Agent to act instantly on cross-channel data without refreshing the UI.
  - **Competitor Gaps**: Many basic website builders (Wix, Squarespace) require manual page refreshes to see the latest inventory on the storefront, leading to poor customer experiences and lost revenue when concurrent purchases occur.
  - **Technical Gap Identified**: OHC's current architecture has REST APIs and some webhook endpoints, but a standardized, multi-tenant WebSocket sync gateway (`/api/sync`) that handles connection management, presence, and topic-based subscriptions (e.g., `inventory:{product_id}`, `feed:{tenant_id}`) is missing.

  ## 3. Design Doc

  ### Architecture Overview (Mermaid)
  ```mermaid
  graph TD
      ClientPOS[Mobile POS App] -->|WebSocket| WSSync[WebSocket Sync Gateway]
      ClientWeb[Storefront Web App] -->|WebSocket| WSSync
      ClientOwner[Owner Dashboard] -->|WebSocket| WSSync
      WSSync -->|Subscribe/Publish| RedisPubSub[Redis Pub/Sub]
      RedisPubSub -->|Event| InventoryService[Inventory Service]
      RedisPubSub -->|Event| OrderService[Order Service]
      InventoryService --> DB[(PostgreSQL Ledger)]
      OrderService --> DB
      AgentOps[Operations Agent] -->|Publish| RedisPubSub
  ```

  ### Core Components
  1.  **WebSocket Sync Gateway (Rust/Backend)**: A high-performance WebSocket server handling thousands of concurrent connections. It manages connection state, authenticates clients (using SPIFFE/SPIRE identity or session tokens), and routes messages.
  2.  **Redis Pub/Sub Integration**: The gateway subscribes to relevant Redis channels based on client subscriptions. When backend services (e.g., `InventoryService`) or Agents (e.g., `Operations Agent`) publish events to Redis, the gateway pushes these events to the connected WebSockets.
  3.  **Multi-Tenant Isolation**: Crucially, every WebSocket connection and topic subscription must be strictly scoped by `tenant_id`. A client can only subscribe to topics (e.g., `tenant:{tenant_id}:inventory`) that they are authorized for.
  4.  **Client-Side Sync (Flutter/PWA)**: The frontend clients maintain the WebSocket connection, handle automatic reconnection with exponential backoff, and dispatch received events to update local state management (e.g., Riverpod or Provider in Flutter) to trigger immediate UI re-renders.

  ### Mobile UX Flow
  1.  **In-Store Purchase**: Priya rings up a customer on the mobile POS (375px width). She taps "Checkout".
  2.  **Instant Notification**: As the transaction clears, the backend publishes an `inventory_updated` event to Redis.
  3.  **Storefront Update**: A customer browsing the web storefront on their phone receives the WebSocket event, and the "Add to Cart" button instantly changes to "Sold Out" via an optimistic UI update, preventing a double sale.

  ## 4. Implementation Prompt
  **Feature Name**: Real-Time Multi-Tenant WebSocket Sync Engine

  **User Story**: As a business owner operating both online and in-store, I need my online storefront and POS to instantly reflect inventory and order changes so I don't oversell items or miss new orders.

  **Acceptance Criteria**:
  1.  Create a WebSocket gateway endpoint that accepts client connections and authenticates them securely.
  2.  Implement a subscription mechanism allowing clients to subscribe to specific topics (e.g., `inventory:{tenant_id}`, `orders:{tenant_id}`). Enforce multi-tenant authorization so clients can only subscribe to their own tenant's topics.
  3.  Integrate Redis Pub/Sub so that backend events published to Redis are broadcasted to the correct WebSocket clients.
  4.  Provide a frontend client utility (in the relevant frontend tech stack) that establishes the connection, handles reconnections, and exposes a stream of events for UI components to consume.
  5.  Ensure 100% unit test coverage for the WebSocket gateway logic and publish/subscribe mechanisms.

  **Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
