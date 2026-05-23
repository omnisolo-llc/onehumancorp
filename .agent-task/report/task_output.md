issue_title: "[architecture] Intelligent Edge-Caching Dynamic Storefronts"
issue_description: |
  ## Title
  Intelligent Edge-Caching Dynamic Storefronts

  ## Problem Statement
  Small business owners (e.g., Maya the baker, Priya the boutique owner) need their digital storefronts to be blazing fast, visually stunning, and highly reliable, even during unexpected traffic spikes (like going viral on TikTok) or spotty network conditions. Slow loading times or downtime lead directly to lost sales and poor user experience, especially on mobile devices.

  ## Research Report
  - **Target Technology**: Vercel/Cloudflare Workers, Redis, globally distributed CDNs, edge computing.
  - **Competitive Analysis**: Shopify uses their proprietary network and fast CDNs. Wix and Squarespace rely heavily on heavy client-side hydration, which can be slow. OHC needs to pre-render dynamic storefront content at the edge (close to the customer) to achieve sub-second load times globally.
  - **Data & References**: Edge caching reduces TTFB (Time to First Byte) by up to 60%. Next.js App Router with React Server Components allows for optimal edge-rendering capabilities.
  - **Key Challenges**: Caching dynamic content like inventory levels (e.g., "3 items left in stock") without serving stale data. Multi-tenant data segregation at the edge level.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      User[Mobile User / Customer] --> EdgeCDN[Edge Network CDN];
      EdgeCDN --> EdgeWorker[Edge Computing Worker];
      EdgeWorker -- "Cache Hit" --> KVStore[Edge KV / Cache];
      EdgeWorker -- "Cache Miss" --> API[Rust API Server];
      API --> PrimaryDB[PostgreSQL DB];
      API -. "Cache Invalidation" .-> KVStore;
      EdgeWorker --> UI[Server Rendered UI];
  ```

  ### UI Wireframes & Screen Flow
  - **Business Owner View (Operations Dept)**: A simple "Performance" settings card. A visual indicator showing "Storefront optimized for speed: ON". No complex cache TTL settings.
  - **Mobile UX Flow (375px first)**:
      - Customer taps link from Instagram.
      - Page loads instantly (cached edge).
      - Dynamic elements (cart count, inventory) are fetched asynchronously but gracefully without layout shift (skeletons).

  ### AI Agent Integration Points
  - **Ops Agent**: Monitors traffic spikes. If a spike is detected, it proactively adjusts caching strategies and scales read replicas without human intervention.
  - **Marketing Agent**: Analyzes edge analytics to identify high-traffic regions and suggests localized campaigns.

  ### Key Design Decisions
  - Adopt aggressive Edge Caching with Stale-While-Revalidate (SWR) patterns for catalog data.
  - Enforce strict tenant isolation within the Edge KV store (each tenant's cache is logically separated).
  - Utilize a hybrid rendering model: static shell pre-rendered, dynamic user-specific data (cart) fetched client-side.

  ## Implementation Prompt
  Design and implement an intelligent edge-caching layer for dynamic storefronts. The system should automatically cache catalog and generic content at the edge (e.g., Cloudflare Workers or similar) while securely managing multi-tenant data isolation. Ensure that high-frequency dynamic data (like inventory counts) uses a stale-while-revalidate strategy to balance performance and freshness. The underlying infrastructure should be completely hidden from the business owner, managed autonomously by the Operations AI Agent.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
