issue_title: "[Architecture] Edge-Caching Dynamic Storefronts"
issue_description: |
  # [Edge-Caching Dynamic Storefronts] Ultra-Fast Global Storefront Delivery

  ## Problem Statement
  Small business owners like Maya (the baker) and Priya (the boutique owner) depend on their online storefronts to load instantly for customers, especially on mobile devices with poor cellular connections. However, OneHumanCorp (OHC) currently lacks a robust mechanism to serve dynamic catalog and pricing data at edge-caching speeds. A slow-loading storefront leads to lost sales and poor search engine rankings. Competitors like Shopify and Vercel-hosted Next.js sites excel in this area by delivering sub-100ms load times globally.

  ## Research Report
  - **Competitive Landscape**:
    - Shopify leverages Cloudflare's edge network for lightning-fast image and catalog delivery.
    - Vercel and Netlify use Edge Functions and CDN caching to deliver pre-rendered dynamic content.
  - **Current OHC Deficit**:
    - Storefront requests currently route to central regional servers, incurring latency overheads for distant customers.
    - Dynamic content (inventory, specific localized pricing) bypassing caches means the central database must frequently resolve these read-heavy workloads.
  - **Strategic Need**:
    - Implement a globally distributed Edge-Caching capability for OHC storefronts.
    - Offload Read-Heavy catalog loads from core DB instances to Edge nodes.
    - Support instant dynamic invalidation (e.g., when a flash sale ends or an item goes out of stock).

  ## Design Doc

  ### Mobile UX Flow (375px First)
  1. **End-Customer Journey**:
     - A customer taps Maya’s Instagram link.
     - The OHC storefront loads instantly (<100ms) with product images and cached prices directly from a nearby edge node.
     - If stock is critical (e.g., last item), a background lightweight fetch checks actual availability invisibly as the user browses, ensuring the "Add to Cart" button is perfectly accurate.

  2. **Business Owner UX**:
     - For Maya, edge caching is completely invisible. She sees no technical settings.
     - When she updates a price or uploads a new cake photo, an AI Operations Agent automatically triggers targeted cache invalidation tags.

  ### Zero Trust & Security (SPIFFE/SPIRE)
  - **Multi-Tenant Isolation**: Edge caching nodes will rigorously partition cache pools per tenant by embedding the `tenant_id` within the caching key/namespace.
  - **Secure Identity**: Storefront Worker Edge compute nodes will be issued short-lived x509 SVIDs via SPIRE, allowing them to mutually authenticate (mTLS) with the Core OHC Database. This ensures that unauthorized requests from outside the edge workers cannot query core catalog endpoints.

  ### Architecture Data Entities (ER Diagram)
  ```mermaid
  erDiagram
      TENANT {
          string id PK
          string name
          string region
      }
      CATALOG_ITEM {
          string id PK
          string tenant_id FK
          string name
          float price
          int inventory_count
      }
      CACHE_CONFIG {
          string tenant_id PK, FK
          string cache_strategy
          int ttl_seconds
      }
      EDGE_CACHE_TAG {
          string id PK
          string tenant_id FK
          string entity_ref
          datetime expires_at
      }
      TENANT ||--o{ CATALOG_ITEM : owns
      TENANT ||--o| CACHE_CONFIG : uses
      TENANT ||--o{ EDGE_CACHE_TAG : issues
      CATALOG_ITEM ||--o{ EDGE_CACHE_TAG : bound_to
  ```

  ### Architecture Sequence
  ```mermaid
  sequenceDiagram
      participant CustomerMobile as Customer (Mobile 375px)
      participant EdgeNode as Edge Cache Node (CDN)
      participant StorefrontWorker as Edge Worker (Compute)
      participant CorePlatform as OHC Core Platform DB
      participant AIAdmin as AI Operations Agent

      CustomerMobile->>EdgeNode: GET /maya-cakes
      alt Cache Hit
          EdgeNode-->>CustomerMobile: 200 OK (Instant Storefront Render)
      else Cache Miss
          EdgeNode->>StorefrontWorker: Proxy Request
          StorefrontWorker->>CorePlatform: Fetch Dynamic Catalog Data (mTLS via SPIFFE)
          CorePlatform-->>StorefrontWorker: Returns Catalog Data
          StorefrontWorker-->>EdgeNode: Cache Response & Tags
          EdgeNode-->>CustomerMobile: 200 OK (Storefront Render)
      end

      Note over AIAdmin, CorePlatform: Maya updates cake price
      AIAdmin->>CorePlatform: Mutate Price Data
      CorePlatform->>EdgeNode: Invalidate Cache Tag (maya-cakes)
  ```

  ### Key Design Decisions
  - **Invisible Complexity**: The caching layer, edge computing, and invalidation rules are 100% hidden behind OHC platform abstractions and managed by AI agents. No "Clear Cache" buttons for the user.
  - **Tag-Based Invalidation**: Storefront content will be cached using highly granular tags (e.g., `tenant-123-catalog`, `tenant-123-product-456`).
  - **Graceful Stale-While-Revalidate**: If an edge node experiences core connectivity issues, it serves a slightly stale catalog to maintain storefront uptime, passing the "offline capabilities" target.

  ## Implementation Prompt
  Implement a global Edge-Caching strategy for all public-facing OHC dynamic storefronts. This must include an edge computing proxy layer that automatically caches dynamic catalog, pricing, and variant data close to the end user. Implement a tag-based cache invalidation mechanism so that when a tenant modifies their storefront or inventory, the AI agents can clear only the necessary edge caches. Ensure that end customers loading storefronts experience sub-100ms Initial Page Loads worldwide, and that tenant changes are reflected globally within 2 seconds. The implementation must abstract all CDN configuration away from the end-user. Include SPIRE configuration to guarantee mTLS from edge node workers to core database read endpoints.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
