issue_title: "Implement Edge-Cached Dynamic Storefront Cache Control"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## 1. Problem Statement
  Small business (SMB) owners need high performance and discoverability for their storefronts. When a post goes viral, the resulting traffic spike can overwhelm centralized databases, leading to high latency and lost revenue. In addition, search engines struggle to index dynamic content that is slow to render.

  ## 2. Research Report
  - **Context**: The `docs/business/market_research/[research]_universal_edge_cached_dynamic_storefront_seo.md` document highlights the critical need for a Universal Edge-Cached Dynamic Storefront.
  - **OHC Opportunity**: Implement a robust caching mechanism at the service level, specifically for storefronts, to offload reads to an edge cache (e.g. Cloudflare) or a local fast cache (Redis), and automatically invalidate it when product data changes.

  ## 3. Design Doc
  ### Data Model & Architecture
  - Introduce an `EdgeCache` service or extend the current `CacheInvalidator` pattern.
  - When an operation alters a product, inventory, or storefront layout, an event or direct invalidation call should clear the related cache key `ohc:cache:storefront:{tenant_id}`.
  - Storefront read endpoints must first check the cache.

  ### Mobile UX Flow (375px)
  - Not directly a UI change, but ensures the mobile storefront loads in under 1 second even under heavy load.

  ### AI Integration
  - The "Marketing Agent" can proactively trigger SEO pre-rendering of the storefront upon major updates and populate the cache.

  ## 4. Implementation Prompt
  **Feature Name**: Edge-Cached Storefront Engine
  **Target Persona**: Maya the Baker
  **Outcome**: Maya's storefront is instantly available and highly ranked on search engines, gracefully handling viral traffic spikes.

  **Next Actions**:
  1. Add a `StorefrontCache` struct in the backend that uses Redis to cache HTML or JSON representations of the storefront.
  2. Implement cache invalidation hooks in the product/inventory mutation services.
  3. Ensure the main storefront API endpoint serves from the cache if available.

  **Priority**: P1
  **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
