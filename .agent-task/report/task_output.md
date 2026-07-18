issue_title: "Implement Edge-Cached Dynamic Storefront Architecture for Instant Scaling"
issue_description: |
  # Research Report: Edge-Cached Dynamic Storefront Architecture

  ## Title
  Implement Universal Edge-Cached Dynamic Storefront Architecture for Instant Scaling

  ## Problem Statement
  Small business owners like Maya the Baker or Leo the Musician rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, lost revenue, and SEO penalties. Current solutions either require complex technical configurations (Vercel/Next.js) or expensive enterprise plans (Shopify Plus). OHC needs an invisible, autonomous edge-caching layer that instantly scales dynamic storefronts without any configuration from the non-technical owner.

  ## Research Report
  - **Market Context**: SMBs lack the technical expertise to configure CDNs or Server-Side Rendering (SSR). They need "enterprise-grade" performance out-of-the-box.
  - **Competitor Analysis**: Shopify uses Cloudflare but requires manual optimization. Wix/Squarespace provide basic caching but struggle with sudden viral spikes.
  - **The OHC Differentiator**: True "Invisible AI Automation." The OHC architecture must automatically push all storefront reads to a global edge cache. When inventory changes (e.g., an item sells out), the Operations Agent must autonomously invalidate the specific edge cache key globally, ensuring accurate stock levels without user intervention.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Browser] -->|Request| B(Cloudflare / Edge CDN)
      B -->|Cache Miss| C[OHC Storefront Service]
      C --> D[PostgreSQL / Redis]
      B -->|Cache Hit| A

      E[Operations Agent] -->|Inventory Delta Detected| F[Edge Cache Invalidator]
      F -->|Purge Specific Key| B

      G[Marketing Agent] -->|Storefront Update| H[Agentic SEO Pre-renderer]
      H -->|Push Static HTML| B
  ```

  ### Mobile UX Flow
  - **Invisible to User**: There is no UI configuration for edge caching. It is a core platform capability. The owner only experiences the benefit of their storefront remaining fast during a viral spike.
  - **Agent Notification (Optional)**: If a massive spike occurs, the Agent Feed may show a card: "Your store is experiencing 10x traffic! We've automatically scaled your infrastructure to handle it."

  ### AI Agent Integration Points
  - **Operations Agent**: Monitors inventory changes and triggers targeted cache invalidations.
  - **Marketing Agent**: Automatically pre-renders optimized static HTML for search engine crawlers when storefront content is updated.

  ### Key Design Decisions
  - **Universal Edge Caching**: Default on for all public storefront routes.
  - **Targeted Invalidation**: Only purge the specific product or category page that changed, not the entire site cache.
  - **Agentic Pre-rendering**: Use AI to generate and inject optimal SEO meta tags and structured data into the pre-rendered HTML before pushing to the edge.

  ## Implementation Prompt
  Implement the backend infrastructure to support universal edge caching for public storefront routes. Develop the `EdgeCacheInvalidator` service that the Operations Agent can call to purge specific cache keys when inventory or product details change. Design the system to be completely invisible to the end-user (no configuration toggles). Assume Cloudflare or a similar CDN will be placed in front of the application.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
