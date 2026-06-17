issue_title: "Implement Distributed Edge Cache for Catalog and Inventory (EdgeOffload)"
issue_description: |
  **Problem Statement**
  The current OHC architecture serves catalog and inventory directly from the central database or a standard Redis instance in cloud-native mode, and uses simple application logic in standalone mode. This presents a challenge for merchants like Priya (Boutique Operator) and Fatima (Food Cart Operator), who experience high concurrent read traffic during product drops or lunch rushes, while operating over flaky mobile networks. The lack of an edge caching layer means that all read requests hit the backend, leading to increased latency, potential database bottlenecks, and a degraded "first paint" experience on mobile devices (375px). A resilient system requires a distributed edge caching mechanism that can serve high-read volumes with low latency, offloading traffic from the primary datastore while ensuring eventual consistency with inventory mutations (like POS tap-to-pay transactions).

  **Research Report**
  Leading platforms like Shopify (with their dynamic edge caching) and Vercel/Next.js (with ISR) utilize edge caching heavily to serve storefronts with low latency. Our competitor analysis shows that fast initial loads directly correlate with conversion rates. OHC currently implements inventory locks (Redlock in `booking.rs` and `inventory/service.rs`) and CRDT-based offline POS sync (`sync/offline_pos.rs`), but lacks a dedicated read-optimized edge cache layer for the catalog.
  By introducing a distributed edge cache architecture, we can push catalog data and available inventory counts closer to the client. This will involve designing an `EdgeCache` component that can fall back to local memory in standalone mode and leverage a distributed KV store in cloud mode. We also need a mechanism to gracefully invalidate or update the edge cache when the central inventory changes (e.g., via the `inventory.updated` pub/sub event seen in `cache_invalidator.rs`).

  **Design Doc**
  *Architecture Diagram (Mermaid)*
  ```mermaid
  graph TD
      Client[Mobile/Web Client] --> Edge[Edge Cache Service]
      Edge -- "Cache Hit" --> Client
      Edge -- "Cache Miss" --> BE[Core Backend Service]
      BE --> DB[(Primary DB - PostgreSQL)]
      BE -- "Invalidation / Update Event" --> PubSub[Pub/Sub Message Bus]
      PubSub --> Edge
  ```
  *Mobile UX Flow:*
  When a user (e.g., Fatima's customer) opens the storefront on a 375px mobile screen, the catalog items and their availability status (e.g., "Sold Out" toggles) load instantly from the Edge Cache, showing skeleton loaders for less than 100ms.

  *AI Agent Integration:*
  The `OperationsAgent` or `StorefrontAgent` can monitor cache hit rates and inventory levels, proactively notifying the owner if a high-velocity item is about to sell out or if cache performance drops.

  *Key Design Decisions:*
  1.  **Read-Through Cache:** The `EdgeCache` service acts as a read-through cache for the catalog API.
  2.  **Event-Driven Invalidation:** Utilize the existing Pub/Sub infrastructure (`inventory.updated` events) to trigger cache invalidations or partial updates, ensuring near real-time consistency.
  3.  **Hybrid Degradation:** In standalone mode, the Edge Cache degrades to an in-memory LRU cache.

  **Implementation Prompt**
  Implement the `EdgeCache` module for the catalog and inventory read paths. The user-facing outcome is that the storefront loads instantly (under 100ms) even under heavy read load. Ensure that the cache is properly invalidated when inventory is updated or reserved. Write comprehensive unit tests for the caching logic and Playwright E2E tests verifying that the storefront UI correctly displays inventory changes after a purchase, even when served from the cache. Ensure the UI implementation follows the mobile-first (375px) Translucent Glass design system.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report, architecture]
assignees: []
