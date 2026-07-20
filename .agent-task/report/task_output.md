issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  ### Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, lost revenue, and SEO penalties. SMBs lack the technical expertise to configure CDNs, caching layers, or SSR/SSG.

  ### Research Report
  - **Market Context**: Competitors like Shopify offer strong edge network capabilities. Vercel/Next.js are the gold standard for developers but inaccessible to SMBs. Wix/Squarespace require manual configuration and lack autonomous scalability during spikes.
  - **OHC Gap**: OHC needs to provide Universal Edge Caching and Agentic SEO Pre-rendering invisibly and autonomously.

  ### Design Doc
  - **Architecture Diagram**:
    ```mermaid
    graph TD
        User[Browser/Bot] --> EdgeCache[Edge Cache / CDN]
        EdgeCache -- Cache Miss --> OHC_Server[OHC Server]
        OHC_Server --> Postgres[(Postgres)]
        OpsAgent[Operations Agent] -->|Invalidate Cache| EdgeCache
        MarketingAgent[Marketing Agent] -->|Pre-render & Push| EdgeCache
    ```
  - **Mobile UX Flow**:
    - The storefront loads instantly (under 1 second) on a 375px mobile screen, even on a slow 3G connection, because the HTML is served directly from the edge cache.
  - **AI Agent Integration Points**:
    - **Operations Agent**: Automatically purges specific edge cache keys when inventory updates (e.g., an item sells out) to prevent overselling.
    - **Marketing Agent**: Autonomously triggers pre-rendering of SEO-optimized static HTML (with meta tags and structured data) and pushes it to the edge when storefront content is updated.
  - **Key Design Decisions**:
    - All storefront reads hit a global edge cache automatically.
    - Implement programmatic cache invalidation via the Operations Agent.
    - Implement an agentic pre-rendering pipeline for SEO.

  ### Implementation Prompt
  Implement the Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering feature.
  1. Set up the infrastructure for Universal Edge Caching (e.g., configuring Nginx as an edge cache in the Docker Compose stack).
  2. Implement programmatic cache invalidation in the Operations Agent when inventory or storefront data changes.
  3. Implement an autonomous pre-rendering pipeline triggered by the Marketing Agent to generate and push SEO-optimized HTML to the edge cache.
  Acceptance Criteria: Storefronts must be served from the edge cache. Updates to inventory must instantly invalidate the relevant cache keys. The system must autonomously pre-render SEO-optimized HTML upon content changes.

  ### Priority
  P0

  ### Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
