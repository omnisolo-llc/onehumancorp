issue_title: "[Architecture] Implement Universal Edge-Cached Dynamic Storefront for SEO"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & SEO Architecture

  ## Executive Summary
  This report investigates the architecture needed for OneHumanCorp to serve fast, SEO-optimized, universally edge-cached dynamic storefronts. The objective is to design a high-performance serving layer that delivers instant page loads and strong SEO metrics while remaining seamlessly updated via the central control plane, directly serving our non-technical business owner personas.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Shopify and Vercel Next.js Commerce excel at edge delivery, leveraging CDNs (Cloudflare, Vercel Edge) to serve sub-100ms storefronts globally. They utilize Incremental Static Regeneration (ISR) or Edge middleware to blend static speed with dynamic inventory and pricing. OHC currently lacks a dedicated, high-performance edge-caching tier, meaning storefront requests hit the core application servers directly, leading to higher latency and suboptimal SEO scoring due to slower Time to First Byte (TTFB).

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:** Maya (Home Baker) and Priya (Boutique Operator) need their public storefronts to load instantly for customers sharing links on Instagram or browsing on mobile connections. They also need strong SEO to be discoverable on Google.
  - **The Gap:** The current platform serving model does not aggressively utilize edge caching or static generation for public storefronts. This limits SEO performance and increases infrastructure load during high-traffic events (e.g., product drops).

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User as Customer (Browser/Mobile)
      participant Edge as Edge CDN (Cloudflare/Varnish)
      participant Core as ohc-core (API/Service)
      participant DB as Central Ledger (PostgreSQL)

      User->>Edge: Request Storefront URL
      alt Cache Hit
          Edge-->>User: Serve Pre-rendered HTML (Sub-100ms)
      else Cache Miss
          Edge->>Core: Fetch Storefront Data
          Core->>DB: Query Tenant Config & Products
          DB-->>Core: Data
          Core-->>Edge: Dynamic HTML + Cache Headers
          Edge-->>User: Serve Response & Cache
      end

      User->>Edge: Client-side Hydration (Cart/Inventory)
      Edge->>Core: API Request (Bypass Cache)
      Core-->>User: Dynamic State Updates
  ```

  ### Data Model & Sync Protocol
  - **Storefront Edge Cache:** Implement a caching layer (e.g., using Cloudflare Workers/CDN or a localized reverse proxy like Varnish/NGINX with aggressive caching for standalone) that serves pre-rendered HTML.
  - **Cache Invalidation:** The core `ohc-core` application must publish cache invalidation events (e.g., via Valkey/Redis PubSub) whenever a tenant's product, inventory, or layout changes. The edge tier subscribes to these events to purge specific tenant/route cache keys.
  - **Dynamic Hydration:** Pre-rendered HTML contains placeholders for highly dynamic data (e.g., cart count, specific inventory limits). The client-side (Flutter Web/PWA) hydrates these specifics via lightweight API calls immediately after the static shell loads.

  ### AI Agent Coordination
  - **Operations Agent:** Monitors cache hit ratios and SEO performance metrics. It can suggest layout optimizations or flag if a storefront is experiencing unusually high traffic, automatically scaling invalidation frequency.
  - **Marketing Agent:** Analyzes SEO metadata (meta tags, open graph images) and automatically updates them based on the owner's inventory descriptions, triggering a cache rebuild.

  ### Mobile-First Implementation
  - Edge delivery prioritizes mobile viewports, ensuring the critical rendering path for 375px screens is unblocked and sub-100ms.
  - Image assets must be served via the CDN layer, utilizing WebP compression and responsive `srcset` generation.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Issue Prompt for Implementer:**
  Implement the universal edge-caching layer for tenant storefronts.
  1. Introduce a reverse proxy caching mechanism (or document the CDN integration pattern) in the `deploy/` stack.
  2. Implement the Cache Invalidation service in `ohc-core` that listens to database mutations on `products`, `tenants`, and `inventory` and triggers targeted cache purges.
  3. Update the frontend delivery mechanism to serve a static HTML shell with SEO metadata pre-populated from the tenant's configuration, followed by dynamic hydration for cart/user state.
  4. Ensure end-to-end tests verify that updating a product name correctly invalidates the cache and reflects on the public storefront within 5 seconds.

  **Estimated Scope:** Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []