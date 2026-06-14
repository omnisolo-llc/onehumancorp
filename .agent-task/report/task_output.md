issue_title: "Implement Edge-Cached Storefront Infrastructure with Agentic Cache Invalidation"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Small business owners rely on social media vitality. When a post goes viral, the resulting traffic spike overwhelms unoptimized central databases, causing slow load times, frustrated customers, and lost revenue. In addition, dynamic client-side rendering hurts organic discoverability since search engines struggle to index it. Current tools like Vercel or Cloudflare are too complex for non-technical users to configure. OHC needs an invisible, autonomous architecture that provides enterprise-grade performance without manual setup.

  ## Research Report
  - **Competitive Analysis**: Shopify leverages Cloudflare for fast edge delivery but requires apps for advanced SEO. Vercel/Next.js excels technically but is inaccessible to SMBs. Wix/Squarespace need manual configuration and lack autonomous scalability during unpredictable spikes.
  - **OHC Differentiator**: True invisible automation. The storefront must be globally cached by default. When the Operations Agent updates inventory, it must autonomously trigger a global cache purge for that specific item to prevent overselling. When the Marketing Agent updates the site, it should trigger Agentic SEO Pre-rendering to inject static HTML with meta tags directly at the edge.

  ## Design Doc
  - **Architecture**:
    - **Universal Edge Caching Layer**: All storefront reads route through an edge cache layer autonomously.
    - **Agentic Cache Invalidation System**: A service module that listens to inventory state changes and automatically issues a cache purge request for the affected product keys.
    - **SEO Pre-rendering Pipeline**: An asynchronous worker triggered by content updates that renders the site to static HTML with structured data, updating the edge cache.

  ```mermaid
  graph TD
      Client[Mobile/Desktop Client] --> EdgeCache[Universal Edge Caching Layer]
      EdgeCache -->|Cache Miss| API[OHC API Server]
      EdgeCache -->|Cache Hit| Client

      API --> DB[(PostgreSQL Ledger)]

      OpsAgent[Operations Agent] -->|Inventory Update| API
      MarketingAgent[Marketing Agent] -->|Content Update| API

      API --> InvalidationService[Cache Invalidator]
      InvalidationService -->|Purge Keys| EdgeCache

      API --> SEOPipeline[Agentic SEO Pre-rendering]
      SEOPipeline -->|Static HTML + Meta Tags| EdgeCache
  ```

  - **Multi-Tenant Data Model**: Define entities in the database to track cache states and invalidation rules per tenant, ensuring strict multi-tenant isolation.
  - **AI Agent Integration**: The Operations Agent (handling inventory) and Marketing Agent (handling content) will utilize a new capability to automatically clear stale edges when they perform state mutations.
  - **Mobile UX Flow**: The owner sees no complex CDN settings. They only see an "Optimized for Speed" badge in their Agent Feed or Storefront settings, with details hidden behind "Advanced Settings". Everything must fit perfectly on a 375px mobile screen.

  ## Implementation Prompt
  - Create the core data models and database migrations for tracking storefront cache configurations per tenant.
  - Implement a cache invalidation service that can purge specific route or product keys across the edge layer.
  - Integrate this service with the Operations Agent workflow so that when inventory changes (e.g., an item sells out), the cache is automatically invalidated without user intervention.
  - Develop the Agentic SEO Pre-rendering background job that generates static HTML with meta tags upon content updates.
  - Ensure all database queries respect row-level security and strict multi-tenant isolation rules.
  - Write comprehensive E2E Playwright tests verifying that storefront changes propagate correctly through the caching layer and that edge latency targets are met.
  - Ensure the UI for any related settings follows the OHC Premium Token library (Glassmorphism, 375px mobile-first layout).

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
