issue_title: "[research] Build Universal Edge-Cached Dynamic Storefront & SEO Pipeline"
issue_description: |
  # Research Report: Universal Edge-Cached Dynamic Storefront & SEO Pipeline

  ## 1. Problem Statement
  Small business owners (e.g., Maya the Home Baker, Carlos the Handyman) struggle with discoverability and performance. Legacy platforms like Shopify or WordPress require complex caching setups, CDN configurations, and manual SEO optimization. As a result, many SMB websites suffer from slow load times on mobile devices and poor search engine rankings, ultimately losing potential sales. OHC currently lacks a unified, high-performance edge-caching architecture and an autonomous SEO generation pipeline.

  ## 2. Research Report
  - **Market Context**: Performance is a direct driver of conversion. Shopify and BigCommerce heavily leverage CDNs (like Fastly or Cloudflare) but often expose caching complexities to developers or advanced users. Newer platforms (like Vercel/Next.js) offer edge rendering but are developer-centric. Wix and Squarespace handle this invisibly but suffer from generic SEO metadata.
  - **The OHC Opportunity**: OHC can differentiate by offering an edge-cached dynamic storefront that is completely invisible to the user. When an owner updates a product or service, the "Promoter" AI agent autonomously regenerates SEO metadata (titles, descriptions, schema.org JSON-LD) and invalidates the edge cache, ensuring instant global performance without any technical input from the owner.
  - **Competitor Gaps**:
    - *Shopify*: Good performance, but SEO requires manual tuning or third-party apps.
    - *Wix*: Slower dynamic rendering; generic SEO.
    - *Vercel/Next.js*: Best-in-class performance, but not built for non-technical SMB owners out-of-the-box.

  ## 3. Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Owner
      participant App as Mobile App
      participant OperationsAgent as Operations Agent
      participant PromoterAgent as Promoter Agent
      participant API as API Layer
      participant Cache as Edge Cache (CDN)
      participant DB as Database

      Owner->>App: Updates Product/Service
      App->>API: Save Changes
      API->>DB: Persist Changes
      API->>OperationsAgent: Notify Update
      OperationsAgent->>PromoterAgent: Request SEO Update
      PromoterAgent->>API: Generate & Save SEO Metadata
      API->>Cache: Invalidate Cache Keys
      API-->>App: Confirmation
  ```

  ### Mobile UX Flow (375px)
  1. The owner updates a product description or price via the mobile app.
  2. A subtle "Optimizing for search..." toast appears and disappears quickly.
  3. No SEO forms or caching toggles are presented to the user; the process is entirely autonomous.

  ### AI Integration
  - **The Promoter (Marketing Agent)**: Monitors product/service updates. Automatically generates optimized SEO titles, descriptions, and structured data (Schema.org) based on the updated content and business context.

  ### Edge Caching Strategy
  - Implement a stale-while-revalidate caching strategy at the edge.
  - Granular cache invalidation based on entity tags (e.g., `tenant_id`, `product_id`).

  ## 4. Implementation Prompt
  **Feature Name**: Universal Edge-Cached Dynamic Storefront & Autonomous SEO
  **Target Persona**: Maya the Home Baker
  **Outcome**: Maya updates her cake catalog. The system autonomously generates SEO metadata to help her rank for "custom cakes near me" and instantly updates the edge cache, ensuring her site loads in under 1 second globally.

  **Next Actions**:
  1. **Data Model**: Extend the existing product/service schemas to store generated SEO metadata.
  2. **AI Pipeline**: Implement the Promoter Agent's autonomous SEO generation logic upon entity updates.
  3. **Cache Invalidation**: Build a centralized cache invalidation service that hooks into the update lifecycle.
  4. **Edge Rendering**: Adapt the storefront rendering engine to utilize edge caching with stale-while-revalidate headers.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
