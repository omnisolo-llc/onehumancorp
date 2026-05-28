issue_title: "Multi-Tenant Edge-Caching Dynamic Storefront Architecture"
issue_description: |
  ## Problem Statement
  Latency and poor loading times on low-bandwidth networks negatively impact conversions for our personas (Maya the baker, Priya the boutique owner, Fatima the food cart operator). Storefronts need sub-50ms Time-To-First-Byte (TTFB) while supporting real-time inventory updates (e.g., sold-out toggles) to ensure smooth operations even under high load or poor connectivity. Our users run their businesses entirely from their phones, and their customers often access these storefronts from mobile devices on 3G/4G connections.

  ## Research Report
  ### Competitive Analysis
  - **Shopify:** Utilizes a globally distributed edge network with stale-while-revalidate caching and edge workers for personalized rendering.
  - **Wix:** Employs targeted cache invalidation and edge-side rendering to serve static assets while dynamically fetching tenant-specific data.
  - **Stripe:** Offers instant localized loading for payment pages with a heavy emphasis on edge-cached static shells and dynamic API payloads.

  ### OHC's Gap
  Currently, our storefronts lack a unified edge-caching layer capable of achieving sub-50ms latency across global multi-tenant deployments. We need an architecture that merges static edge caching with Zero-Trust multi-tenant data isolation and instant invalidation triggered by AI Agent operations (e.g., when an AI automatically updates inventory after a sale).

  ## Design Doc

  ### Key Design Decisions
  1. **Edge-Rendered Shells:** Storefronts are delivered as edge-cached, static HTML/App shells globally.
  2. **Multi-Tenant Cache Isolation:** Strict namespace isolation via Zero-Trust SPIFFE/SPIRE ensuring tenant A cannot access tenant B's cached catalog.
  3. **Event-Driven Cache Invalidation:** When the AI Operations Department alters state (e.g., marks an item sold out), it fires a gRPC event to invalidate only the specific edge cache keys.
  4. **Optimistic Offline Updates:** For business owners (e.g., Fatima updating her menu), offline changes are saved locally and synced in the background.

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Mobile Customer] -->|Requests Storefront| B(Edge CDN/Cache)
      B -->|Cache Miss / Dynamic Payload| C[API Gateway]
      C --> D[Storefront Rendering Service]
      D --> E[(Multi-Tenant Catalog DB)]

      F[AI Operations Agent] -->|Updates Inventory| E
      F -->|Triggers Cache Invalidation| G[Cache Invalidator]
      G -->|Purge Keys| B
  ```

  ### Mobile UX Flow (375px First)
  1. **Customer View:** User opens Maya's link-in-bio. The edge cache instantly serves the glassmorphic, macOS-style storefront shell (TTFB < 50ms).
  2. **Data Hydration:** Dynamic elements (current stock, personalized recommendations) load asynchronously via small JSON payloads.
  3. **Owner View (Offline):** Fatima uses her low-end Android phone offline to mark "Chicken over Rice" as sold out. The app optimistically updates the UI.
  4. **Sync & Invalidate:** Upon regaining connectivity, the app syncs the change, the AI Agent updates the database, and the cache is targeted-purged.

  ## Implementation Prompt
  **Task:** Implement the Multi-Tenant Edge-Caching Dynamic Storefront architecture.

  **CUJ:**
  1. Maya (baker) adds a new "Vegan Chocolate Cake" from her phone.
  2. The AI Operations department detects the catalog update and instantly invalidates the specific edge cache for her storefront.
  3. A customer opening her Instagram link immediately sees the new cake loaded in under 50ms.

  **Acceptance Criteria:**
  - Establish the edge caching strategy (e.g., Stale-While-Revalidate) with a strict sub-50ms TTFB target for the initial shell.
  - Implement Zero-Trust multi-tenant cache isolation using SPIFFE identity rules so cached data cannot bleed across tenants.
  - Create the invalidation hook that the AI Agent departments can trigger upon state changes.
  - Ensure the Mobile UX is flawless on a 375px viewport with a premium translucent glass aesthetic.
  - Do not prescribe specific lower-level cache implementations (e.g., Varnish vs Redis vs Cloudflare), but ensure the system guarantees the architectural constraints.

  **Estimated Scope:** Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
