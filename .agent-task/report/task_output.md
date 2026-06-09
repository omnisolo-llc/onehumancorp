issue_title: "Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, lost revenue, and SEO penalties due to slow client-side rendering. Furthermore, SMBs lack the technical expertise to configure CDNs, caching layers, Server-Side Rendering (SSR), or Static Site Generation (SSG) for optimal SEO discoverability.

  ## Research Report
  - **Market Context:** Our research indicates a major gap in current platform offerings. When SMBs experience viral traffic, they often face site crashes or extremely slow load times, driving down conversions.
  - **Competitor Analysis:**
    - *Shopify*: Offers strong edge network capabilities (via Cloudflare), but advanced SEO optimization often requires third-party apps, adding to the "App Tax".
    - *Vercel/Next.js*: Excellent for developers, providing ISR and Edge computing, but completely inaccessible to non-technical users.
    - *Wix/Squarespace*: Provide basic SEO tools but require manual configuration and lack autonomous, instant scalability during massive, unpredictable spikes.
  - **The OHC Differentiator:** OHC must provide enterprise-grade performance and discoverability invisibly. This involves an automated edge-caching layer and agent-driven SEO pre-rendering that require zero manual configuration from the user.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer Request] --> B{Cloudflare Global Edge Cache}
      B -- Cache Hit --> C[Serve Pre-rendered HTML]
      B -- Cache Miss --> D[OHC Core Storefront Engine]
      D --> E[PostgreSQL Ledger]
      F[Operations Agent] -->|Inventory Change| G[Edge Cache Invalidation]
      G --> B
      H[Marketing Agent] -->|Content/SEO Update| I[Agentic SEO Pre-rendering Worker]
      I -->|Push Static HTML| B
  ```

  ### Mobile UX Flow (375px)
  - The feature is fundamentally **invisible** to the end-user during setup.
  - The owner's view on the mobile dashboard (375px) remains clean. They will see notifications in their Agent Feed from the Marketing Agent: "Your new product 'Vegan Chocolate Cake' has been indexed and pre-rendered for Google. Expected SEO impact: +15% traffic."

  ### AI Agent Integration
  - **Operations Agent ("The Manager"):** When an item sells out or its price changes, the Operations Agent immediately triggers a targeted cache invalidation at the edge to prevent overselling.
  - **Marketing Agent ("The Promoter"):** Continuously monitors the product catalog and site content. When a change is detected, it autonomously triggers the Agentic SEO Pre-rendering pipeline. This pipeline generates highly optimized static HTML with injected meta tags and structured data (JSON-LD) and pushes it to the edge cache.

  ### Data Model & Invariants
  - Introduce an `EdgeCacheConfig` and `PreRenderJob` entity within the database.
  - Ensure strict multi-tenant isolation so cache invalidations are scoped entirely to a single `tenant_id`.

  ## Implementation Prompt
  **Feature Name:** Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  **Target Persona:** Maya the Baker

  **Outcome:** Maya updates a cake description on her phone. Without her knowing, the Marketing Agent pre-renders the new page, optimizes the SEO meta tags, and pushes it to a global edge cache. When her Instagram post goes viral the next day, her storefront loads instantly for 100,000 visitors without touching the OHC central database.

  **Next Actions for Engineering:**
  1. Implement the Edge Cache integration layer (e.g., Cloudflare API or similar CDN) to handle automated cache invalidation upon inventory state changes.
  2. Build the Agentic SEO Pre-rendering Worker that the Marketing Agent can invoke. This worker should generate optimized static HTML and structured data for the storefront.
  3. Integrate the Operations Agent to trigger targeted cache purges when `InventoryCount` changes or a product is marked as `is_sold_out`.
  4. Create Playwright E2E tests simulating a product update and verifying that the corresponding cache invalidation event is emitted and the pre-rendered SEO content is generated.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
