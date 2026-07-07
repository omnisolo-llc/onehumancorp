issue_title: "Implement Edge-Cached Storefront with Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## Research Report
  Based on competitive analysis:
  - **Shopify:** Offers strong edge network capabilities (via Cloudflare) for fast global delivery of storefronts. SEO is robust but often requires third-party apps for advanced optimization.
  - **Vercel/Next.js Ecosystem:** The gold standard for developers (ISR, Edge computing), but inaccessible to non-technical users without significant development investment.
  - **Wix/Squarespace:** Provide easier SEO tools, but they still require manual configuration and lack the autonomous, instant scalability of true edge architectures during massive, unpredictable spikes.

  To differentiate, OHC must provide **Universal Edge Caching** combined with **Agentic SEO Pre-rendering** autonomously, requiring no user configuration.

  ## Design Doc
  ### Architecture
  ```mermaid
  graph TD
      A[User Request] -->|Edge Cache (Cloudflare/Fastly)| B(Edge Nodes)
      B -->|Cache Miss| C[OHC API Server (Rust)]
      C --> D[(PostgreSQL)]
      E[Operations Agent] -->|Inventory Update| F[Invalidation Queue]
      F -->|Purge Key| B
      G[Marketing Agent] -->|Content Update| H[Pre-rendering Service]
      H -->|Push Static HTML| B
  ```

  ### Mobile UX Flow
  - The feature is **completely invisible** to the user in their day-to-day operations.
  - **Analytics View:** The only visible aspect is a premium, translucent glass "Performance Card" in the 375px mobile Tauri dashboard, showing metrics like "Global Load Time (<50ms)" and "SEO Health (100/100)".

  ### AI Agent Integration Points
  - **Operations Agent ("The Manager"):** When inventory changes (e.g., an item sells out in-store via POS), this agent instantly triggers an edge cache purge for that specific product and storefront.
  - **Marketing Agent ("The Promoter"):** When product descriptions or images are updated, this agent autonomously kicks off a headless browser pre-rendering job, injecting optimal meta tags, OpenGraph data, and structured schema (JSON-LD), and pushes the compiled static HTML to the edge.

  ## Implementation Prompt
  Implement the foundation for Universal Edge Caching and Agentic SEO Pre-rendering.

  1.  **Cache Invalidation Framework:** Create a Rust service in the backend that handles cache invalidation requests (e.g., via a trait that can be implemented for different CDN providers, with a mock/local implementation for development).
  2.  **Agentic Triggers:** Hook up the `Operations Agent` (or equivalent inventory update logic) to trigger cache invalidations when stock levels change.
  3.  **Pre-rendering Hook:** Hook up the `Marketing Agent` (or equivalent product/content update logic) to emit an event that *would* trigger a pre-rendering job (the actual headless browser rendering can be a stub or simple HTML string replacement for this first iteration).
  4.  **UI Verification:** Ensure the 375px mobile dashboard (Tauri/React) displays a simplified "Storefront Performance" card indicating the automated edge caching and SEO status.

  *Acceptance Criteria:*
  - A mock edge cache invalidation is triggered when an inventory count reaches zero.
  - A mock SEO pre-render event is emitted when a product description is updated.
  - A dashboard card in the UI shows edge performance status.
  - E2E tests verify the flow from inventory update to invalidation event.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
