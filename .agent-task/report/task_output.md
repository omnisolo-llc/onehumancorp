issue_title: "Research: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Architecture

  ## Problem Statement
  Small businesses (SMBs) using OHC often experience slow load times during traffic spikes (e.g., from a viral social media post) and suffer from poor search engine visibility. Traditional platforms (like Shopify or Wix) provide edge caching or SEO tools, but they frequently require manual configuration, complex third-party apps, or deep technical knowledge. Non-technical users need enterprise-grade performance and autonomous, proactive SEO optimizations to occur invisibly in the background.

  ## Research Report
  - **Market Landscape:** E-commerce giants like Shopify use robust edge networks (e.g., Cloudflare) for fast global delivery, but advanced SEO often relies on paid apps. The Next.js ecosystem provides powerful Server-Side Rendering (SSR) and Incremental Static Regeneration (ISR), but it's inaccessible to non-technical users. Platforms like Wix and Squarespace offer simpler SEO tools but still demand manual configuration and lack instant, dynamic scalability during massive, unpredictable spikes.
  - **OHC Opportunity:** OHC must deliver "instant" loading (sub-100ms Time to First Byte - TTFB) globally while maintaining 100% dynamic capabilities (e.g., "Sold Out" states synchronizing within milliseconds to prevent overselling). Furthermore, OHC can leverage AI Agents to autonomously manage SEO without the user ever touching a meta tag or configuring a CDN.
  - **The Gap Identified:** There is a missing link between high-performance enterprise e-commerce (complex, fast) and accessible SMB platforms (simple, slow/manual SEO). OHC needs an invisible architecture that bridges this gap by automatically edge-caching storefront reads and proactively pre-rendering SEO-optimized static HTML via AI.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Customer / Search Crawler] -->|Request| B{Edge Network CDN}
      B -->|Cache Hit| C[Static HTML / Edge Cached Assets]
      B -->|Cache Miss| D[PostgreSQL Central Ledger]
      E[Operations Agent] -->|Inventory Change Detected| F[Edge Cache Invalidation Service]
      F -->|Purge Surrogate Key| B
      G[Marketing Agent - The Promoter] -->|Content/SEO Update Detected| H[Agentic SEO Pre-rendering Engine]
      H -->|Generate JSON-LD & Meta Tags| I[Static HTML Generator]
      I -->|Push Updated HTML| B
  ```

  ### Mobile UX Flow
  - **Zero Configuration:** The beauty of this architecture is that it is completely invisible to the user. There are no toggles for "Enable CDN" or "Configure SEO".
  - **Agent Feed Notification (375px):**
    - The user opens their OHC mobile app.
    - They see an Action Card in the Unified Agent Feed from "The Promoter" (Marketing Agent): "I've detected rising local searches for 'Vegan Custom Cakes'. I've generated a new SEO-optimized landing page for your store to capture this traffic."
    - User clicks: "[Preview & Approve]".
    - The preview shows the fast-loading, generated page. Upon approval, it's instantly pre-rendered and pushed to the edge cache.

  ### AI Agent Integration
  - **The Promoter (Marketing Agent):** Constantly analyzes the tenant's product catalog, customer reviews, and market trends. It autonomously generates structured SEO metadata (JSON-LD), highly optimized meta tags, and alt text for images. When content changes, it triggers the pre-rendering engine to generate static HTML snapshots for web crawlers and pushes them to the edge cache.
  - **The Manager (Operations Agent):** Monitors inventory levels. If an item sells out in-store (via POS) or online, The Manager instantly triggers the edge cache invalidation service to purge the specific surrogate cache key for that product, preventing double-booking and ensuring the storefront reflects accurate real-time stock.

  ### Key Design Decisions
  - **Invisible by Default:** Do not expose complex caching rules or SEO metadata editors to the non-technical user unless they explicitly enter an "Advanced" mode.
  - **Agent-Driven SEO:** Move away from reactive SEO (user fills out a form) to proactive SEO (Agent analyzes trends and creates optimized content autonomously).
  - **Surrogate Keys for Invalidation:** Use tagging/surrogate keys (e.g., `tenant-id:123`, `entity:product:456`) in the edge cache to allow the Operations Agent to precisely and instantly invalidate only the affected pages without clearing the entire site cache.

  ## Implementation Prompt
  **Feature:** Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering Engine
  **Target Persona:** Maya the Home Baker (who needs her site to load instantly when her Instagram post goes viral, and needs to be found on Google without knowing what a meta tag is).

  **Next Actions for Engineering:**
  1.  **Edge Integration:** Implement a caching layer (or simulate one locally for testing) that utilizes surrogate keys for fine-grained cache invalidation.
  2.  **Agentic SEO Pre-rendering Engine:** Extend the `MarketingAgent` (`The Promoter`) to include an asynchronous job that generates static, SEO-optimized HTML (including JSON-LD schema) based on the tenant's current product catalog and pushes it to the edge cache.
  3.  **Operations Cache Invalidation:** Connect the inventory update events in the backend to the cache invalidation service so that when a product is modified or sold out, its specific edge cache is instantly purged.
  4.  **Verification:** Implement Playwright E2E tests to verify that a storefront page is served from the cache, that the cache is correctly invalidated upon an inventory change, and that the pre-rendered HTML contains the Agent-generated SEO metadata.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []