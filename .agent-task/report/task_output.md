issue_title: "Implement Edge-Cached Storefront Integration for SEO and Traffic Management"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  SMBs hosted on OHC rely heavily on social media traffic spikes. When a post goes viral, the influx of users can crash unoptimized storefronts. Furthermore, client-side rendered websites suffer in SEO rankings. SMB owners like Maya the Baker or Leo the Musician do not have the technical skills to configure SSR, Edge Caching, or CDNs. OHC needs to build an invisible, autonomous edge-caching and SEO pre-rendering layer.

  ## Research Report
  - **Market Gap:** Shopify relies heavily on Cloudflare, and Next.js / Vercel provide ISR/Edge caching, but both expect some technical knowledge for setup and maintenance. Wix has caching but scales poorly under massive, sudden traffic spikes without expensive enterprise tiers.
  - **OHC Opportunity:** Implement an "invisible" caching and pre-rendering mechanism controlled by the existing Operations and Marketing AI Agents.
  - **The Solution:** A globally distributed edge cache layer where storefront reads are served instantly. The AI agents handle cache invalidation upon inventory updates or content changes, and autonomously trigger SEO pre-rendering to generate static HTML for web crawlers.

  ## Design Doc
  ### System Architecture (Mermaid)
  ```mermaid
  graph TD;
      CustomerBrowser[Customer Browser/Mobile] --> EdgeCache[Edge Cache CDN];
      SearchCrawler[Googlebot/Crawlers] --> EdgeCache;
      EdgeCache -- Cache Miss --> OHCServer[OHC API Server];
      OHCServer --> PostgresDB[(Postgres DB)];
      OpsAgent[Operations Agent] --> CacheInvalidationEngine[Cache Invalidation System];
      CacheInvalidationEngine -- Purge Key --> EdgeCache;
      MarketingAgent[Marketing Agent] --> SEOPreRenderEngine[SEO Pre-Render Engine];
      SEOPreRenderEngine -- Static HTML --> EdgeCache;
  ```

  ### Mobile UX Flow
  - From the owner's perspective (Priya, Maya), there is **zero UI**. The storefront simply loads blazingly fast (< 200ms) even during a viral Instagram spike.
  - From the customer's perspective, the storefront responds instantaneously on a 375px viewport.

  ### AI Agent Integration
  - **Operations Agent:** Hooks into the inventory management system. When stock hits 0, it calls the Cache Invalidation API to immediately purge the edge cache for that specific product, preventing overselling.
  - **Marketing Agent:** Upon updating site copy or adding new images, it triggers the SEO Pre-Render Engine to generate an updated static HTML payload with embedded Open Graph tags and pushes it to the edge.

  ## Implementation Prompt
  **Target Persona:** Maya the Baker
  **Outcome:** Maya's cake shop goes viral on TikTok. Her storefront effortlessly handles 10,000 simultaneous visitors because the site is served entirely from an edge cache. Search engines correctly index her site because the Marketing Agent pre-rendered the HTML. Maya didn't have to click a single configuration button.

  **Next Actions:**
  1.  **Cache Configuration Data Model:** Implement internal structures in the backend to manage edge cache headers and invalidation keys.
  2.  **Agent Invalidation Hook:** Extend the Operations Agent to trigger cache invalidation upon inventory depletion or critical updates.
  3.  **SEO Pre-rendering Service:** Create a background worker that generates static HTML payloads for storefronts when the Marketing Agent signals a content change, pushing them to the CDN/Edge cache.
  4.  **Zero-Config Enforcement:** Ensure all of this operates completely invisibly to the tenant.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
