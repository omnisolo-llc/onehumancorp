issue_title: "[Architecture] Edge-Cached Dynamic Storefronts & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## 1. Executive Summary
  This research investigates the critical need for a Universal Edge-Cached Dynamic Storefront combined with Agentic SEO Pre-rendering for the OneHumanCorp (OHC) platform. It identifies a major gap in current offerings where small businesses (SMBs) suffer from slow load times during traffic spikes and poor search engine visibility due to dynamic rendering limitations. By leveraging edge caching and AI-driven SEO pre-rendering, OHC can provide enterprise-grade performance and discoverability to non-technical users invisibly.

  ## 2. Market Context & Pain Points
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to:
  - **High Latency & Timeouts:** Frustrating potential customers and increasing bounce rates.
  - **Lost Revenue:** Every second of delay directly impacts conversion rates.
  - **SEO Penalties:** Search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability.
  - **Complexity Barrier:** SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## 3. Competitive Landscape
  - **Shopify:** Offers strong edge network capabilities (via Cloudflare) for fast global delivery of storefronts. SEO is robust but often requires third-party apps for advanced optimization.
  - **Vercel/Next.js Ecosystem:** The gold standard for developers (ISR, Edge computing), but inaccessible to non-technical users without significant development investment.
  - **Wix/Squarespace:** Provide easier SEO tools, but they still require manual configuration and lack the autonomous, instant scalability of true edge architectures during massive, unpredictable spikes.

  ## 4. The OHC Differentiator: Invisible & Autonomous
  OHC's approach must go beyond providing caching infrastructure. It must be **invisible and autonomous**.
  - **Universal Edge Caching:** All storefront reads must hit a global edge cache (e.g., Cloudflare) automatically. No configuration needed by the user.
  - **Agentic Cache Invalidation:** When the Operations Agent updates inventory (e.g., an item sells out), it instantly purges the specific edge cache key globally, ensuring accurate stock levels and preventing overselling.
  - **Agentic SEO Pre-rendering:** When the Marketing Agent updates the website, it autonomously triggers a pre-rendering process. This generates highly optimized, static HTML injected with relevant meta tags and structured data, pushing it directly to the edge. This ensures web crawlers instantly see the most relevant, fast-loading version of the site, boosting organic ranking without the user lifting a finger.

  ## 5. Strategic Value to OHC
  Implementing this architecture positions OHC not just as a store builder, but as an enterprise-grade performance engine.
  - **Guaranteed Uptime & Speed:** Crucial for user trust during their most important moments (viral spikes).
  - **Automated Growth:** Agent-driven SEO pre-rendering passively increases organic traffic, directly impacting the SMB's bottom line.
  - **Cost Efficiency:** Offloading reads to the edge significantly reduces the load and scaling costs of the central PostgreSQL database.

  ## 6. System Design
  ### Architecture Diagram (Mermaid)
  ```mermaid
  graph TD
      Client[Customer Browser] --> CDN[Edge Cache Network]
      CDN -- Cache Hit --> Client
      CDN -- Cache Miss --> AppServer[OHC App Server]
      AppServer --> DB[(PostgreSQL)]

      Agent[Marketing Agent] --> PreRender[SEO Pre-render Engine]
      PreRender --> CDN : Push Static HTML

      OpsAgent[Operations Agent] --> Invalidator[Cache Invalidation Queue]
      Invalidator --> CDN : Purge Specific Keys
  ```

  ### Core Data Models
  - `StorefrontCacheConfig`: Tenant-level configuration for edge caching rules.
  - `SEOPreRenderJob`: Queue item for the Marketing Agent to generate static HTML.
  - `CacheInvalidationEvent`: Pub/Sub event for the Operations Agent to trigger edge purges.

  ### Mobile UX Flow (375px)
  - The storefront loads instantly (under 1s) from the edge cache on mobile devices.
  - Fast, responsive scrolling and image loading without layout shift.
  - PWA support enables offline browsing of previously visited pages.

  ## 7. Implementation Prompt
  Implement the foundation for Universal Edge-Cached Dynamic Storefronts and Agentic SEO Pre-rendering.
  - **Requirement 1:** Create the necessary data schemas for `StorefrontCacheConfig` with robust multi-tenant isolation.
  - **Requirement 2:** Implement an `SEOPreRenderQueue` backed by PostgreSQL (using SKIP LOCKED) to handle background pre-rendering jobs triggered by the Marketing Agent.
  - **Requirement 3:** Develop a `CacheInvalidationService` that the Operations Agent can call to issue targeted edge purges (mock the actual CDN API call for now, but design the interface).
  - **Requirement 4:** Ensure 100% unit test coverage for all new services and queues.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
