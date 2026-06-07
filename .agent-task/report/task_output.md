issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Mission Queue Protocol: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical SMB owners rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized databases, leading to high latency, timeouts, lost revenue, and SEO penalties. Currently, OHC does not autonomously provide the edge-caching and static pre-rendering that are standard in enterprise tools, leaving SMB users without robust discoverability and scalable performance.

  ## Research Report
  - **Findings**: The research report `docs/business/market_research/[research]_universal_edge_cached_dynamic_storefront_seo.md` indicates that users need an invisible, autonomous system to cache storefront reads globally and pre-render dynamic content into static HTML with SEO tags.
  - **Competitive Advantage**: By removing manual configuration of CDNs, SSR, or SSG, OHC can outcompete Shopify, Wix, and Squarespace for non-technical users looking for instant, scalable storefronts.
  - **Integration Focus**:
    - **Universal Edge Caching**: Automatic caching of public storefront content.
    - **Agentic Cache Invalidation**: The Operations Agent instantly purges edge caches when inventory changes (preventing double booking / overselling).
    - **Agentic SEO Pre-rendering**: The Marketing Agent autonomously triggers a process to render static HTML (with meta and structured data) on website updates, pushing it to the edge.

  ## Design Doc
  ### Architecture Flow (Mermaid.js)
  ```mermaid
  sequenceDiagram
      participant Owner as SMB Owner
      participant MA as Marketing Agent
      participant OA as Operations Agent
      participant Cache as Edge Cache (CDN)
      participant OHC as OHC Backend (PostgreSQL)

      Owner->>MA: Updates Website/Inventory
      MA->>OHC: Triggers Pre-rendering Process
      OHC->>Cache: Pushes Static HTML (SEO Optimized)

      Owner->>OA: Item Sells Out / Inventory Change
      OA->>Cache: Autonomously Invalidates Cache Keys
  ```
  ### Mobile UX Flow (375px)
  - **No new complex UI**: The system works implicitly behind the scenes.
  - **Agent Feed Notification**: A card may appear in the 375px Owner Feed stating "Your store is pre-rendered for maximum speed and SEO" or "Inventory synced globally," giving the user confidence without asking for configuration.

  ### AI Agent Integration
  - **Marketing Agent (The Promoter)**: Rebuilds static assets with updated metadata when storefront products change.
  - **Operations Agent (The Manager)**: Signals cache invalidation upon inventory events.

  ## Implementation Prompt
  Implement the backend support and agent logic required to generate SEO-optimized static HTML for public storefronts and integrate cache invalidation via the Operations Agent. Ensure that the logic correctly intercepts product updates and handles the corresponding static asset regeneration and edge cache invalidation automatically.
  Do not expose any CDN/SEO configuration settings to the user; this process must remain entirely invisible. The result should verify that public endpoints serve pre-rendered HTML and that inventory updates trigger cache purges.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
