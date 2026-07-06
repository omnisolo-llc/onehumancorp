issue_title: "Design Research: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## 1. Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to:
  - **High Latency & Timeouts:** Frustrating potential customers and increasing bounce rates.
  - **Lost Revenue:** Every second of delay directly impacts conversion rates.
  - **SEO Penalties:** Search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability.
  - **Complexity Barrier:** SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO.

  ## 2. Research Report
  - **Market Context:** Platforms like Shopify offer strong edge network capabilities (via Cloudflare) for fast global delivery, but SEO is often complex and requires third-party apps. Vercel/Next.js provides excellent developer tools (ISR, Edge computing) but is inaccessible to non-technical users. Wix/Squarespace provide easier SEO tools but still require manual configuration and lack autonomous, instant scalability during massive spikes.
  - **The OHC Opportunity:** By leveraging universal edge caching and agentic SEO pre-rendering, OHC can provide enterprise-grade performance and discoverability invisibly to the user.
  - **Competitor Gaps:**
    - *Shopify:* Strong edge, but SEO requires manual effort or apps.
    - *Vercel/Next.js:* Too technical for the target persona.
    - *Wix/Squarespace:* Manual SEO configuration, lacks autonomous scaling.

  ## 3. Design Doc
  ### Data Model & Sync (PostgreSQL & Redis)
  - **Central Ledger (PostgreSQL):** The ultimate source of truth.
  - **Edge Cache (e.g., Cloudflare/Fastly):** All storefront reads hit the global edge cache.
  - **Agentic Cache Invalidation:** When the Operations Agent updates inventory (e.g., an item sells out), it instantly purges the specific edge cache key globally via API, ensuring accurate stock levels.

  ```mermaid
  erDiagram
      EdgeCache ||--o{ Storefront : serves
      OperationsAgent ||--o{ EdgeCache : invalidates
      MarketingAgent ||--o{ Storefront : prerenders
      OperationsAgent ||--o{ Inventory : updates
      Inventory ||--o{ Storefront : reflects
      MarketingAgent ||--o{ SEOMetadata : generates
  ```

  ```mermaid
  sequenceDiagram
      participant Customer
      participant EdgeCache
      participant Storefront
      participant OperationsAgent
      participant MarketingAgent

      Customer->>EdgeCache: Request Storefront
      EdgeCache-->>Customer: Serve Cached Page

      OperationsAgent->>Inventory: Update Stock (Item Sold Out)
      OperationsAgent->>EdgeCache: Invalidate Cache Key

      MarketingAgent->>Storefront: Content Updated
      MarketingAgent->>EdgeCache: Push Prerendered HTML

      Customer->>EdgeCache: Request Storefront (Post Update)
      EdgeCache->>Storefront: Fetch Fresh Content (Cache Miss)
      Storefront-->>EdgeCache: Return Fresh Content
      EdgeCache-->>Customer: Serve Fresh Page
  ```

  ### AI Agent Coordination
  - **Marketing Agent (Agentic SEO Pre-rendering):** When the Marketing Agent updates the website, it autonomously triggers a pre-rendering process. This generates highly optimized, static HTML injected with relevant meta tags and structured data, pushing it directly to the edge.

  ### Mobile UX Flow (375px)
  1. **Owner View (Dashboard):** The owner sees a simplified "Performance & SEO" card. It displays basic stats (e.g., "Site is running blazing fast globally") without exposing complex CDN settings.
  2. **Agent Notifications:** The owner receives push notifications when the Marketing Agent has automatically optimized a new page for SEO and pushed it to the edge.

  ## 4. Implementation Prompt
  **Feature Name:** OHC Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering
  **Target Persona:** Maya the Baker (experiences viral traffic spikes from Instagram).
  **Outcome:** Maya's storefront remains blazing fast during a viral spike, and her new product pages are automatically optimized for SEO and pushed to the edge without her lifting a finger.

  **Next Actions:**
  1. Implement the Edge Cache invalidation logic triggered by inventory changes (coordinated by the Operations Agent).
  2. Develop the Agentic SEO Pre-rendering pipeline triggered by the Marketing Agent when site content changes.
  3. Create the simplified "Performance & SEO" dashboard card for the mobile view (375px).

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
