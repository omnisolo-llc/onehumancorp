issue_title: "[Architecture] Edge-Caching Dynamic Storefronts & Agentic SEO Pre-Rendering"
issue_description: |
  # [Architecture] Universal Edge-Cached Dynamic Storefronts & Agentic SEO Pre-Rendering

  ## Problem Statement
  For OHC's target personas (like Maya the baker and Priya the boutique owner), speed and discoverability are revenue drivers. If their online storefronts take more than 2 seconds to load, potential customers bounce. Furthermore, if their custom products and AI-generated collections aren't indexable by Google due to client-side-only rendering, they lose organic traffic. Currently, our architecture lacks a unified approach to instantly serving dynamic storefronts at the edge with pre-rendered SEO content. We need a system where catalog updates (inventory changes, new AI-generated product descriptions) seamlessly trigger edge invalidations, ensuring sub-50ms Time To First Byte (TTFB) globally while maintaining 100% SEO visibility.

  ## Research Report
  **Findings & Competitive Analysis:**
  - **Shopify:** Utilizes a globally distributed CDN and Edge compute (Oxygen) to serve storefronts with sub-100ms TTFB. They heavily rely on server-side rendering (SSR) and edge caching for SEO.
  - **Wix/Squarespace:** Provide solid CDN caching, but their reliance on heavy visual builder payloads often results in slower Core Web Vitals compared to custom-built edge solutions.
  - **OHC Opportunity:** By integrating an edge-caching layer directly with our AI Swarm, we can go beyond simple caching. The `Marketing Agent` can automatically generate and pre-render optimized SEO meta-tags and structured data (JSON-LD) when a product is created, pushing the pre-rendered HTML directly to the edge cache. This eliminates the need for the owner to understand technical SEO.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      subgraph Customer
          Browser[Mobile/Desktop Browser]
      end

      subgraph Edge Network (CDN)
          EdgeCache[Edge Cache / Workers]
      end

      subgraph OHC Core
          API[Storefront API]
          Postgres[(Postgres DB)]
          Redis[(Redis Invalidation Queue)]
      end

      subgraph AI Swarm
          MarketingAgent[Marketing Agent: SEO & Metadata]
          OpsAgent[Operations Agent: Inventory]
      end

      Browser -->|Request Storefront| EdgeCache
      EdgeCache -->|Cache Miss| API
      API --> Postgres
      API -->|Return Rendered HTML| EdgeCache

      OpsAgent -->|Updates Inventory| Postgres
      MarketingAgent -->|Generates SEO Data| Postgres
      Postgres -->|Trigger| Redis
      Redis -->|Invalidate Edge Path| EdgeCache
  ```

  ### Mobile UX Flow (375px first)
  - **Merchant View:** Priya adds a new dress to her catalog. She clicks "Publish." A small, non-blocking toast appears: "Publishing to your global storefront..." The Marketing agent silently works in the background to generate SEO tags and trigger the edge cache update. No technical settings are exposed unless she clicks "Advanced SEO."
  - **Customer View:** A customer clicks a link on Instagram. The storefront loads almost instantly (sub-50ms) from the nearest edge node, complete with optimized images and full SEO metadata for social sharing.

  ### AI Agent Integration Points
  - **Marketing Agent:** Hooked into the product creation/update lifecycle. It automatically generates SEO titles, descriptions, and JSON-LD schema based on the product details and pushes this to the database, which then feeds the pre-rendered edge content.
  - **Operations Agent:** Monitors inventory levels. When an item goes out of stock, it triggers a targeted cache invalidation for that product page and the main catalog so the "Sold Out" badge appears instantly globally without requiring a full site rebuild.

  ### Key Design Decisions
  - **Edge-First Delivery:** Storefronts must be served from a CDN with aggressive caching. The origin server should only be hit for cache misses or dynamic actions (like adding to cart).
  - **Event-Driven Invalidation:** We must implement a robust cache invalidation strategy. When a product or collection is updated in Postgres, an event must be published to a queue (e.g., Redis) that triggers a targeted invalidation at the edge.
  - **Pre-rendered HTML for SEO:** The backend must be capable of serving fully rendered HTML (with injected SEO metadata) to bots and initial page loads, not just a blank shell that requires JS to hydrate.

  ## Implementation Prompt
  **Context:** You are an Implementer agent. Your task is to implement the core mechanics for Edge-Cached Dynamic Storefronts and the associated cache invalidation pipeline.
  **User Journey (CUJ):** A merchant updates a product description. The Marketing Agent generates new SEO tags. The system invalidates the specific edge cache for that product. A subsequent customer request hits the origin, receives the newly pre-rendered HTML (with SEO tags), and the response is cached at the edge for future requests.
  **Acceptance Criteria:**
  1.  Implement a `CacheManager` service in the backend that handles cache invalidation logic.
  2.  Set up an event listener (e.g., listening to Postgres triggers or application-level events) that calls the `CacheManager` when a product or storefront setting is updated.
  3.  Integrate the `MarketingAgent` (or a mock representation) to automatically generate and save SEO metadata during product creation/updates.
  4.  Ensure the storefront API endpoint can serve pre-rendered HTML containing the SEO metadata.
  5.  Write unit and E2E tests to verify that updating a product correctly triggers an invalidation event and that the updated content is served on the next request.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
