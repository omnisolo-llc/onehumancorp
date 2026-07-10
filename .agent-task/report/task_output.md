issue_title: "Implement Universal Edge-Cached Dynamic Storefront SEO & Agentic Pre-rendering"
issue_description: |
  # Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical small business owners (e.g., Maya the Baker) rely on social media virality, causing massive traffic spikes. Currently, these spikes hit the centralized PostgreSQL database on every page load, causing latency, timeouts, and poor SEO ranking. SMBs lack the skills to configure edge caching, CDN routing, and SSG (Static Site Generation) themselves. They need a system that instantly serves their custom domain (e.g., `mayascakes.com`) with enterprise-grade speed and discoverability, entirely invisibly.

  ## Research Report
  - **Shopify:** Uses edge networks (Cloudflare) to cache read-heavy storefronts globally, but dynamic localized pricing is difficult, and setup isn't fully invisible.
  - **Wix/Squarespace:** Provide basic SEO and have moved to CDN caching, but still struggle with heavy JS payloads affecting TTFB (Time to First Byte).
  - **Vercel/Next.js:** Gold standard for Edge/ISR but requires heavy developer resources.
  - **OHC Gap:** OHC needs an invisible, agentic solution: The "Universal Edge-Cached Dynamic Storefront." Caching must be globally distributed, and invalidation must be handled by AI Agents autonomously.
  - See `docs/technical/research/[architecture]_universal_edge_cached_dynamic_storefronts.md` for full details.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Browser] -->|mayascakes.com| B(CDN / Edge Proxy - Nginx/OpenResty)
      B -->|Domain Lookup| C[Edge K/V Store - Redis]
      C -->|Tenant ID: 123| B
      B -->|Cache Hit| D[Return Pre-rendered HTML]
      B -->|Cache Miss| E[OHC Core Backend / SSR Service]
      E -->|Render HTML & JSON-LD| F[(PostgreSQL: Storefront Content)]
      F --> E
      E -->|Store in Cache| B
      D --> A
      A -->|Hydrate PWA| G[OHC Flutter App]
      H[Operations/Marketing Agent] -->|Inventory/Content Update| I[Webhook: Invalidate Cache Key]
      I --> B
  ```

  ### UI Wireframes & Screen Flow (375px)
  - **Screen 1 (Customer View - Storefront):** 375px optimized storefront. Hero image, dynamic product grid. Rendered instantly from CDN.
  - **Screen 2 (Owner View - Agent Feed):** 375px feed showing "Marketing Agent: Pre-rendered Storefront updated for SEO." with "View Analytics" button.
  - **Screen 3 (Owner View - Settings - Advanced):** "Advanced Edge Configuration" (hidden behind an Advanced toggle). Shows Custom Domain status and Cache Purge options.

  ### Mobile UX Flow
  1. **Customer taps link on Instagram:** The edge node instantly returns the skeletal HTML and critical CSS (sub-500ms TTFB) for `mayascakes.com`.
  2. **Instant Paint:** The browser paints the storefront immediately on the mobile screen.
  3. **Hydration:** The Flutter PWA engine loads asynchronously in the background, hydrating the page for smooth, app-like interactions (e.g., Tap to Pay, Add to Cart).
  4. **Owner Updates:** The owner updates a product price. The Operations Agent autonomously invalidates the cache; the owner sees no loading screens or configuration pages.

  ### AI Agent Integration Points
  - **Marketing Agent / Operations Agent:** Automatically invalidates the edge cache via a Webhook/API event whenever the owner updates a product, changes a price, or publishes a new blog post.
  - **SEO Pre-rendering:** The Marketing Agent periodically suggests SEO improvements and, upon approval, updates the pre-rendered HTML meta tags at the edge.

  ### Key Design Decisions
  - **Edge Routing (Domain to Tenant Mapping):** Use an Edge Proxy (e.g., Nginx with Lua/OpenResty) with a distributed K/V store (Redis) to map custom domains (Host headers) to `tenant_id`. This keeps DB load near zero for routing.
  - **Stale-While-Revalidate:** Configure `Cache-Control: stale-while-revalidate` at the edge to serve instant responses while fetching fresh data asynchronously, ensuring customers never wait.
  - **Agentic Invalidation:** Agents autonomously purge the Redis cache keys. The owner never clicks a "Clear Cache" button.

  ## Implementation Prompt
  - Create the Edge Proxy configuration (e.g., `nginx.conf` with Lua/OpenResty) to handle domain-to-tenant routing via Redis caching.
  - Implement the `stale-while-revalidate` caching strategy in the proxy.
  - Build the backend endpoint (`/api/v1/storefront/{tenant_id}`) that generates the pre-rendered HTML/SEO metadata.
  - Create an event listener in the Agent framework to purge the Redis cache key when inventory or content changes.
  - Do NOT prescribe specific database schemas or internal backend Rust struct names; focus on the proxy logic, cache headers, and agent event connection.
  - Implement an integration test verifying custom domain mapping and cache HIT/MISS behavior.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
