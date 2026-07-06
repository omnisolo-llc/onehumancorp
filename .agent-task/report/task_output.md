issue_title: "Architecture Design: Edge-Cached Dynamic Multi-Tenant Storefronts"
issue_description: |
  **Title**: Architecture Design: Edge-Cached Dynamic Multi-Tenant Storefronts

  **Problem Statement**:
  Small business storefronts (e.g., Maya's cake shop, Priya's boutique) must load instantly on mobile networks, rank highly on Google, and correctly route traffic to the appropriate tenant based on custom domains (e.g., `mayascakes.com` -> `tenant_id: 123`). Traditional dynamic SSR (Server-Side Rendering) is too slow on 3G networks, and pure SPA (Single Page Applications) suffer in SEO. OHC needs a globally distributed, edge-cached serving architecture that supports millions of distinct tenant domains while remaining fast and cost-effective.

  **Research Report**:
  - **Shopify**: Uses massive Ruby on Rails monoliths fronted by Cloudflare. They aggressively cache at the edge but struggle with dynamic localized pricing.
  - **Wix/Squarespace**: Historically suffered from slow load times due to heavy JS payloads. Have moved towards SSR + CDN, but TTFB (Time to First Byte) can still lag.
  - **OHC Opportunity**: Leverage a modern approach: serve static assets via CDN, use edge compute (like Cloudflare Workers or Fly.io) for instant tenant domain resolution and initial HTML rendering, and hydrate with the Flutter/PWA application for dynamic interactions.

  **Design Doc**:
  - **Architecture Diagram**:
    ```mermaid
    graph TD
        A[Customer Browser] -->|mayascakes.com| B(CDN / Edge Node)
        B -->|Domain Lookup| C[Edge K/V Store - Redis]
        C -->|Tenant ID| B
        B -->|Cache Hit| D[Return Pre-rendered HTML]
        B -->|Cache Miss| E[OHC Core Backend / SSR Service]
        E -->|Render| F[(PostgreSQL: Storefront Content)]
        F --> E
        E -->|Store in Cache| B
        D --> A
        A -->|Hydrate PWA| G[OHC Flutter App]
    ```

  - **Mobile UX Flow (375px)**:
    1. Customer taps a link on Instagram.
    2. The edge node instantly returns the skeletal HTML and critical CSS (sub-500ms).
    3. The browser paints the storefront immediately.
    4. The Flutter PWA engine loads asynchronously in the background, hydrating the page for smooth, app-like interactions (e.g., Tap to Pay, Add to Cart).

  - **AI Agent Integration Points**:
    - **Marketing Agent**: Automatically invalidates the edge cache when the owner updates a product, changes a price, or publishes a new blog post.

  - **Key Design Decisions**:
    - **Edge Routing**: DNS and initial HTTP requests hit an edge network. The edge node maps the custom domain to the internal `tenant_id` using a distributed Key/Value store, preventing the core DB from handling routing lookups.
    - **Stale-While-Revalidate**: Use Cache-Control headers to serve stale content while asynchronously fetching the updated version, ensuring zero wait time for the user.
    - **Asset Compression**: All images uploaded by the owner are automatically compressed to WebP and served from a CDN.

  **Implementation Prompt**:
  Design the Edge-Cached Dynamic Multi-Tenant Storefront serving architecture. Define the reverse proxy configuration (e.g., Nginx, Caddy, or Cloudflare Worker script) required to inspect incoming HTTP Host headers, map them to a specific OHC `tenant_id` via a high-speed cache, and route the request to the correct SSR rendering service. Detail the caching strategy (Cache-Control headers, invalidation events) to ensure sub-second TTFB for end customers while keeping the storefront data fresh. Create a testing plan that verifies domain resolution, cache hits/misses, and successful cache invalidation upon product updates.

  **Priority**: P0
  **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
