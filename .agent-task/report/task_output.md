issue_title: "[architecture] Edge-Caching Multi-Tenant Storefront and High-Scale Capability Discovery"
issue_description: |
  # Research Report: High-Scale Storefront Infrastructure

  **Problem Statement:**
  For OHC to serve highly active merchants (like Priya's pop-up drops or Maya's viral cake sales), the online storefronts must withstand massive traffic spikes without exposing the underlying multi-tenant backend to DDOS-like strain. Competitors like Shopify leverage extensive CDN and edge-caching architectures. OHC currently relies heavily on real-time database queries or localized syncing, which poses a severe scalability risk for the public-facing buyer experience.

  **Discovery and Gaps:**
  Through researching competitor systems (Shopify's edge architecture, Wix's caching strategies) and auditing OHC's current capabilities, the highest impact missing architecture is a Multi-Tenant Edge-Caching Storefront Engine. Maya or Priya cannot succeed if a viral TikTok crashes their storefront.

  **System Design Deep Dive:**
  - **Business Journey:** A buyer clicks a link from TikTok. The storefront must load in <200ms globally, regardless of the tenant's primary database region.
  - **Data Model:** Introduce `StorefrontCacheConfig` linked to the `Tenant`. The engine acts as a reverse proxy, heavily caching the HTML/JSON payload of the storefront. Cache invalidation is driven by events (e.g., Inventory dropping to 0, pricing changes).
  - **AI Coordination:** The Operations Agent monitors cache invalidation events. If a product goes out of stock, the agent triggers an immediate purge at the edge and updates the UI to show "Sold Out".
  - **Mobile & Zero-Trust:** Edge delivery ensures the 375px mobile UI loads instantly. The edge layer enforces strict tenant boundaries, stripping PII and ensuring isolation.

  **Implementation Prompt for Swarm:**
  Implement the Edge-Caching Multi-Tenant Storefront Engine. Define the caching strategy and event-driven invalidation pipeline. Ensure public storefront routes are served from the edge with sub-200ms latency. The system must automatically invalidate cached assets when inventory or pricing changes occur in the core ledger. Do not prescribe specific CDN providers (e.g., Cloudflare vs Fastly); focus on the application-level cache headers, invalidation events, and multi-tenant routing logic.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
