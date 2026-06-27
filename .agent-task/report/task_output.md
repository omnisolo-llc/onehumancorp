issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical small business owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, lost revenue, and poor SEO visibility due to dynamic rendering limitations. They lack the technical expertise to configure CDNs or Server-Side Rendering (SSR).

  ## Research Report
  - **Competitor Systems**: Shopify offers strong edge network capabilities (via Cloudflare), but SEO often requires third-party apps. Vercel/Next.js is great for developers but inaccessible to non-technical users. Wix/Squarespace provide easier SEO tools but require manual configuration and lack autonomous scalability.
  - **The OHC Differentiator**: OHC's approach must be **invisible and autonomous**. It requires universal edge caching (Cloudflare/CDN), Agentic Cache Invalidation when inventory changes, and Agentic SEO Pre-rendering for optimal search engine discoverability.

  ## Design Doc
  - **Architecture**: All storefront reads hit a global edge cache automatically.
  - **AI Coordination**: The Operations Agent actively monitors inventory levels. Upon changes, it purges the specific edge cache key to prevent overselling. The Marketing Agent autonomously triggers SEO pre-rendering to generate static HTML with optimal meta tags and pushes it to the edge.
  - **Mobile-First**: Ensures enterprise-grade performance and lightning-fast loading speeds on mobile devices (375px viewport) during traffic spikes.

  ## Implementation Prompt
  - **Outcome**: A dynamic storefront that seamlessly scales during traffic spikes through universal edge caching, with inventory levels remaining accurate via agent-driven cache invalidation, and SEO optimized autonomously through agentic pre-rendering.
  - **CUJ**: Maya experiences a viral spike from Instagram. Her storefront loads instantly via the edge cache. When an item sells out, the Operations Agent instantly invalidates the cache, preventing overselling. The Marketing Agent updates the pre-rendered HTML for search engines seamlessly.
  - **Acceptance Criteria**: Implement the caching strategy and agent integrations without exposing configuration complexity to the user.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
