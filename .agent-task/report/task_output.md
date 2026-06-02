issue_title: "[Architecture] Edge-Caching Dynamic Storefronts Implementation"
issue_description: |
  ## Problem Statement
  As OneHumanCorp grows, our users (like Maya the Baker or Priya the Boutique Owner) will experience traffic spikes, particularly when their social media posts go viral. Currently, every page load hits the centralized multi-tenant database, which can lead to latency degradation or dropped requests. Non-technical users need their online store to load instantly on a customer's phone, regardless of where they are in the world, while still displaying real-time inventory and pricing updates.

  ## Research Report
  - **Competitor Analysis:**
    - **Shopify:** Utilizes a globally distributed edge network (Cloudflare) combined with complex cache tagging. They serve storefronts from the edge while using Edge Functions to inject personalized or dynamic elements.
    - **Vercel / Next.js:** Employs ISR (Incremental Static Regeneration) and Edge caching for instant load times without sacrificing dynamic product availability.
  - **Market Needs:** Customers demand sub-second load times on mobile. If a store takes more than 2 seconds to load over 3G, conversion rates plummet.
  - **Current Gap:** OHC currently renders views dynamically from the core API, which is a bottleneck. We need a caching tier that sits in front of the API, tightly coupled with our KAIROS Orchestration engine for instant cache invalidation when data changes.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Buyer Mobile App / Web] -->|Requests Storefront| B(Edge CDN / Cache Tier);
      B -- Cache Hit --> A;
      B -- Cache Miss --> C(OHC API Gateway);
      C --> D[Storefront Service];
      D --> E[(PostgreSQL Read Replica)];

      F[Operations Agent] -->|Updates Inventory| G[Inventory Service];
      G -->|Sold Out Event| H[Cache Invalidation Bus];
      H -->|Purge Key| B;
  ```

  ### Key Design Decisions & Multi-Tenancy
  - **Surrogate Keys / Cache Tags:** Every response from the API must include Cache-Control headers with surrogate keys identifying the `tenant_id` and the specific resource (e.g., `storefront:{tenant_id}`, `product:{product_id}`).
  - **Event-Driven Invalidation:** When an AI agent (like Operations updating stock) or a user changes data, a message is published. A dedicated cache invalidation service consumes this and issues targeted purge requests to the CDN based on the surrogate keys.
  - **Mobile UX Flow:** The storefront loads instantly (<100ms) on a 375px viewport even on slow 3G connections.

  ## Implementation Prompt
  Implement the Edge Caching strategy for multi-tenant dynamic storefronts.
  - Define the HTTP headers required for Cache-Control and Surrogate Keys (e.g., `Cache-Tag: tenant:123, product:456`) in the main API layer.
  - Create a Cache Invalidation Service that listens to internal Redis Pub/Sub events (e.g., `inventory.updated`, `storefront.published`) and purges the appropriate cache tags.
  - Update the KAIROS AI agents (specifically Operations and Marketing) to emit these events when modifying tenant data.
  - Ensure 100% unit test coverage for the invalidation logic and verifiable tenant isolation.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
