issue_title: "Implement Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Mission Queue Protocol: Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Small businesses (like Maya the Baker) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized databases, leading to high latency, timeouts, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## Research Report
  Our research across the e-commerce platform landscape reveals that while platforms like Shopify offer strong edge network capabilities, they still require third-party apps for advanced SEO optimization. Wix and Squarespace provide easier SEO tools but require manual configuration and lack the autonomous scalability of true edge architectures during massive spikes. By implementing Universal Edge-Cached Dynamic Storefronts with Agentic SEO Pre-rendering, OHC will close this gap, delivering unparalleled speed, reliability, and automated discoverability invisibly to non-technical users.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] -->|Edge Cache (e.g. Cloudflare)| B(Edge Nodes)
      B -->|Cache Miss| C[Storefront Next.js Server]
      C -->|Read Data| D[(PostgreSQL Central Ledger)]
      E[Operations Agent] -->|Inventory Update| F[Agentic Cache Invalidation]
      F -->|Purge Key| B
      G[Marketing Agent] -->|Website Update| H[Agentic SEO Pre-rendering]
      H -->|Generate Static HTML + Meta Tags| I[Push to Edge Cache]
      I --> B
  ```

  ### Mobile UX Flow
  There is no direct user-facing UX flow for this feature as it operates invisibly in the background. The user benefits from:
  1. Instantly loading storefronts on mobile devices (375px viewports) even during traffic spikes.
  2. Accurate inventory levels displayed to customers, preventing overselling.
  3. Improved search engine rankings driving organic traffic to their site.

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** Actively monitors stock levels and triggers the Agentic Cache Invalidation process whenever inventory changes.
  - **Marketing Agent ("The Promoter"):** Detects website updates (e.g., new products, blog posts) and autonomously triggers the Agentic SEO Pre-rendering process.

  ### Key Design Decisions
  - **Universal Edge Caching:** All storefront reads must hit a global edge cache automatically. No configuration is needed by the user.
  - **Agentic Cache Invalidation:** Real-time purging of specific edge cache keys to ensure accurate stock levels and prevent double-booking.
  - **Agentic SEO Pre-rendering:** Generating highly optimized, static HTML injected with relevant meta tags and structured data, and pushing it directly to the edge for instant web crawler indexing.

  ## Implementation Prompt
  Implement the Universal Edge-Cached Dynamic Storefront architecture. This involves configuring Next.js to leverage edge caching (e.g., via Vercel Edge Network or Cloudflare Workers). Develop the "Agentic Cache Invalidation" mechanism triggered by inventory updates from the Operations Agent. Finally, build the "Agentic SEO Pre-rendering" pipeline triggered by website updates from the Marketing Agent, ensuring static HTML generation with optimized meta tags and structured data. Verify the implementation through Playwright E2E tests simulating traffic spikes and checking SEO meta tag injection.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
