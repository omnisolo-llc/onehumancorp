issue_title: "Architecture Deep Dive: Distributed Edge-Cached Storefront for SMBs"
issue_description: |
  # Research Report: OHC Edge-Cached Dynamic Storefront & SEO Architecture

  ## Executive Summary
  This report investigates the architectural gap in current small business (SMB) storefront platforms (like Shopify and Wix) where multi-tenant performance, edge caching, and dynamic SEO optimization are often at odds. The objective is to design a high-performance, edge-cacheable, and dynamically localized storefront architecture for OneHumanCorp (OHC) that gives micro-SMEs (like Maya the baker and Priya the boutique owner) enterprise-grade SEO and load times without any technical configuration.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  - **The Status Quo**: Platforms like Shopify provide CDN caching for static assets but struggle with edge-caching fully dynamic, inventory-aware pages without expensive enterprise tiers (Shopify Plus). Wix and GoDaddy rely heavily on client-side rendering (CSR) or slow server-side rendering (SSR), penalizing SEO and First Contentful Paint (FCP).
  - **The OHC Opportunity**: OHC can leverage a modern Next.js/React architecture (or its existing robust frontend setups) combined with a global CDN (e.g., Cloudflare Workers/Vercel Edge) to serve statically generated pages that are incrementally regenerated (ISR) based on inventory and AI agent updates.
  - **The Gap**: Currently, OHC's storefront delivery is too tightly coupled to the main Rust API backend, lacking a specialized edge-delivery layer for high-traffic public storefronts. This causes unnecessary load on the central DB and slower response times for global customers.

  ## 2. Deep Dive Architecture Design (Track 2)
  ### Data Model & Delivery Pipeline
  - **Central Source of Truth**: PostgreSQL (via Rust API) remains the single source of truth for products, pricing, and inventory.
  - **Edge Cache Layer**: Implement a distributed key-value store (e.g., Redis at the edge or Cloudflare KV) to cache rendered HTML and API responses per `tenant_id` and `product_id`.
  - **Cache Invalidation Protocol**: When the Operations Agent updates inventory (e.g., an item sells out) or the Marketing Agent updates a description, the Rust backend publishes an invalidation event. The Edge Cache immediately flushes the specific route, ensuring strong eventual consistency.

  ### Entity-Relationship Diagram
  ```mermaid
  erDiagram
      TENANT {
          string id PK
          string domain
          string default_locale
      }
      PRODUCT {
          string id PK
          string tenant_id FK
          int inventory_count
          float price
      }
      SEO_METADATA {
          string product_id FK
          string title_tag
          string meta_description
          string json_ld
      }
      EDGE_CACHE {
          string cache_key PK
          string rendered_html
          datetime expires_at
      }

      TENANT ||--o{ PRODUCT : owns
      PRODUCT ||--o| SEO_METADATA : has
      PRODUCT ||--o{ EDGE_CACHE : invalidates
  ```

  ### System Sequence Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      participant CDN as Edge Cache / CDN
      participant API as OHC Central API (Rust)
      participant DB as PostgreSQL
      participant Agent as Marketing/Operations Agent

      Customer->>CDN: GET /product/summer-dress (Priya's Store)
      alt Cache Hit
          CDN-->>Customer: Return Cached HTML (< 50ms)
      else Cache Miss
          CDN->>API: Fetch Dynamic Data
          API->>DB: Query Product & SEO
          DB-->>API: Data
          API-->>CDN: Rendered Response
          CDN-->>Customer: Return HTML & Store in Cache
      end

      Note over Customer,Agent: Inventory Purchase Event
      Customer->>API: POST /checkout (Buys last dress)
      API->>DB: Update Inventory (Count = 0)
      API->>Agent: Trigger 'Item Sold Out' Event
      API->>CDN: Publish Invalidation Event (Flush Cache)
      Agent->>API: Update Storefront state to "Sold Out"
  ```

  ### AI Agent Coordination
  - **SEO & Marketing Agent**: Automatically analyzes product descriptions and tenant metadata to generate optimized `<meta>` tags, structured data (JSON-LD for Google Shopping), and alt text for images. These are baked into the edge-cached HTML.
  - **Localization Agent**: Detects the incoming request's `CF-IPCountry` or `Accept-Language` headers at the edge and serves pre-translated content or triggers a background translation task if the cache misses.

  ## 3. Mobile & Security Integrity (Track 3)
  - **Mobile-First UX (375px Flow)**:
    - **Screen 1 (Home/Feed)**: Edge-cached full-bleed product image header. Large 44x44px "Add to Cart" sticky bottom bar. Statically rendered skeleton text that hydrates instantly.
    - **Screen 2 (Product Detail)**: Swipeable image carousel (lazy loaded). AI-generated SEO description below the fold. Sticky checkout button.
    - **Performance**: The edge-rendered storefront must hit strict Web Vitals targets: LCP < 2.5s, CLS < 0.1, and FID < 100ms on a 375px mobile viewport using a simulated 3G connection.
  - **Zero Trust & Rate Limiting**: The edge layer must enforce strict rate limiting to protect the origin API from DDoS attacks, utilizing tenant-scoped API keys (SPIFFE identity for internal microservices).

  ## 4. Implementation Prompt
  **Feature Name**: Edge-Cached Autonomous Storefront Delivery

  **Target Persona**: Priya (Boutique Owner wanting fast online sales and high SEO ranking)

  **Outcome**: A decoupled, edge-cacheable storefront delivery service that serves sub-100ms pages to customers while automatically syncing with the central OHC inventory and SEO agents.

  **Critical User Journey (CUJ)**:
  1. Priya adds a new "Summer Dress" via the OHC Mobile App.
  2. The Marketing Agent automatically generates SEO metadata and structured JSON-LD.
  3. The Rust backend persists the product and triggers an Edge Cache rebuild for Priya's tenant storefront.
  4. A customer in another country visits Priya's OHC storefront. The Edge Cache serves the localized, fully rendered HTML in < 100ms.
  5. The customer buys the dress. The inventory decreases, and a cache invalidation event immediately updates the storefront to show "Low Stock".

  **Next Actions for Engineering**:
  1. **Infrastructure**: Provision an Edge Cache layer (e.g., Cloudflare Workers or Redis Edge).
  2. **Service**: Create a specialized `StorefrontDelivery` service in the Rust backend to handle fast-path reads and cache invalidation webhooks.
  3. **Agent Integration**: Extend the Marketing Agent to generate and persist localized SEO structured data alongside product records.
  4. **Frontend**: Update the public storefront templates to utilize Server-Side Rendering (SSR) with Incremental Static Regeneration (ISR) tied to the Edge Cache.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
