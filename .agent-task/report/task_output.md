issue_title: "Implement Edge-Cached Dynamic Storefront with Agentic SEO Pre-rendering"
issue_description: |
  # Mission Queue Protocol: Agentic Edge-Cached SEO & Dynamic Storefront

  ## Problem Statement
  Non-technical SMB owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency and timeouts. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) / Static Site Generation (SSG) for SEO. OHC currently lacks an invisible, autonomous edge-caching and SEO pre-rendering pipeline.

  ## Research Report
  - **Market Context**: Platforms like Shopify offer strong edge network capabilities via Cloudflare. Vercel/Next.js are gold standards but inaccessible to non-technical users. Wix/Squarespace provide easier SEO tools but require manual setup and lack autonomous instant scalability.
  - **The OHC Differentiator**: OHC's approach must be invisible and autonomous. All storefront reads hit a global edge cache automatically. Agentic Cache Invalidation occurs instantly when inventory updates. Agentic SEO Pre-rendering autonomously triggers when the website is updated, generating highly optimized, static HTML with meta tags and structured data, pushing it directly to the edge.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner as Store Owner
      participant MarketingAgent as Marketing Agent
      participant Queue as ohc_job_queue (PostgreSQL)
      participant Worker as Pre-Render Worker
      participant ObjectStore as MinIO / GCS
      participant EdgeCache as Nginx Edge / CDN
      participant Customer as Web Customer

      Owner->>MarketingAgent: Updates Product / Storefront via OHC App
      MarketingAgent->>Queue: Enqueues SEOPreRenderJob (tenant_id)
      Queue->>Worker: Dequeue (SKIP LOCKED)
      Worker->>Worker: Generates Static HTML with SEO Meta Tags & JSON-LD
      Worker->>ObjectStore: Uploads HTML artifact
      Worker->>EdgeCache: Issues cache invalidation for tenant routes
      Customer->>EdgeCache: GET /tenant/product
      EdgeCache-->>Customer: Serves 200 OK (Cache Miss, fetched from ObjectStore -> Cache Hit)
  ```

  - **Data Model & Invariants**:
    - `StorefrontConfiguration` (Tenant config for edge rendering).
    - `SEOPreRenderJob` (Tracks pre-rendering tasks).
    - Redis cache for mapping tenant routes to pre-rendered HTML paths in object storage (MinIO/GCS).
  - **AI Department Coordination**:
    - **Marketing Agent**: Automatically triggers `SEOPreRenderJob` when a user updates their storefront or adds products. It injects structured data and meta tags.
    - **Operations Agent**: Triggers targeted cache invalidation when inventory levels cross thresholds (e.g., item sells out).
  - **Edge Flow**:
    1. Request hits Nginx Edge Cache.
    2. Cache miss -> Nginx fetches pre-rendered static HTML from OHC Core (served from object storage/Redis).
    3. Real-time dynamic updates (like cart status) use client-side hydration or edge workers.

  ### Mobile UX Flow
  - The feature is **entirely invisible** to the non-technical owner in their day-to-day workflow.
  - In the "Advanced Settings" or "Marketing Insights" section on a 375px viewport, the owner sees a card: "Your store is globally accelerated and SEO optimized."
  - They see a list of pre-rendered pages (Home, Product X) and a "Refresh SEO" button for manual intervention, though AI handles it automatically.

  ## Implementation Prompt
  **Target Persona**: Maya the Baker
  **Outcome**: Maya updates her cake catalog. The Marketing Agent automatically generates optimized SEO HTML and pushes it to the edge cache. When her Instagram post goes viral, her OHC store loads instantly from the cache without hitting the central Postgres database, handling the spike effortlessly.

  **Critical User Journey (CUJ) & Acceptance Criteria**:
  1. User updates a product description or storefront configuration.
  2. The system enqueues an `SEOPreRenderJob`.
  3. A background worker (handled by the Marketing Agent) processes the job, generates static HTML for the storefront with injected SEO meta tags and JSON-LD structured data.
  4. The generated HTML is stored and its route is cached in Redis/Nginx.
  5. Subsequent GET requests to the public storefront route serve the pre-rendered HTML with minimal latency.
  6. **UI Verification**: Add a simple read-only "Storefront Performance" card in the Marketing or Settings tab to show "Edge Cache Active" and "Last SEO Optimization time".

  **Next Actions for Implementer**:
  - Define the data model for `SEOPreRenderJob` (tenant_id, path, status, generated_html_url).
  - Implement a background worker (using the existing job queue) to handle pre-rendering.
  - Create the API endpoints to serve the pre-rendered HTML and trigger manual invalidation.
  - Update the frontend to display the "Storefront Performance" card.
  - Ensure zero-trust isolation: each tenant's pre-rendered pages are strictly isolated.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
