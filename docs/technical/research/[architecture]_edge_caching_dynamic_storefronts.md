# [Architecture] Edge-Caching Dynamic Storefronts & Catalog

## Problem Statement
Small business owners like Maya (Home Baker) and Priya (Boutique Owner) rely heavily on social media (Instagram, TikTok) to drive traffic. When a viral post sends a sudden spike of hundreds or thousands of visitors to their OneHumanCorp (OHC) storefronts, the site must load instantly. However, because their catalogs are dynamic (inventory levels change, Maya accepts custom cake quotes, Priya updates variants), static site generation (SSG) alone is insufficient, and traditional server-side rendering (SSR) risks slow load times or downtime under heavy load. A slow load time means lost sales from impatient mobile users. They need an architecture that delivers sub-second load times globally while always displaying accurate, up-to-date business data, all without them needing to know what a "CDN" or "Cache" is.

## Research Report
**Competitor Systems Audit:**
- **Shopify:** Utilizes a highly optimized edge caching strategy (Fastly) combined with Liquid templating and Storefront API. However, caching dynamic personalized elements (like cart state or localized pricing) requires complex workarounds or relies on client-side fetching which can increase Time to Interactive (TTI).
- **Vercel / Next.js:** Offers excellent ISR (Incremental Static Regeneration) and edge middleware. It is powerful but developer-focused. OHC needs to abstract this capability so that any change in the core ledger automatically triggers the correct cache invalidations.
- **Wix:** Employs a robust CDN and caching layer, but heavily relies on client-side hydration, which can be sluggish on lower-end Android devices (like Fatima's).

**Gaps Identified:**
OHC lacks a unified, globally distributed edge-caching architecture that natively understands our Zero-Trust multi-tenant data model and our specific AI agent interventions. We need a system where a storefront is served instantly from the edge (like a static file), but intelligently hydrates localized data (pricing, currency, inventory) instantly upon load, seamlessly managed by our AI swarm.

## Design Doc

### Architecture Diagram
```mermaid
graph TD;
    subgraph Global Edge Network (CDN/Edge Workers)
        Client[Mobile Browser / Web Client] --> EdgeNode[Edge Worker];
        EdgeNode --> Cache[(Edge Cache)];
        EdgeNode -- "Cache Miss / Dynamic Action" --> API[OHC API Gateway];
    end

    API --> CoreLedger[(Cloud Postgres Ledger)];
    API --> KVStore[(Global KV / Redis)];

    subgraph Agent Departments
        API --> MktAgent[Marketing: SEO & Analytics];
        API --> OpsAgent[Ops: Inventory Sync];
        OpsAgent -- "Invalidates" --> KVStore;
        OpsAgent -- "Purges" --> Cache;
    end

    subgraph Mobile Device (Store Owner)
        App[OHC App 375px] --> API;
    end
```

### Mobile UX Flow (375px First)
1. **The Shopper Experience:** A customer taps the link in Maya's Instagram bio. The edge node closest to them serves the pre-rendered HTML of the storefront in < 50ms.
2. **Dynamic Hydration:** Within milliseconds, the edge worker injects the current localized currency and live inventory status (e.g., "Only 2 left!") fetched from a low-latency global KV store, before the first paint finishes.
3. **The Owner Experience:** Priya updates a dress price from $45 to $50 in her OHC mobile app.
4. **Instant Invalidation:** She taps "Save". The Operations Agent intercepts the update, writes to the core ledger, and simultaneously purges the specific edge cache tags for that product globally. The next visitor sees $50 instantly.
5. **No Technical Jargon:** Priya never sees "Cache Cleared." She just sees a green checkmark: "Price updated live."

### AI Agent Integration Points
- **Operations Agent:** Monitors the core ledger for inventory and price changes. Automatically issues targeted cache invalidation requests (via cache tags) to the edge network without full site rebuilds.
- **Marketing Agent:** Pre-computes SEO metadata and Open Graph images for the edge cache, ensuring that social media link previews are always fast and up-to-date.
- **Finance Agent:** Supplies localized pricing matrices to the edge workers so they can dynamically swap currencies based on the shopper's Geo-IP.

### Key Design Decisions & Security
- **Edge-Side Includes (ESI) / Middleware:** Use edge compute to assemble dynamic personalized data (cart, localized price) into the cached static shell before delivering to the client. This keeps TTI extremely low on low-end devices.
- **Tag-Based Invalidation:** Every cached resource is tagged with Tenant ID and Entity IDs (e.g., `tenant:123`, `product:456`). When a product changes, only that specific product's cache is purged globally, avoiding expensive full-site rebuilds.
- **Zero-Trust Multi-Tenancy:** The edge workers use strictly scoped access tokens (SPIFFE SVIDs) when communicating back to the API Gateway to fetch dynamic data. The cache itself partitions data securely by Tenant ID.

## Implementation Prompt
Implement the Edge-Caching Dynamic Storefront architecture.
- **User-Facing Outcome:** Storefronts load globally in under 1 second, even on low-end 3G networks. When a business owner updates a product or inventory drops to zero, the live site updates globally within seconds without manual intervention.
- **CUJ (Critical User Journey):**
  1. A massive traffic spike hits a storefront. The Edge Network serves 99% of requests from cache.
  2. The store owner updates a product detail on their mobile app.
  3. The backend updates the database and triggers a tag-based cache invalidation.
  4. The next visitor immediately sees the updated product detail.
- **Acceptance Criteria:**
  - Architecture must support edge-rendering or highly effective edge caching (e.g., stale-while-revalidate).
  - Implement tag-based cache invalidation linked to database entity updates.
  - Ensure zero data leakage between tenants at the edge caching layer.
  - The AI Operations Agent must be responsible for managing complex cache invalidation rules (e.g., cascading invalidations for category pages when a product is added).
  - Abstract all caching terminology from the merchant UI.

## Priority
P0

## Estimated Scope
Large
