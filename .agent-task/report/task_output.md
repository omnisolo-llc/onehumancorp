issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  ## Mission Queue Protocol Brief

  ### Problem Statement
  Non-technical SMB owners rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ### Research Report
  - **Market Context**: Competitors like Shopify offer strong edge network capabilities via Cloudflare. Vercel/Next.js is the gold standard for edge computing but is inaccessible to non-technical users.
  - **The OHC Opportunity**: Provide an invisible and autonomous edge caching and SEO pre-rendering solution. All storefront reads hit a global edge cache automatically.
  - **Agentic Value**: When the Operations Agent updates inventory, it instantly purges the specific edge cache key globally. When the Marketing Agent updates the website, it autonomously triggers a pre-rendering process, generating highly optimized, static HTML injected with relevant meta tags and pushing it to the edge.

  ### Design Doc
  - **Architecture Diagram**:
    ```mermaid
    graph TD;
      Client-->EdgeCache[Global Edge Cache];
      EdgeCache-- Cache Miss -->API[OHC Core API];
      API-->DB[(PostgreSQL Ledger)];
      MarketingAgent[Marketing Agent]-->PreRenderService[Pre-rendering Service];
      PreRenderService-->EdgeCache;
      OperationsAgent[Operations Agent]-->EdgeCache[Purge Cache on Update];
    ```
  - **Mobile UX Flow**: The user experiences no change in UI complexity. All caching and pre-rendering happens invisibly in the background. Pages load instantly on 375px viewports even under heavy load.
  - **AI Agent Integration Points**:
    - **Operations Agent**: Triggers targeted cache invalidation upon inventory or pricing updates.
    - **Marketing Agent**: Autonomously triggers SEO pre-rendering when content or layout changes occur.

  ### Implementation Prompt
  **Target Persona**: Maya the Baker
  **User-Facing Outcome**: Maya's custom cake storefront handles a viral Instagram spike without breaking a sweat, loading instantly for every visitor globally. Her site ranks higher on Google automatically because the Marketing Agent generates and pushes optimized static HTML to the edge cache.
  **Critical User Journey (CUJ)**:
  1. Maya updates the price of a cake via the mobile app.
  2. The Operations Agent updates the database and instantly purges the specific edge cache key for that product.
  3. Maya changes the layout of her homepage.
  4. The Marketing Agent triggers the pre-rendering service to generate a new static HTML file for the homepage, complete with optimized meta tags, and pushes it to the edge cache.
  5. Customers visiting the site instantly receive the updated, edge-cached version.
  **Acceptance Criteria**:
  - Storefront read requests must be served from the edge cache with minimal latency.
  - Operations Agent must correctly and immediately invalidate cache keys upon inventory/price changes.
  - Marketing Agent must successfully trigger and deploy pre-rendered HTML to the edge upon content changes.

  ### Priority
  P1

  ### Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
