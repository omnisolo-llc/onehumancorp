issue_title: "[architecture]_edge_cached_dynamic_storefronts"
issue_description: |
  # [Architecture] Edge-Caching Dynamic Storefronts & Agentic UI Engine

  ## Problem Statement
  Small business owners like Maya (the baker) and Priya (the boutique owner) need beautiful, fast-loading storefronts that update instantly when they change their inventory or pricing via the mobile app. Currently, typical e-commerce platforms either rely on slow server-side rendering for dynamic data, or static site generation that requires full rebuilds when inventory changes. If a user in London visits Maya's storefront hosted in the US, latency can lead to dropped sales. OHC needs a dynamic, globally edge-cached storefront architecture that allows AI agents to instantly mutate UI components and inventory states without requiring manual deploys or rebuilds by the business owner.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify:** Utilizes Hydrogen (React-based framework) and Oxygen (global edge hosting). Provides excellent edge-caching and dynamic storefronts, but requires significant developer effort to customize or integrate deep AI mutations.
  - **Wix/Squarespace:** Fast and reliable but highly monolithic. UI mutations require using their heavy web builders. Not designed for autonomous AI agent-driven UI generation.
  - **Vercel/Next.js:** Provides the ideal primitives (ISR - Incremental Static Regeneration, Edge middleware) but needs to be abstracted away from the business owner completely.

  **Gaps Identified:**
  OHC currently lacks a global edge-caching strategy for user storefronts that supports instant, agent-driven invalidation. When the Operations Agent updates Maya's inventory after an offline tap-to-pay transaction, the storefront must reflect this globally in milliseconds. We also lack a standardized way for the Marketing Agent to generate and inject dynamic promotional UI components (e.g., a "Flash Sale" banner) directly into the storefront.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Global Edge Network (CDN/Workers)
          Edge[Edge CDN] --> ISRApi[Next.js App Router];
          Edge --> ImageOptimizer[Edge Image Optimizer];
      end

      ISRApi --> Cache[(Distributed KV Cache - Redis)];
      ISRApi -- "Cache Miss / Revalidate" --> Gateway[OHC API Gateway];

      subgraph Control Plane
          Gateway --> MainDB[(Cloud Postgres)];
          Gateway --> Agents[AI Agent Swarm];
      end

      subgraph Agent Departments
          Agents --> OpsAgent[Ops Agent: Inventory Mutation];
          Agents --> MarketingAgent[Marketing Agent: UI Generation];
      end

      OpsAgent -- "Triggers" --> InvalidationWorker[Cache Invalidation Queue];
      MarketingAgent -- "Updates" --> ComponentRegistry[(UI Component Registry)];
      InvalidationWorker -- "Purge Tags" --> Cache;
  ```

  ### Mobile UX Flow (375px First)
  1. **Storefront Management:** Maya opens the OHC mobile app. The "Storefront" tab displays a live preview of her website, perfectly scaled for a 375px viewport.
  2. **AI UI Generation:** Maya types, "Add a Mother's Day special banner." The Marketing Agent generates a translucent, glassmorphism-styled banner component and injects it into the `ComponentRegistry`.
  3. **Instant Preview & Publish:** Maya sees the preview instantly. Upon tapping "Publish", the Operations Agent triggers the `InvalidationWorker`.
  4. **Global Edge Update:** The cache is purged via targeted cache tags, and the next customer visiting Maya's site sees the new banner in <50ms without a full site rebuild.

  ### AI Agent Integration Points
  - **Operations Agent:** Monitors inventory and pricing changes (including offline tap-to-pay syncs) and autonomously triggers targeted cache invalidation tags to keep the edge cache fresh.
  - **Marketing Agent:** Can autonomously construct and propose new Next.js React Server Components (RSCs) based on business goals, storing them in the `ComponentRegistry` for immediate rendering.

  ### Key Design Decisions
  - **Incremental Static Regeneration (ISR):** We will leverage Next.js App Router with targeted `revalidateTag` to update specific product pages or UI components rather than rebuilding the entire site.
  - **Headless Component Registry:** The storefront will read from a dynamic registry of UI components, allowing AI agents to inject new sections (like a testimonial carousel) without touching the core codebase.
  - **Zero-Trust Multi-Tenancy:** Each tenant's cache keys and invalidation tags are strictly isolated to prevent cross-tenant cache poisoning.

  ## Implementation Prompt
  Implement the Edge-Cached Dynamic Storefront Engine and Component Registry.
  - **User-Facing Outcome:** Business owners can view a live preview of their storefront in the mobile app. When they or an AI agent makes a change to inventory or UI, the published storefront updates globally in milliseconds without a visible "build" process.
  - **CUJ:** Maya asks the AI to create a new "Vegan Cake" product. The AI creates the DB record, generates a product image, and updates the storefront. A customer in another country immediately sees the new product loading instantly from a local edge node.
  - **Acceptance Criteria:**
    - Storefront must achieve >95% cache hit rate with <100ms TTFB globally.
    - Implementing a targeted cache invalidation mechanism triggered by backend mutations.
    - Support a dynamic component registry where AI agents can push new UI modules.
    - All storefronts must pass strict mobile-first usability metrics on 375px viewports.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
