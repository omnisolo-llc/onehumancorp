issue_title: "[Platform] Implement Global Edge Caching & Agentic SEO Pre-rendering for Storefronts"
issue_description: |
  ## Problem Statement
  Small business owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) for SEO.

  ## Research Report
  Our competitive analysis shows that while platforms like Shopify offer strong edge network capabilities (via Cloudflare) for fast global delivery, and Vercel/Next.js are the gold standard for developers, they are either inaccessible to non-technical users or require manual configuration. OHC needs a solution that is **invisible and autonomous**.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] --> B[Global Edge CDN/Cache e.g. Cloudflare]
      B -->|Cache Miss| C[Storefront Application Server]
      C -->|Query| D[Database]
      C -->|Generate HTML| B
      B -->|Cache Hit| A

      E[Operations Agent] -->|Inventory Update| F[Event Bus]
      F --> G[Cache Invalidation Worker]
      G -->|Purge Key| B

      H[Marketing Agent] -->|Storefront Update| I[Pre-rendering Worker]
      I -->|Generate Static HTML with SEO Tags| B
  ```

  ### Mobile UX Flow
  1. This capability is largely invisible to the end user (the business owner).
  2. The only visible change might be a setting in "Advanced Storefront Settings" to manually trigger a cache purge or rebuild, though this should rarely be necessary.
  3. The primary benefit is experienced by the *customer* of the business, who enjoys lightning-fast load times even during traffic spikes.

  ### AI Agent Integration
  - **Operations Agent**: Automatically triggers cache invalidation for specific products when inventory changes (e.g., an item sells out).
  - **Marketing Agent**: Automatically triggers SEO pre-rendering when storefront content (descriptions, images) is updated.

  ## Implementation Prompt
  - Design the architecture for a universal edge caching layer (e.g., Cloudflare Workers or similar) for OHC storefronts.
  - Implement the cache invalidation logic triggered by the Operations Agent.
  - Implement the Agentic SEO pre-rendering pipeline triggered by the Marketing Agent.
  - Ensure all operations are multi-tenant safe and require zero configuration from the business owner.

  ## Priority & Scope
  - **Priority**: P0
  - **Estimated Scope**: Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
