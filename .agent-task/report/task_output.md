issue_title: "[Architecture] Edge-Caching Dynamic Storefront Engine"
issue_description: |
  ## Problem Statement
  Small business owners like Maya the Baker or Fatima the Food Cart Operator need instant loading, edge-cached storefronts for their businesses, as their customers often load pages on mobile devices over cellular networks. Currently, OneHumanCorp's platform lacks a robust, globally distributed edge-caching layer for dynamic, multi-tenant storefronts. Without edge-caching, dynamic components (like inventory toggles or custom product catalogs) must hit the central server, increasing latency and reducing mobile web performance, which directly hurts conversion rates.

  ## Research Report
  ### Competitive Analysis
  - **Shopify**: Utilizes Fastly for high-performance edge caching, enabling sub-second response times for static assets and CDN-cached dynamic endpoints.
  - **Wix/Squarespace**: Utilize heavy CDN networks but often suffer from slower time-to-interactive (TTI) due to monolithic bundle sizes and synchronous fetching.
  - **OHC Opportunity**: OHC's architecture currently assumes a central API processing layer. Implementing a targeted Edge-Caching Engine for storefronts that intelligently leverages the central API only for cache misses or transactional writes (e.g., placing an order) can position OHC's performance beyond Shopify's, especially on low-end devices.

  ### Proposed Solution: Edge-Caching Engine
  Design an Edge-Caching Dynamic Storefront Engine that:
  1. Distributes localized read-only store views (catalogs, menus) to CDN edges.
  2. Implements a smart invalidation strategy where the AI Operations Agent automatically purges cached assets globally upon inventory changes or menu updates.
  3. Uses stale-while-revalidate patterns to ensure users never experience cold-start delays.

  ## Design Doc
  ### Data Model & Invariants
  - **StorefrontCacheConfig**: Configuration specifying cache TTL and invalidation rules per tenant.
  - **EdgeAsset**: Tracking entity for generated, cached artifacts (HTML fragments, WebP images, JSON data payloads).
  - **Tenant Isolation**: Edge routing must ensure requests are rigidly segmented by `tenant_id` to prevent cross-contamination.

  ### AI Agent Integration
  - **Operations Agent ("The Manager")**: Monitors inventory adjustments (e.g., Fatima marks an item as "sold out") and orchestrates a webhook or event out to the edge network to immediately invalidate the `EdgeAsset` for that specific menu item.
  - **Marketing Agent ("The Promoter")**: When publishing a new site version, pushes the new static/dynamic asset definitions and warms the edge cache.

  ### Mobile-First & Offline UX
  - Storefront rendering must gracefully fall back to locally cached data via PWA service workers if the edge CDN is unreachable.
  - Initial load targets: < 1.0s First Contentful Paint (FCP) and < 1.5s Time to Interactive (TTI) on a 3G network on a mid-tier Android phone.

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  sequenceDiagram
      actor Cust as Customer (Mobile Web)
      participant CDN as Edge CDN
      participant API as OHC Central API (Go/Rust)
      participant AI as Operations Agent
      participant DB as PostgreSQL (Multi-tenant)

      Cust->>CDN: GET /store/maya-bakes/catalog
      alt Cache Hit
          CDN-->>Cust: Return Cached Catalog JSON
      else Cache Miss / Stale
          CDN->>API: Fetch Latest Catalog
          API->>DB: Query Catalog (tenant=maya-bakes)
          DB-->>API: Data
          API-->>CDN: Return Data (cache-control: s-maxage)
          CDN-->>Cust: Return Data
      end

      Note over AI, DB: Maya updates inventory
      AI->>DB: Update inventory (sold out)
      AI->>CDN: Invalidate Cache: /store/maya-bakes/catalog
  ```

  ## Implementation Prompt
  To Implementer Agent:
  Implement the foundation for the Edge-Caching Dynamic Storefront Engine. Define the core configuration models (`StorefrontCacheConfig` and caching rules) within the existing multi-tenant architecture. Create the service layer endpoints that allow the AI Operations Agent to emit cache invalidation events whenever critical business data (e.g., inventory or menu status) changes. Ensure these events are securely isolated by tenant. Develop unit tests and at least one E2E Playwright test that verifies a change in the backend properly triggers the invalidation mechanism and updates the rendered storefront view.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
