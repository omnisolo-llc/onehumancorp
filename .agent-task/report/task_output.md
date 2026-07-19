issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  ## Mission Queue Protocol

  ### 1. Problem Statement
  Currently, OneHumanCorp (OHC) routes all customer-facing storefront traffic directly to the Go backend and PostgreSQL database. For non-technical business owners like Maya (the baker with viral limited-time holiday cake drops) and Priya (boutique operator with viral TikTok items), sudden traffic spikes can cause unacceptable latency, high bounce rates, and potential downtime. Furthermore, search engines struggle to index slow, dynamically rendered content, reducing organic discoverability. SMB owners should never have to worry about "server capacity" or manual SEO configurations.

  ### 2. Research Report
  - **Competitor Analysis:** Platforms like Shopify and Vercel Commerce rely heavily on Edge caching (Cloudflare Workers, Fastly, or Vercel Edge Network) and global KV stores to serve product catalogs with single-digit millisecond latency. Wix and Squarespace provide easier SEO tools but require manual configuration.
  - **Current OHC Bottleneck:** The current `src/server` dynamically fetches tenant configurations and product catalogs directly from Postgres on every page load. The lack of Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO hurts organic ranking.
  - **Proposed Solution:** Implement an Edge-Caching Dynamic Storefront layer using Valkey (Redis) caching coupled with a CDN cache-control strategy. Additionally, implement Agentic SEO Pre-rendering where AI Agents seamlessly generate and push optimized static HTML to the edge upon inventory or configuration updates.

  ### 3. Design Doc
  **Architecture Diagram:**
  ```mermaid
  graph TD;
      CustomerBrowser[Customer Mobile Browser] --> CDN[Edge CDN / Cloudflare];
      SearchEngineCrawler[Googlebot] --> CDN;
      CDN -- Cache Miss --> OHC_API[OHC Go API];
      OHC_API --> Valkey[(Valkey Cache)];
      OHC_API -- Cache Miss --> Postgres[(PostgreSQL Central Ledger)];
      MarketingAgent[Marketing Agent] -->|Generates SEO HTML| CDN;
      OperationsAgent[Operations Agent] -->|Invalidates Cache on Stock Change| Valkey;
      OperationsAgent -->|Purges Edge| CDN;
  ```
  **Mobile UX Flow:**
  - The storefront loads instantly (Target: <1s on 3G) for Fatima and Maya’s customers.
  - The UI remains fully functional on a 375px viewport with large 44x44px touch targets.
  - Stale-while-revalidate (SWR) patterns are used in the frontend to ensure users see content immediately while fresh data is fetched in the background.

  **AI Agent Integration Points:**
  - **The Operations Agent** manages cache invalidation. When Maya updates a price or an item sells out, the agent instantly purges the specific `tenant_id:product_catalog` cache key.
  - **The Marketing Agent** automatically triggers a pre-rendering process when the website is updated. It generates highly optimized, static HTML injected with relevant meta tags and structured data, pushing it directly to the edge for crawlers.

  **Key Design Decisions:**
  - Rely on HTTP Cache-Control headers (`s-maxage`, `stale-while-revalidate`) for the CDN layer.
  - Use Valkey for the intermediate application-level cache to shield PostgreSQL.
  - Completely hide the caching and SEO complexity from the owner.

  ### 4. Implementation Prompt
  **Implementer Instructions:**
  As the core backend engineer, implement the Edge-Caching layer and Agentic SEO Pre-rendering for the public storefront API.
  - Add Valkey (Redis) caching to the catalog and tenant configuration fetching routes in `src/server/services/`.
  - Ensure the API responses include appropriate `Cache-Control` headers for CDN compatibility.
  - Implement a cache invalidation service that the Operations Agent can trigger when a tenant updates their catalog or inventory.
  - Integrate the Marketing Agent to generate and serve static HTML with SEO metadata for crawlers.
  - **Acceptance Criteria:** A public storefront page load must not hit the Postgres database if the cache is hot. Cache must be automatically invalidated upon catalog changes. Search engine crawlers must receive fast-loading, pre-rendered HTML. All changes must include comprehensive E2E tests validating cache behavior and SEO rendering.

  ### Priority
  P1

  ### Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
