# [Architecture] Multi-Tenant Edge-Caching Storefront Engine

## Title
Architect Multi-Tenant Edge-Caching Storefront Engine

## Problem Statement
For OHC to serve highly active merchants (like Priya's pop-up drops or Maya's viral cake sales), the online storefronts must withstand massive traffic spikes without exposing the underlying multi-tenant backend to DDOS-like strain. Competitors like Shopify leverage extensive CDN and edge-caching architectures. OHC currently relies heavily on real-time database queries or localized syncing, which poses a severe scalability risk for the public-facing buyer experience. Maya or Priya cannot succeed if a viral TikTok crashes their storefront.

## Research Report
Through researching competitor systems (Shopify's edge architecture, Wix's caching strategies) and auditing OHC's current capabilities, the highest impact missing architecture is a Multi-Tenant Edge-Caching Storefront Engine.
- **Shopify:** Leverages a globally distributed edge network (Fastly/Cloudflare) to cache storefront pages, achieving incredibly low latency and high availability even during flash sales.
- **Wix:** Utilizes SSR (Server-Side Rendering) with aggressive caching layers.
- **OHC Gap:** OHC currently lacks a formal, edge-first caching strategy tailored for its multi-tenant architecture. The system needs to ensure that public storefronts are served directly from the edge, minimizing trips to the core Postgres database, while still maintaining real-time accuracy for critical data like inventory and pricing.

## Design Doc

### High-Level Architecture
- **Trigger**: A buyer clicks a link from TikTok. The storefront must load in <200ms globally, regardless of the tenant's primary database region.
- **Data Model**: Introduce `StorefrontCacheConfig` linked to the `Tenant`. The engine acts as a reverse proxy, heavily caching the HTML/JSON payload of the storefront. Cache invalidation is driven by events (e.g., Inventory dropping to 0, pricing changes).
- **Mobile UX Flow**: Edge delivery ensures the 375px mobile UI loads instantly. The edge layer enforces strict tenant boundaries, stripping PII and ensuring isolation.

```mermaid
graph TD
    A[Buyer clicks link (e.g., from TikTok)] --> B[Edge CDN Node]
    B --> C{Is valid cached storefront available?}
    C -- Yes --> D[Serve Cached Storefront (< 50ms)]
    C -- No --> E[OHC API Gateway]
    E --> F[Fetch Storefront Data (Tenant DB)]
    F --> G[Render & Cache at Edge]
    G --> D

    H[Inventory/Price Change Event] --> I[Operations Agent]
    I --> J[Trigger Cache Invalidation for Tenant Storefront]
    J --> B
```

### AI Agent Integration
- **Operations Agent**: Monitors cache invalidation events. If a product goes out of stock, the agent triggers an immediate purge at the edge and updates the UI to show "Sold Out". It can also pre-warm caches for upcoming scheduled sales drops.

### Key Design Decisions
- **Event-Driven Invalidation**: The cache is not time-based (TTL) but event-based. When inventory changes, the specific storefront cache is purged and optionally pre-warmed.
- **Multi-Tenant Routing**: The edge layer must correctly route and isolate caches based on the incoming domain/tenant ID to prevent cross-contamination.
- **Zero-Trust**: The edge layer strips any unnecessary PII or sensitive headers before caching or forwarding requests.

## Implementation Prompt
Implement the Edge-Caching Multi-Tenant Storefront Engine. Define the caching strategy and event-driven invalidation pipeline. Ensure public storefront routes are served from the edge with sub-200ms latency. The system must automatically invalidate cached assets when inventory or pricing changes occur in the core ledger. Do not prescribe specific CDN providers (e.g., Cloudflare vs Fastly); focus on the application-level cache headers, invalidation events, and multi-tenant routing logic.

## Priority
P0

## Estimated Scope
Large
