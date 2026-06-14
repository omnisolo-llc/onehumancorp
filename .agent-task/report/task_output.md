issue_title: "[Architecture] Edge Caching & Dynamic Storefront Rendering Engine"
issue_description: |
  # Issue Brief: Edge Caching & Dynamic Storefront Rendering Engine

  ## Problem Statement
  For our user personas like Priya (Boutique Operator) and Maya (Home Baker), the loading speed of their storefronts is critical. High latency leads to cart abandonment and lower search engine rankings. However, building high-performance, edge-cached dynamic storefronts is complex and outside the skill set of non-technical owners. Current competitors like Shopify offer some level of edge caching, but often require complex configurations or paid apps for advanced optimization.

  ## Research Report
  - **Competitor Analysis:**
    - Shopify: Offers a global CDN and edge caching, but custom dynamic logic at the edge (like personalized pricing or localized inventory) can be complex or require expensive apps.
    - Wix/Squarespace: Generally slower due to heavier initial payloads and less aggressive edge caching strategies for dynamic content.
    - Vercel/Next.js: Industry standard for edge rendering and caching, but requires significant technical expertise to set up and manage.
  - **Market Gap:** There is a need for an "invisible" edge caching and dynamic rendering engine that automatically optimizes storefronts for speed and personalized content delivery without any manual configuration from the owner.
  - **Proposed Solution:** Implement an edge caching and dynamic rendering engine integrated into the OHC platform. This engine will use CDN edge nodes to cache static assets and use edge compute (like Cloudflare Workers or Vercel Edge Functions) to dynamically render personalized content (e.g., localized pricing, inventory availability) closer to the user, significantly reducing latency.

  ## Design Doc
  - **Architecture:**
    - Integration with a global CDN (e.g., Cloudflare, Fastly).
    - Edge compute layer for dynamic rendering and personalized content delivery.
    - Intelligent caching strategy: cache static content at the edge, dynamically fetch personalized content via edge compute, and invalidate cache intelligently upon inventory or pricing updates.
  - **AI Integration:**
    - AI agents can proactively monitor storefront performance and automatically adjust caching rules or suggest optimizations to the owner (e.g., "Your images are too large, I've compressed them for you").
  - **Mobile UX:** The engine operates invisibly in the background. The mobile UX impact is simply a dramatically faster, more responsive storefront for the end-user, leading to higher conversion rates for the owner.

  ## Implementation Prompt
  Implement the Edge Caching & Dynamic Storefront Rendering Engine.
  - **Outcome:** Storefronts should load significantly faster (target < 1s TTI) globally. Personalized content (localized pricing, inventory) should be rendered at the edge.
  - **CUJ:** Maya updates her cake pricing. The edge cache is automatically and intelligently invalidated, and the new pricing is reflected instantly across all global edge nodes without any manual intervention.
  - **Acceptance Criteria:**
    - Implement integration with a CDN/Edge compute provider.
    - Develop intelligent caching and invalidation logic based on business events (inventory updates, pricing changes).
    - Ensure dynamic content (localized pricing, inventory) is rendered at the edge.
    - Add automated E2E tests verifying edge caching behavior and latency improvements.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
