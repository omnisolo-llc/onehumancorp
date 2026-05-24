issue_title: "Edge-Caching Dynamic Storefronts"
issue_description: |
  # [Architecture] Edge-Caching Dynamic Storefronts

  ## Problem Statement
  Small business owners like Maya (the baker) or Priya (the boutique owner) often drive explosive traffic through viral social media posts on platforms like TikTok or Instagram. When a video showcasing Maya's new vegan cake goes viral, her OHC storefront might instantly experience thousands of concurrent visitors. If every visitor triggers a direct database query to render the page, the site will suffer from high latency or crash entirely, resulting in lost sales and frustrated customers. These business owners need a storefront that loads instantly (sub-100ms) anywhere in the world, while still accurately displaying dynamic, real-time information such as "Only 1 left in stock!" or personalized AI chat widgets.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify:** Utilizes a globally distributed edge network (powered by Cloudflare) to aggressively cache static storefront assets and HTML pages. They manage dynamic elements (like cart state and inventory) via lightweight asynchronous JavaScript calls or edge computing (Workers).
  - **Vercel / Next.js:** Pioneers in Edge caching and Serverless computing, using Stale-While-Revalidate (SWR) and edge middleware to serve personalized content instantly while revalidating data in the background.
  - **Wix:** Employs extensive CDN caching but sometimes struggles with the "Time to Interactive" metric on heavier dynamic templates compared to headless commerce solutions.

  **Gaps Identified:**
  OneHumanCorp currently lacks a defined multi-tier Edge-Caching Architecture. We need a system that ensures the storefront is pushed to the edge (CDN/PoPs) for instant global delivery, while safely isolating and instantly invalidating cache when dynamic business state changes (e.g., an item selling out).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Mobile Visitor 375px
          Browser[Customer Browser] --> EdgeCDN[Edge CDN / PoP];
      end

      subgraph Edge Layer
          EdgeCDN -- Cache Hit --> Browser;
          EdgeCDN -- Cache Miss / Dynamic Request --> API[OHC Global API Gateway];
      end

      subgraph Core Platform
          API --> CoreLogic[Storefront Engine];
          CoreLogic --> CacheDB[(Redis / In-Memory Cache)];
          CoreLogic --> MainDB[(Cloud Postgres Ledger)];
      end

      subgraph Agent Departments
          OpsAgent[Operations Agent] -->|Inventory Depleted| Invalidater[Cache Invalidation Queue];
          Invalidater --> EdgeCDN: Purge Cache / SWR Trigger;
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Viral Surge:** A customer taps the link in Maya's TikTok bio.
  2. **Instant Load:** The storefront's HTML, CSS (Glassmorphism design tokens), and product images load in under 100ms from the nearest Edge node, displaying the catalog flawlessly on their 375px screen.
  3. **Dynamic Hydration:** A subtle loading skeleton on the "Stock" indicator resolves in ~200ms, asynchronously fetched from the API, revealing "Only 2 left!".
  4. **Action:** The customer taps the large, accessible "Add to Cart" button (styled with OHC premium translucent cards) and checks out smoothly.

  ### AI Agent Integration Points
  - **Operations Agent:** Constantly monitors inventory levels. When an item is purchased, this agent instantly triggers a targeted cache invalidation payload to the Edge CDN, ensuring subsequent visitors see the updated "Sold Out" state.
  - **Marketing Agent:** Tracks page load times and bounce rates. If edge caching rules are causing stale personalized content, it can dynamically adjust the Time-To-Live (TTL) configuration for specific tenant blocks.

  ### Key Design Decisions & Security
  - **Stale-While-Revalidate (SWR):** Adopt SWR caching policies. The CDN serves the stale cache instantly to the user while concurrently fetching the updated page from the origin in the background.
  - **Asynchronous Dynamic Hydration:** The core page layout and visual assets are aggressively cached at the edge. Highly volatile data (cart count, specific inventory numbers) is hydrated client-side via lightweight API calls that bypass the edge cache.
  - **Zero-Trust Multi-Tenancy:** Cache keys are strictly namespaced by `tenant_id` and domain. Cache invalidation requests must carry SPIFFE SVIDs asserting the identity of the backend service triggering the purge, preventing malicious cross-tenant cache eviction.

  ## Implementation Prompt
  Implement the Edge-Caching Dynamic Storefront architecture.
  - **User-Facing Outcome:** Storefronts must load globally in under 100ms, even during massive traffic spikes. When an item goes out of stock, the storefront must reflect this change to new visitors within 1 second.
  - **CUJ (Critical User Journey):**
    1. Thousands of users click a storefront link simultaneously.
    2. The Edge CDN serves the cached storefront instantly.
    3. A user buys the last item.
    4. The Operations Agent triggers a cache purge.
    5. The very next user sees the item as "Sold Out."
  - **Acceptance Criteria:** Implement Edge CDN integration with SWR caching policies. Separate static layout rendering from dynamic state hydration (inventory/cart). Ensure cache invalidation is tightly coupled to the Operations Agent's inventory tracking. All UI must maintain the 375px mobile-first Glassmorphism design system. Do not prescribe specific CDN providers (e.g., Cloudflare vs. Fastly) in the code; use an abstract caching interface.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []