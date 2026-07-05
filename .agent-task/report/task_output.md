issue_title: "Implement Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  **Title**: Implement Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  **Problem Statement**:
  SMB owners like Maya (baker) or Leo (musician) rely on social media virality. When a post goes viral, the traffic spike overwhelms central databases, causing timeouts and lost revenue. Additionally, search engines struggle with slow, client-rendered sites, hurting SEO. SMBs lack the technical skill to set up CDNs, edge caching, or SSR/SSG.

  **Research Report**:
  Competitor Analysis:
  - **Shopify**: Uses Cloudflare for edge delivery.
  - **Vercel/Next.js**: Great for developers, but inaccessible to non-technical users.
  - **Wix/Squarespace**: Require manual setup for SEO, lacking automatic edge scalability during unexpected spikes.

  **Design Doc**:
  Architecture Diagram:
  ```mermaid
  sequenceDiagram
      participant Customer
      participant EdgeCache as Edge CDN Cache
      participant OHC Backend
      participant OpsAgent as Operations Agent
      participant SEOMarketingAgent as Marketing Agent

      Customer->>EdgeCache: Request Storefront page
      alt Cache Hit
          EdgeCache-->>Customer: Return cached static page
      else Cache Miss
          EdgeCache->>OHC Backend: Fetch page data
          OHC Backend-->>EdgeCache: Return response
          EdgeCache-->>Customer: Return response
      end

      rect rgb(200, 220, 240)
          note right of OpsAgent: Agentic Caching
          OpsAgent->>OHC Backend: Inventory depleted (Cake sold out)
          OHC Backend->>EdgeCache: Purge specific product cache key globally
      end

      rect rgb(220, 240, 200)
          note right of SEOMarketingAgent: Agentic SEO Pre-rendering
          SEOMarketingAgent->>OHC Backend: Site update published
          OHC Backend->>EdgeCache: Pre-render and push static HTML & Meta Tags
      end
  ```

  Mobile UX Flow:
  1. The feature is invisible to the end user (SMB owner), requiring no manual setup or toggle.
  2. For the end customer, viewing the site on a 375px mobile viewport is near-instantaneous due to edge delivery.

  AI Agent Integration Points:
  - **Operations Agent**: Monitors inventory and automatically invalidates specific cache keys when stock changes to prevent overselling.
  - **Marketing Agent**: Automatically triggers SEO pre-rendering and cache warming when the user publishes new site content.

  Key Design Decisions:
  - **Invisible by default**: Owners don't configure CDNs or cache rules.
  - **Event-driven invalidation**: Cache is purged via Agents reacting to backend state changes, rather than a time-to-live (TTL) approach.

  **Implementation Prompt**:
  Implement an Edge-Cached Dynamic Storefront layer with Agentic SEO Pre-rendering. This requires building a middleware or caching layer that automatically caches storefront read requests globally (simulating edge CDN behavior). Integrate the Operations Agent to listen for inventory change events and automatically invalidate the relevant cache keys. Additionally, implement the Marketing Agent to autonomously pre-render static HTML with optimized SEO meta-tags whenever storefront content is updated, pushing this to the cache. Ensure this entire process requires absolutely zero configuration from the SMB owner.

  Acceptance Criteria:
  - Storefront read requests are served from the cache layer.
  - Inventory updates instantly purge the specific product's cache.
  - Content updates automatically trigger the generation of pre-rendered, SEO-optimized HTML.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
