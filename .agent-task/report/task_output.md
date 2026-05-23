issue_title: "[architecture] Edge-Cached Dynamic Storefronts"
issue_description: |
  # [Edge-Cached Dynamic Storefronts] Sub-50ms Loading for Mobile Conversion

  ## Problem Statement
  For businesses running on OneHumanCorp (OHC)—like Maya's custom cake shop on Instagram or Priya's boutique—every millisecond of page load time on a 3G mobile network hurts conversion. Small business owners cannot manually configure CDNs, Varnish caching, or React Suspense boundaries. They need dynamic elements (inventory counts, local availability, dynamic pricing) to appear instantly. Currently, if an OHC storefront takes more than 1 second to load over cellular data, the customer leaves. We must provide the performance of a static site with the real-time capabilities of a dynamic web app, entirely out of the box, with zero configuration.

  ## Research Report
  **Competitive Analysis:**
  - **Shopify:** Utilizes heavily optimized global CDNs (Cloudflare integration) and their Hydrogen framework (React Server Components) for fast loading, but custom storefronts require developer setup.
  - **Wix/Squarespace:** Often suffer from "bloated JS" issues. While they have improved, mobile performance scores on PageSpeed Insights often lag behind hand-coded sites due to generic builder payloads.
  - **Vercel/Next.js:** The gold standard for ISR (Incremental Static Regeneration) and edge caching, but requires deep engineering knowledge.

  **Market Needs:**
  A non-technical merchant needs a storefront that loads in sub-50ms globally, feeling native and instantaneous, while still reflecting real-time inventory (e.g., "Only 1 vegan cake left!").

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Edge Network (Cloudflare/Fastly)
          EdgeCache[Edge Cache] --> User[Mobile User];
          EdgeWorker[Edge Worker - Deno/V8] --> User;
      end

      User -->|Initial Request| EdgeCache;
      EdgeCache -. Miss / Stale .-> Origin[OHC Origin Server];
      EdgeWorker -->|Fetch Dynamic Slices| Origin;
      Origin --> Postgres[(Primary DB)];

      subgraph OHC Backend
          Postgres --> CacheInvalidator[Cache Invalidator Agent];
          CacheInvalidator -->|Purge Tags| EdgeNetwork[Edge Network API];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Link Click:** A customer clicks Maya's Instagram link-in-bio.
  2. **Instant Shell:** The storefront shell (glassmorphism UI, branding, images) loads instantly from the nearest edge cache (sub-50ms).
  3. **Dynamic Slices:** Small dynamic islands (like "1 in stock" or personalized greetings) stream in via edge workers/server-sent events within 200ms.
  4. **Offline Resilience:** If the network drops while browsing, cached catalog pages remain navigable via a minimal service worker.

  ### AI Agent Integration Points
  - **Ops Agent:** Monitors inventory levels. When inventory crosses a threshold (e.g., drops to 0), it triggers a targeted cache invalidation for just that product's tags at the edge.
  - **Marketing Agent:** Uses edge-computed geolocation to instantly swap banner text (e.g., "Free shipping in Brooklyn" vs. "We ship nationwide").

  ### Key Design Decisions
  - **Tag-Based Invalidation over TTLs:** Do not rely on time-based expiration. Instead, every page component is tagged (e.g., `store-123`, `product-456`). When the DB updates, the Cache Invalidator Agent purges specific tags globally.
  - **Edge-Assembled Pages:** The main layout is fully static at the edge. Highly dynamic elements (inventory, user session) are fetched post-load or injected by edge workers, preventing slow DB queries from blocking the First Contentful Paint.
  - **No Manual Settings:** The merchant never sees a "Clear Cache" button. The AI agents manage cache coherence autonomously.

  ## Implementation Prompt
  Implement the Edge-Cached Dynamic Storefront architecture.
  - **User-Facing Outcome:** Customers clicking a merchant's link experience near-instant page loads (sub-50ms First Contentful Paint) globally, while still seeing accurate, real-time inventory and pricing.
  - **CUJ (Critical User Journey):**
    1. Customer requests storefront URL.
    2. Edge cache serves the full HTML instantly.
    3. Edge worker seamlessly injects real-time inventory state.
    4. Merchant updates a product's price in the app.
    5. The Ops agent automatically invalidates the specific edge cache tags for that product.
    6. Next customer request fetches the fresh price and re-caches it.
  - **Acceptance Criteria:**
    - Storefront FCP is under 50ms globally.
    - Changes to inventory/pricing reflect on the live site within 500ms of the merchant saving.
    - Cache invalidation uses targeted tagging, not full-site purges.
    - Zero cache-management UI exposed to the merchant.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
