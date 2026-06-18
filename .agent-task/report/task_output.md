issue_title: "[Architecture] Edge-Cached Dynamic Storefronts & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Edge-Cached Dynamic Storefronts & Agentic SEO Pre-rendering

  ## Problem Statement
  Small business owners rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, lost revenue, and poor search engine visibility due to dynamic rendering limitations. Legacy platforms require complex CDNs and SSG configuration that non-technical owners like Maya or Leo cannot manage.

  ## Research Report
  - **Market Gap:** While Vercel/Next.js offer edge compute and ISR for developers, and Shopify offers strong edge caching via Cloudflare, there is no platform that autonomously handles cache invalidation and SEO pre-rendering for non-technical SMBs without configuration.
  - **OHC Differentiator:** By leveraging edge caching and AI-driven SEO pre-rendering, OHC can provide enterprise-grade performance and discoverability invisibly.
  - **Universal Edge Caching:** All storefront reads hit a global edge cache automatically. No configuration needed by the user.
  - **Agentic Cache Invalidation:** When the Operations Agent updates inventory, it instantly purges the specific edge cache key globally, ensuring accurate stock levels and preventing overselling.
  - **Agentic SEO Pre-rendering:** When the Marketing Agent updates the website, it autonomously triggers a pre-rendering process to generate highly optimized, static HTML injected with meta tags and structured data, pushing it directly to the edge.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] --> B[Edge CDN Node]
      B -- Cache Hit --> C[Static Pre-rendered HTML]
      B -- Cache Miss --> D[Storefront API]
      D --> E[PostgreSQL DB]
      F[Operations Agent] -- Inventory Update --> G[Edge Cache Invalidation API]
      H[Marketing Agent] -- Content Update --> I[SEO Pre-render Worker]
      I --> G
  ```

  ### Mobile UX Flow
  - Invisible to the end user. The storefront simply loads instantly even during high traffic.
  - The business owner receives a notification from the Marketing Agent when SEO pre-rendering is complete, confirming the site is optimized for search engines.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory and triggers targeted cache invalidation when stock levels change.
  - **Marketing Agent:** Detects content changes (new products, updated descriptions) and triggers the SEO Pre-render Worker to generate and push static HTML to the edge.

  ### Key Design Decisions
  - **Zero Configuration:** The edge caching and pre-rendering must be entirely invisible to the user.
  - **Targeted Invalidation:** Use surrogate keys or cache tags to invalidate only the specific pages affected by an inventory or content update.
  - **Asynchronous Pre-rendering:** The SEO pre-rendering process must happen in the background without blocking the user's workflow.

  ## Implementation Prompt
  Implement the Agentic SEO Pre-rendering and Edge Cache Invalidation system.
  - The system must automatically generate static HTML for product pages when the Marketing Agent detects a change.
  - The Operations Agent must accurately invalidate specific edge cache entries when inventory levels are updated to prevent overselling.
  - Build the necessary background workers to handle pre-rendering and cache purging asynchronously.
  - Write E2E tests simulating a product update and verifying that the corresponding cache keys are invalidated.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: "P1"
issue_category: "research"
issue_type: "task"
issue_label: ["agent-report"]
assignees: []
