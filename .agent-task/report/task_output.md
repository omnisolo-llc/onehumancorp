issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Mission Queue Protocol: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Title
  Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Small business owners (SMBs) using OHC—like Maya the Baker or Leo the Musician—often experience massive, unpredictable traffic spikes driven by viral social media posts. Currently, dynamic storefront rendering causes high latency, slow load times, and potential database timeouts during these spikes. Additionally, search engines struggle to index dynamically rendered content, leading to poor organic discoverability and SEO penalties. SMB owners lack the technical expertise to configure CDNs, setup Server-Side Rendering (SSR), or manage SEO metadata. They need a system that is invisible, automatic, and highly performant.

  ## Research Report
  Based on our market research (`docs/business/market_research/[research]_universal_edge_cached_dynamic_storefront_seo.md`), competitor platforms either:
  1. Rely heavily on third-party apps for advanced SEO (Shopify).
  2. Require high technical expertise (Vercel/Next.js).
  3. Lack true autonomous edge-caching and instant pre-rendering for dynamic storefronts (Wix/Squarespace).

  OHC must differentiate itself by providing **invisible and autonomous** edge caching and SEO optimization.
  - **Universal Edge Caching**: All storefront reads must hit an edge cache (e.g., Cloudflare) automatically.
  - **Agentic Cache Invalidation**: Inventory updates (e.g., an item selling out) must trigger instant cache invalidation via the Operations Agent.
  - **Agentic SEO Pre-rendering**: Storefront updates via the Marketing Agent must autonomously trigger pre-rendering, generating static HTML with optimized meta tags and structured data pushed to the edge.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant User
      participant MarketingAgent
      participant OperationsAgent
      participant CoreBackend
      participant DB
      participant EdgeCache
      participant WebCrawler

      User->>MarketingAgent: "Update my storefront banner and add new product."
      MarketingAgent->>CoreBackend: Update storefront content
      CoreBackend->>DB: Persist updates
      MarketingAgent->>CoreBackend: Trigger SEO Pre-rendering
      CoreBackend->>CoreBackend: Generate Static HTML & Meta Tags
      CoreBackend->>EdgeCache: Push Static HTML to Edge Cache
      EdgeCache-->>MarketingAgent: Caching complete
      MarketingAgent->>User: "Storefront updated and optimized!"

      WebCrawler->>EdgeCache: Request Storefront
      EdgeCache-->>WebCrawler: Fast, Static HTML
  ```

  ### Mobile UX Flow
  - The feature is completely **invisible** to the owner. There are no toggles, complex settings, or "SEO optimization" buttons required.
  - The owner simply updates their store or inventory via natural language interactions with the Marketing or Operations Agents on their 375px mobile device.
  - The Agent confirms the action via an "Agent Proposal" card, and upon approval, the caching and pre-rendering happen entirely in the background.

  ### AI Agent Integration Points
  - **Marketing Agent**: Monitors changes to storefront content (descriptions, images, layout). When a change is approved, it triggers the pre-rendering pipeline to update SEO metadata and push the new static HTML to the edge.
  - **Operations Agent**: Monitors inventory and pricing changes. When critical business state changes (e.g., an item goes out of stock), it issues a targeted cache invalidation request to the Edge Cache to ensure data consistency.

  ### Key Design Decisions
  - **Zero Configuration**: The system must operate without any user intervention.
  - **Proactive Invalidation**: We cannot rely on Time-to-Live (TTL) alone. The Agents must proactively invalidate caches to prevent selling out-of-stock items.
  - **Agentic Orchestration**: The intelligence lies in the Agents coordinating the technical tasks (pre-rendering, cache invalidation) in response to natural language commands or business events.

  ## Implementation Prompt
  Implement the backend infrastructure for the Universal Edge-Cached Dynamic Storefront.

  **Critical User Journey (CUJ) / Acceptance Criteria**:
  1. Create a `CacheManager` service capable of pushing static content to an edge cache interface and invalidating specific cache keys based on `tenant_id` and `resource_id`.
  2. Extend the `MarketingAgent` to listen for storefront update events. Upon an update, the agent must trigger a pre-rendering process that generates a basic static HTML representation of the storefront (including semantic meta tags for SEO) and pushes it to the `CacheManager`.
  3. Extend the `OperationsAgent` to listen for inventory depletion events. When an item reaches zero stock, the agent must call the `CacheManager` to invalidate the cache for that specific product or storefront page.
  4. The implementation should be verifiable via integration tests simulating cache hits, misses, and invalidation triggered by agent actions. Do not prescribe specific edge providers (e.g., Cloudflare); use an interface that can be mocked for local testing.

  ## Priority
  P1 (High)

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
