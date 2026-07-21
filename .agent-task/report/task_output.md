issue_title: "Implement Edge-Cached Dynamic Storefront & MCP AI Pre-rendering"
issue_description: |
  # Research Report: Agentic Edge-Cached Dynamic Storefront

  ## Problem Statement
  Small businesses (SMBs) struggle with slow load times during traffic spikes and poor search engine visibility due to the limitations of dynamic rendering on traditional platforms. High latency leads to lost revenue, and poor SEO reduces organic discoverability. SMB owners like Maya the Baker or Leo the Musician lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) for SEO optimization. They need enterprise-grade performance and discoverability provided invisibly and autonomously.

  ## Research Findings
  Our research into competitor platforms (Shopify, Wix, Squarespace) and modern edge architectures (Vercel/Next.js ecosystem) reveals a significant gap for SMBs. While developer-focused tools offer advanced Edge computing and Incremental Static Regeneration (ISR), they are inaccessible to non-technical users. Platforms like Shopify offer edge caching via Cloudflare but often require third-party apps for advanced SEO optimization. Wix and Squarespace provide easier SEO tools but still require manual configuration and lack autonomous scalability.

  ## Architecture Design

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] --> B[Global Edge Cache (Cloudflare)]
      B -->|Cache Hit| C[Deliver Static Pre-rendered HTML]
      B -->|Cache Miss| D[Origin Server / OHC Platform]
      D --> E{Agentic Intercept}
      E --> F[Operations Agent]
      F -->|Update Inventory| G[MCP Edge KV Sync]
      G -->|Invalidate/Sync| B
      E --> H[Marketing Agent]
      H -->|Generate SEO Shell| I[MCP SEO Generator]
      I -->|Push Static HTML| B
  ```

  ### Data Model & Sync Protocol
  We propose an **Agentic Edge-Cached Dynamic Storefront**. This system will leverage universal edge caching automatically and use AI agents to manage cache invalidation and SEO pre-rendering.
  - All storefront reads must hit a global edge cache (e.g., Cloudflare) automatically.
  - The `EdgeCachingMcpServer` handles tools for synchronizing inventory updates to the edge KV store and generating static HTML shells for SEO.

  ### UI Wireframes & Mobile UX Flow (375px First)
  - **Storefront View (Mobile):** Customer lands on a pre-rendered, lightning-fast static page. The UI uses glassmorphism cards and clean typography.
  - **Interaction:** The page loads the basic HTML shell instantly (containing title, description, and SEO meta tags).
  - **Dynamic Injection:** Inventory data (e.g., "Only 2 left in stock!") is fetched asynchronously from the Edge KV store and injected into the DOM, ensuring real-time accuracy without blocking the initial render.
  - **Action:** Add to Cart button is prominently displayed. Tapping it reserves inventory via distributed locks.

  ### AI Agent Coordination
  - **Operations Agent**: Automatically calls `mcp_edge_kv_sync` to invalidate or update the specific edge cache key globally when inventory changes (e.g., an item sells out), preventing overselling.
  - **Marketing Agent (MCP SEO Generator)**: Autonomously triggers SEO pre-rendering using `mcp_seo_generator` upon website updates. It generates highly optimized static HTML injected with relevant meta tags (JSON-LD, OpenGraph) and pushes it to the edge.

  ## Proposed Solution & Next Steps
  We need to fully implement the `mcp_seo_generator`, `mcp_edge_kv_sync`, and `mcp_edge_worker_simulation` capabilities within the `EdgeCachingMcpServer` located in `src/server/tools/edge_caching/server.rs`. These tools are currently stubbed.
  - The `mcp_seo_generator` should autonomously generate static HTML shells containing proper SEO metadata.
  - The `mcp_edge_kv_sync` should handle synchronizing inventory updates to a real edge KV store (simulated via Redis in development).
  - Implement end-to-end Playwright tests to verify the cache invalidation and SEO pre-rendering flows.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
