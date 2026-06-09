issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture"
issue_description: |
  # Mission Queue Protocol: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, and lost revenue. Additionally, current dynamic rendering limits SEO performance as web crawlers struggle with slow, client-side rendered content. OHC must provide enterprise-grade performance and discoverability to non-technical users invisibly.

  ## Research Report
  - **Competitor Analysis:** Shopify utilizes Cloudflare for edge caching, while Vercel/Next.js uses ISR. Wix offers manual SEO tools. OHC must abstract this complexity.
  - **The OHC Differentiator:** Universal Edge Caching combined with Agent-Driven Cache Invalidation and Agentic SEO Pre-rendering. This ensures instant load times (<100ms) and automated, top-tier SEO for all OHC storefronts without user configuration.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Buyer Mobile App / Web / Crawler] -->|Requests Storefront| B(Edge CDN - e.g., Cloudflare/Fastly);
      B -- Cache Hit --> A;
      B -- Cache Miss --> C(OHC API Gateway);
      C --> D[Storefront Render Service];
      D --> E[(PostgreSQL Read Replica)];
      F[Operations / Marketing Agent] -->|Updates Inventory / Content| G[Core Services];
      G -->|Mutation Event| H[Cache Invalidation & Pre-render Bus];
      H -->|Purge Key / Pre-render Job| B;
      H -->|Trigger AI SEO Agent| I[SEO Pre-rendering Service];
      I -->|Push Pre-rendered HTML| B;
  ```

  ### Mobile UX Flow
  - 375px Viewport: Storefronts must load instantly (<100ms) on slow 3G connections. High-quality WebP images are served directly from the Edge.

  ### AI Agent Integration Points
  - **Marketing Agent:** Triggers a pre-rendering service to generate static, SEO-optimized HTML upon content updates.
  - **Operations Agent:** Publishes cache invalidation events to purge surrogate keys globally when inventory mutations occur.

  ## Implementation Prompt
  Implement the Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture.
  - **Objective:** Integrate a reverse-proxy CDN layer that caches GET requests for public storefront APIs and pre-renders SEO-optimized HTML.
  - **CUJ:** When a customer opens Maya's storefront link, the product list is served from the cache. When Maya updates a cake price, the system publishes a cache invalidation event and triggers the SEO Agent to pre-render the updated page, ensuring the next customer sees the new price instantly and search engines receive the updated content.
  - **Acceptance Criteria:**
    - E2E tests verifying cache hits for repeated reads.
    - E2E tests verifying cache misses and subsequent cache population following an inventory update.
    - Tests ensuring the SEO pre-rendering service correctly generates static HTML upon content updates.
    - Strict tenant isolation maintained within the cache layer.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
