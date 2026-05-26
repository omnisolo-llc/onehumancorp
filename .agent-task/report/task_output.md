issue_title: "[architecture]_edge_caching_dynamic_storefronts"
issue_priority: "P0"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []
issue_description: |
  # Architecture Brief: Edge-Caching Dynamic Storefronts

  ## Problem Statement
  Small business owners relying on OneHumanCorp (OHC) need their online storefronts and service booking pages to load instantly, regardless of the customer's location or internet connection quality. Maya (baker) and Priya (boutique owner) lose potential sales when their photo-heavy catalogs take seconds to load on mobile connections. Furthermore, sudden viral traffic spikes (e.g., from a TikTok video link-in-bio for Leo the music tutor) can overwhelm central database servers, causing the platform to stutter. Current traditional monolithic web serving relies heavily on database reads per request, leading to latency and vulnerability to traffic spikes. OHC lacks a robust, globally distributed edge-caching architecture for its dynamic storefronts to guarantee sub-100ms load times and 100% availability during viral events.

  ## Research Report
  - **Latency vs Conversion:** Industry benchmarks (Shopify, Amazon) demonstrate a direct correlation between page load speed and conversion rates. A 100ms delay can cost up to 1% in sales.
  - **Competitor Solutions:**
      - **Shopify:** Utilizes a globally distributed CDN (Cloudflare/Fastly) combined with edge computing (Shopify Oxygen) to cache storefront pages, intercepting requests at the edge. They invalidate caches efficiently based on inventory updates or price changes.
      - **Wix/Squarespace:** Heavily cache static assets, but often struggle with dynamic content (inventory, specific availability) under heavy load without sophisticated edge invalidation.
  - **The Gap in OHC:** OHC needs an architecture that pre-renders or edge-caches entire storefront experiences (catalog, pricing, availability) close to the user, while intelligently invalidating these caches the moment an underlying `Tenant` data point (like inventory dropping to zero) changes.

  ## Design Doc

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Customer as Customer (Mobile/Web)
      participant Edge as CDN/Edge Node
      participant Cloud as OHC Cloud Platform
      participant AI as Operations Agent
      participant DB as OHC Database

      Customer->>Edge: Requests Maya's Storefront (mayascakes.com)
      alt Cache Hit
          Edge-->>Customer: Serve cached storefront (Sub-100ms)
      else Cache Miss
          Edge->>Cloud: Fetch Storefront Data
          Cloud->>DB: Query Tenant Data
          Cloud-->>Edge: Return Data & Cache Directives
          Edge-->>Customer: Serve storefront
      end

      Note over Cloud, AI: Inventory changes trigger invalidation

      AI->>Cloud: Update Inventory (Item Sold)
      Cloud->>DB: Commit Change
      Cloud->>Edge: Invalidate Cache Tag (tenant:maya:inventory)
  ```

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Customer View (Storefront):** The experience must be indistinguishable from a natively installed app. The catalog grid loads immediately (cached). Images use modern formats (WebP/AVIF) and lazy loading.
  - **Merchant View (OHC App):** When Priya updates a product price, the app shows a subtle, non-blocking toast: "Updating your online store...". The change is optimistic locally, while the background invalidation propagates globally.

  ### AI Agent Integration Points
  - **Operations Agent:** When processing a backend sale (e.g., via the Unified Inbox or Tap-to-Pay), this agent autonomously emits fine-grained cache invalidation events (using tags like `tenant_id:product_id`). It ensures the edge never serves stale "in-stock" data for a sold-out item.
  - **Promoter Agent:** When publishing a new template or making broad design changes, this agent manages the bulk invalidation of the tenant's edge cache.

  ### Key Data & Multi-Tenancy Invariants
  1. **Tenant-Scoped Cache Tags:** All cached assets and API responses MUST be tagged with the relevant `tenant_id`. Invalidation requests MUST include the `tenant_id` to prevent one merchant's update from clearing another's cache.
  2. **Stale-While-Revalidate:** The edge should employ stale-while-revalidate strategies to ensure customers always get a fast response, even if the cache is slightly out of date (except for critical inventory paths which require strict synchronization).

  ## Implementation Prompt
  **To Implementer Agent:**
  Design and implement the Edge-Caching architecture for OHC dynamic storefronts. Define the caching strategies (e.g., HTTP Cache-Control headers, Edge Side Includes, or CDN-specific tagging) for different types of storefront data (static assets vs. inventory availability). Implement the invalidation pipeline within the OHC backend so that whenever a core entity (Product, Booking Slot) is updated, an event is fired to purge the relevant edge cache using tenant-specific tags. Ensure the Operations Agent can trigger these invalidations reliably. Focus on the interfaces, event structures, and multi-tenant isolation rules; do not prescribe a specific CDN provider (like Cloudflare or Fastly) in your code, but build it to be provider-agnostic.

  ## Priority
  P1

  ## Estimated Scope
  Large
