issue_title: "Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering"
issue_description: |
  # Implement Universal Edge-Cached Dynamic Storefront & Agentic SEO Pre-rendering

  ## Problem Statement
  Non-technical small business owners (like Maya the Baker or Leo the Musician) rely heavily on social media vitality. When a post goes viral, the resulting traffic spike can overwhelm unoptimized, centralized databases, leading to high latency, timeouts, and lost revenue. Furthermore, search engines struggle to index slow, client-side rendered dynamic content, reducing organic discoverability. SMBs lack the technical expertise to configure CDNs, caching layers, or Server-Side Rendering (SSR) for SEO. OHC must bridge this gap invisibly.

  ## Research Report
  ### Market Context
  - SMBs lose out on revenue directly proportional to the latency introduced during viral traffic spikes.
  - Traditional caching requires manual configuration and technical expertise (setting up Cloudflare, Vercel/Next.js).
  - Competitors like Shopify provide edge networks but require third-party apps for advanced SEO. Wix/Squarespace provide easier SEO tools but lack instant, autonomous scaling under massive load.
  ### The OHC Opportunity
  - **Universal Edge Caching**: All storefront reads must hit a global edge cache (e.g., Cloudflare/Nginx Edge Cache) automatically.
  - **Agentic Cache Invalidation**: The Operations Agent instantly purges specific edge cache keys when inventory changes.
  - **Agentic SEO Pre-rendering**: The Marketing Agent autonomously triggers a pre-rendering process upon website updates, generating highly optimized static HTML injected with meta tags and structured data.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Customer
      participant EdgeCache as Edge Cache (Nginx/CDN)
      participant OHC as OHC Server
      participant OpsAgent as Operations Agent
      participant MktgAgent as Marketing Agent
      participant DB as PostgreSQL

      Customer->>EdgeCache: Request Storefront Page
      alt Cache Hit
          EdgeCache-->>Customer: Return Cached HTML (Fast)
      else Cache Miss
          EdgeCache->>OHC: Fetch Page
          OHC->>DB: Query Storefront Data
          OHC-->>EdgeCache: Return Rendered HTML
          EdgeCache-->>Customer: Return HTML
      end

      Note over OpsAgent, DB: Inventory Update Event
      OpsAgent->>DB: Update Inventory (e.g., Sold Out)
      OpsAgent->>EdgeCache: Purge Cache Key (Invalidation)

      Note over MktgAgent, OHC: Storefront Update Event
      MktgAgent->>OHC: Trigger Pre-render
      OHC->>DB: Query Latest Content
      OHC->>EdgeCache: Push Pre-rendered HTML (SEO Optimized)
  ```

  ### UI Wireframes / Screen Flow (375px First)
  - **User Facing (Invisible)**: The business owner doesn't see a complex "CDN Configuration" screen.
  - **Marketing Agent Settings**: In the OHC Mobile App -> Storefront Settings -> "SEO & Discoverability".
  - **Toggle**: "Auto-optimize for Search Engines" (Default: ON).
  - **Status Indicator**: "Your storefront is globally distributed and lightning fast." (Read-only green badge).
  - **Agent Feed Update**: "Marketing Agent pre-rendered your new cake catalog for Google." (Action: Review SEO Metadata).

  ### Mobile UX Flow
  1. Owner (Maya) updates her custom cake catalog via the OHC mobile app.
  2. Maya saves changes.
  3. The Marketing Agent intercepts the "Catalog Updated" event in the background.
  4. The Marketing Agent generates updated SEO metadata, triggers HTML pre-rendering, and pushes it to the edge.
  5. Maya receives an Agent Feed notification: "Your new catalog is live and optimized for Google."

  ### AI Agent Integration Points
  - **Marketing Agent (MktgAgent)**: Subscribes to `StorefrontUpdated` events. Uses an LLM to generate SEO meta tags and JSON-LD structured data based on the content changes. Triggers the pre-rendering pipeline.
  - **Operations Agent (OpsAgent)**: Subscribes to `InventoryUpdated` and `OrderPlaced` events. Triggers targeted cache invalidation via API calls to the Edge Cache layer to ensure product availability is accurate.

  ### Key Design Decisions
  - **No Manual Configuration**: Keep all caching and SEO configurations hidden. Use intelligent defaults.
  - **Agent-Driven Workflows**: Rely on Agents (Marketing/Ops) to handle the complex coordination of cache invalidation and pre-rendering rather than hardcoded monolith logic. This fits the OHC paradigm.
  - **Event-Driven Architecture**: Decouple the frontend mutations from the caching layer using asynchronous background events.

  ## Implementation Prompt
  **To the Implementer Agent:**
  Your mission is to implement the Universal Edge-Cached Dynamic Storefront and Agentic SEO Pre-rendering capabilities.

  **Acceptance Criteria:**
  1. **Edge Cache Layer**: Introduce an edge caching strategy (e.g., using OpenResty/Nginx as defined in docker-compose) for storefront read endpoints.
  2. **Agentic Invalidation**: Implement the `OpsAgent` logic to listen for inventory changes and issue cache purge commands for specific items/storefronts.
  3. **Agentic Pre-rendering**: Implement the `MktgAgent` workflow to generate SEO metadata (using the LLM provider) upon storefront updates and trigger a background job to pre-render the static HTML.
  4. **Mobile UX**: Add a simple, non-technical status indicator in the Storefront settings screen on the Flutter frontend (simulated/implemented) showing the optimization status.
  5. **Testing**: Provide full end-to-end Playwright tests demonstrating a storefront update, cache invalidation, and verifying the returned HTML contains the expected SEO meta tags.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
