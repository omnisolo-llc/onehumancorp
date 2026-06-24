issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture

  ## Problem Statement
  Small business owners face slow load times during traffic spikes and poor search engine visibility due to dynamic rendering limitations. Currently, SMB platforms like Wix or Shopify require complex manual configuration for advanced SEO or caching, or they lack autonomous instant scalability. For non-technical owners like Maya (Baker) or Leo (Musician), viral social media traffic can crash their sites or result in lost revenue due to latency, while dynamic content remains invisible to search engine crawlers.

  ## Research Report
  - **Context:** Viral traffic spikes overwhelm unoptimized databases, leading to high latency, lost revenue, and SEO penalties. Search engines struggle to index client-side rendered dynamic content.
  - **Competitive Analysis:**
    - *Shopify:* Uses Cloudflare for edge delivery but requires apps for advanced SEO.
    - *Vercel/Next.js:* Excellent for developers (ISR, Edge), but inaccessible to non-technical users.
    - *Wix/Squarespace:* Easier SEO tools but still manual and less resilient to massive spikes.
  - **OHC Opportunity:** OHC can implement an invisible, autonomous architecture where all storefront reads hit a global edge cache automatically. AI agents manage cache invalidation (e.g., when inventory changes) and SEO pre-rendering (generating static HTML with meta tags and structured data).

  ## Design Doc
  **Architecture Overview:**
  - **Universal Edge Caching:** Storefront reads are served from a global edge cache (e.g., Cloudflare, Fastly).
  - **Agentic Cache Invalidation:** The Operations Agent monitors state changes (e.g., inventory updates) and automatically purges relevant edge cache keys.
  - **Agentic SEO Pre-rendering:** The Marketing Agent detects content updates, triggers a pre-rendering pipeline to generate static HTML with injected meta tags and structured data, and pushes this to the edge for rapid crawler indexing.

  **Architecture Diagram:**
  ```mermaid
  graph TD;
      User[Shopper / Crawler] --> EdgeCache[Edge Cache CDN];
      EdgeCache -- Cache Miss --> OHC_API[OHC Core API];
      OHC_API --> DB[(PostgreSQL)];

      Owner[SMB Owner] --> AdminUI[OHC App];
      AdminUI --> OHC_API;

      OHC_API -- State Change --> OpsAgent[Operations Agent];
      OpsAgent -- Invalidate --> EdgeCache;

      OHC_API -- Content Change --> MktgAgent[Marketing Agent];
      MktgAgent -- Generate Static HTML & SEO Meta --> PreRender[Pre-rendering Service];
      PreRender -- Push --> EdgeCache;
  ```

  **Mobile UX Flow (375px first):**
  - **Invisibility:** The user sees *no* caching or SEO settings by default. Everything is automated.
  - **Performance:** Storefront loads instantly on mobile, even on slow networks.
  - **Advanced View (Optional):** A simple toggle in "Advanced Settings" shows cache hit rates and recent automated SEO optimizations (e.g., "Updated meta tags for 3 products today").

  **AI Agent Integration:**
  - **Marketing Agent:** Handles the pre-rendering and SEO metadata injection.
  - **Operations Agent:** Handles cache invalidation upon inventory or pricing changes.

  ## Implementation Prompt
  Implement the Universal Edge-Cached Dynamic Storefront and Agentic SEO Pre-rendering Architecture. Create the necessary backend services to manage edge cache invalidation and trigger SEO pre-rendering workflows. Define the data models for tracking cache state and SEO metadata. Ensure the Marketing and Operations agents are integrated to autonomously trigger these processes based on state changes (e.g., product updates). Build the corresponding frontend components (if any, typically hidden behind advanced settings or surfaced as agent notifications) ensuring mobile-first responsive design. Verify the flow end-to-end with Playwright tests demonstrating cache invalidation and pre-rendering triggers without manual user intervention.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
