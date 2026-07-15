issue_title: "[Architecture] Edge-Native Real-Time Storefront Inventory Sync"
issue_description: |
  # Research Report: Edge-Native Real-Time Storefront Inventory Sync

  ## Problem Statement
  Small business owners using OneHumanCorp (OHC) need their storefronts to load instantly (under 1 second) even on poor 3G connections (e.g. Fatima the food cart owner). To achieve this, OHC uses an Edge-Cached Dynamic Storefront architecture where HTML is pre-rendered and cached at the CDN edge (Cloudflare/NGINX). However, caching HTML at the edge introduces a critical problem: **stale inventory and booking data**. If Priya's boutique sells out of a dress, the cached HTML might still show it as available until the cache is invalidated and regenerated. This leads to customer frustration and overselling. We need an architecture that combines the speed of edge-cached static HTML with real-time inventory accuracy, without overwhelming the backend.

  ## Research Findings
  Our user personas demand both speed and accuracy:
  - **Fatima (Food Cart):** Needs her menu to load instantly via QR code, but when a dish sells out, it must immediately show as "Sold Out" to prevent angry customers ordering unavailable food.
  - **Priya (Boutique):** Needs fast product pages for Google Shopping, but accurate stock levels to prevent double-selling a unique item.

  ### The Gap
  Currently, the `storefront_delivery.go` implementation attempts to solve this by injecting dynamic inventory data *during* the Go backend request (`inject_dynamic_inventory`), but this only happens on a cache miss or during background regeneration. The NGINX edge cache (`nginx.conf`) serves the completely static HTML for up to 60 minutes. There is no mechanism for the edge to reconcile the cached HTML with real-time inventory state without a full cache purge and regeneration, which is too slow and resource-intensive for high-velocity sales.

  ### Competitive Analysis
  - **Shopify:** Uses a mix of SSR and client-side fetching. Critical data (inventory, cart) is often fetched client-side after the initial HTML loads.
  - **Vercel/Next.js (ISR):** Good for content, but struggles with real-time inventory without client-side hydration.
  - **OHC's Differentiation:** We will use **Edge Side Includes (ESI) / Edge-Native Hydration via Redis/KV**. The static HTML skeleton is cached at the edge. The edge worker (or a fast path in the NGINX configuration / CDN layer) intercepts the request, reads the HTML, and *injects* the real-time inventory data (fetched from a low-latency edge KV or Redis) *before* sending it to the client. This guarantees instant load times with 100% accurate inventory.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD
      subgraph Edge "Edge Layer (NGINX / CDN)"
          Worker[Edge Request Interceptor]
          Cache[(HTML Cache)]
          KV[(Inventory Edge KV)]
      end

      subgraph Backend "Go + Bazel Backend"
          API[Storefront Delivery API]
          OpsAgent[Operations Agent]
      end

      subgraph DB
          Postgres[(PostgreSQL)]
      end

      Client[Customer Browser] --> Worker
      Worker -- 1. Read Cached HTML --> Cache
      Worker -- 2. Fetch Real-time Inventory --> KV
      Worker -- 3. Inject Inventory & Return HTML --> Client

      OpsAgent -- Inventory Changes --> KV : Push Updates
      OpsAgent -- Inventory Changes --> Postgres : Persist
  ```

  ### Core Components
  1. **Edge Inventory KV:** A low-latency, globally distributed key-value store (e.g., Cloudflare KV, Fastly Edge Dictionaries, or Redis replication to edge nodes) that stores only the current inventory count/status for each product ID.
  2. **Operations Agent (Inventory Sync):** When inventory changes (via POS, online sale, or manual update), the Operations Agent immediately pushes the new state to the Edge Inventory KV.
  3. **Edge Interceptor (Worker/Proxy):** When a request hits the edge, it retrieves the cached HTML skeleton. It then parses specific placeholder tokens (e.g., `<!-- INVENTORY_STATUS_{product_id} -->`) and replaces them with the real-time data fetched from the Edge Inventory KV *before* returning the response to the user.

  ### Mobile UX Flow (375px First)
  - The customer taps a link to Priya's boutique.
  - The edge node returns the fully styled HTML (375px optimized glassmorphism design) in < 50ms.
  - The HTML already contains the correct "Sold Out" or "X items left" badge because it was injected at the edge. No client-side loading spinners or layout shifts (CLS = 0).

  ### Key Design Decisions
  - **Why not client-side fetch?** Client-side fetching introduces layout shifts (CLS) and delays time-to-interactive, especially on slow 3G networks (Fatima's customers).
  - **Why not full cache invalidation?** Regenerating the entire HTML page for every inventory decrement is inefficient and scales poorly during flash sales.
  - **Security:** The Edge KV is read-only from the public internet. Only the authenticated backend (Operations Agent) can write to it.

  ## Implementation Prompt
  Implement the "Edge-Native Real-Time Storefront Inventory Sync" architecture.
  Modify the storefront delivery pipeline so that the static HTML skeleton is cached, but real-time inventory status is injected at the edge layer (e.g. via NGINX SSI, Cloudflare Workers, or a simulated fast-path middleware in the Go backend that reads from a low-latency Redis/KV store) before the response reaches the client.
  Ensure the Operations Agent pushes inventory updates to this edge KV store.
  The user-facing outcome is that storefronts load instantly (served from cache) but always display 100% accurate, up-to-the-millisecond inventory data, preventing overselling without relying on client-side JavaScript fetching.
  Do NOT prescribe specific database schemas or API endpoints; design the robust injection mechanism.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
