issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases. This leads to high latency, lost revenue, and poor SEO visibility because search engine crawlers struggle to index slow, client-side rendered dynamic content. Existing platforms like Vercel are too technical, while Wix/Squarespace lack instant autonomous scalability during unpredictable traffic spikes.

  ## Research Report
  - **Shopify:** Utilizes strong edge caching via Cloudflare but requires third-party apps for advanced SEO.
  - **Vercel/Next.js:** The industry standard for edge performance and SEO via ISR/SSG, but requires significant technical expertise inaccessible to non-technical SMB owners.
  - **Wix/Squarespace:** Simpler but often lack true autonomous instant edge-caching for large traffic spikes.

  The solution for OHC must be completely invisible and autonomous. A "Universal Edge-Cached Dynamic Storefront" where all storefront reads hit a global edge cache. When an Operations Agent updates inventory (e.g., item sells out), it instantly purges the specific edge cache key globally. Furthermore, when the Marketing Agent updates the website, it must autonomously trigger "Agentic SEO Pre-rendering" to generate highly optimized static HTML with meta tags and structured data, pushing it directly to the edge for crawlers to consume instantly.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer/Web Crawler] --> B[Edge Cache CDN e.g., Cloudflare]
      B -- Cache Miss --> C[OHC Frontend]
      C --> D[PostgreSQL/Redis via OHC Backend]
      E[Operations Agent] -->|Inventory Update| F[Cache Invalidation API]
      F -->|Purge Key| B
      G[Marketing Agent] -->|Content Update| H[Agentic SEO Pre-renderer]
      H -->|Push Static HTML| B
  ```

  ### Core Components
  1. **Edge Cache Layer:** Automatically caches dynamic frontend reads based on tenant and route.
  2. **Agentic Cache Invalidation:** The AI Operations Agent issues targeted cache invalidation requests using specific cache tags (e.g., `tenant:123:inventory:456`) when mutating state.
  3. **Agentic SEO Pre-rendering:** The Marketing Agent dynamically generates static HTML representations of product pages with correct `seo_schema_json` and meta tags, and seeds the edge cache to enhance crawler performance.

  ### Mobile UX Flow
  There is zero required UI configuration for this feature. It operates invisibly behind the scenes. However, the system might surface an Action Card in the Agent Feed: "Marketing Agent pre-rendered your new product page for Google! [View Preview]". The preview must load perfectly on a 375px mobile viewport.

  ### AI Agent Integration Points
  - **Operations Agent:** Needs access to a new caching service interface to fire targeted cache purges.
  - **Marketing Agent:** Needs access to an SEO rendering pipeline to compile data into static strings and seed the edge cache.

  ## Implementation Prompt
  Implement the "Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering" architecture.
  - **Phase 1:** Introduce a caching layer/interface that the Operations Agent can use to purge specific cache keys (e.g., when a product's `inventory_count` reaches 0). You do not need to implement the actual edge CDN, but create the adapter/service layer that would interface with it.
  - **Phase 2:** Extend the Marketing Agent to detect product updates and trigger a background pre-rendering job.
  - **Phase 3:** Write the pre-rendering logic to generate static HTML blobs with injected SEO metadata and structured JSON for crawlers, storing these in a designated edge-accessible store.
  - Ensure strict multi-tenant isolation.
  - Estimated Scope: Large
  - No new complex UI settings should be added; this is an invisible infrastructure upgrade.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
