issue_title: "[Architecture] Global Edge-Cached Dynamic Storefronts"
issue_description: |
  ## Problem Statement
  For non-technical business owners like Maya (baker) and Priya (boutique owner), their online storefronts must be blazingly fast and available 100% of the time, regardless of traffic spikes (e.g., from a viral TikTok) or regional connectivity issues. Currently, dynamic e-commerce platforms often suffer from slow initial page loads, database bottlenecks during high traffic, and poor performance in remote areas. Users need a system that ensures their storefront is instantly available globally, handles real-time updates (like inventory dropping to zero or price changes), and requires zero configuration of CDNs, caching layers, or scaling rules.

  ## Research Report
  **Competitor Systems Audit:**
  - **Shopify:** Provides robust global CDN caching and handles massive traffic spikes well. However, this is largely opaque to the user and sometimes real-time inventory updates can lag behind the CDN edge caches.
  - **Wix:** Utilizes caching, but dynamic content (like search or complex filtering) can be slow. Mobile performance scores often suffer due to heavy client-side JavaScript.
  - **Vercel/Next.js (General Architecture):** Uses Edge networks and ISR (Incremental Static Regeneration) to serve dynamic content instantly. This is the industry standard for high-performance e-commerce but is completely inaccessible to non-technical users.

  **Gaps Identified:**
  OHC lacks a zero-configuration, globally distributed architecture that guarantees sub-100ms load times for dynamic storefronts while ensuring strict data consistency for critical operations (checkout, inventory decrement). We need an architecture that seamlessly blends edge-cached static assets with instantaneous, zero-trust dynamic multi-tenant data access, without exposing any of this complexity to the business owner.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD;
      subgraph Global Edge Network
          CDN[Edge CDN Node] --> StaticAssets[Cached HTML/CSS/Images];
          CDN --> EdgeWorker[Edge Computing Worker];
      end

      subgraph OHC Cloud Core
          API[OHC API Gateway] --> MainDB[(Cloud Postgres Multi-Tenant Ledger)];
          API --> Cache[(Redis Cache Layer)];
          API --> Agents[AI Agent Swarm];
      end

      subgraph Business Owner App
          MobileApp[OHC Mobile App 375px] --> LocalDB[(Local CRDT)];
          LocalDB -- Realtime Sync --> API;
      end

      CustomerBrowser --> CDN;
      CustomerBrowser --> EdgeWorker;
      EdgeWorker -- Cache Miss / Dynamic Data --> API;

      subgraph Agent Departments
          Agents --> OpsAgent[Ops: Smart Cache Invalidation];
          Agents --> MarketingAgent[Marketing: SEO Metadata Generator];
      end
  ```

  ### Mobile UX Flow (375px First)
  1. **Instant Publishing:** Maya updates a cake price in her OHC mobile app. She taps "Save".
  2. **Invisible Infrastructure:** The app shows a brief "Publishing..." state. Behind the scenes, the OHC Cloud updates the multi-tenant ledger and triggers an AI Operations Agent to intelligently invalidate only the specific edge cache nodes affecting that product.
  3. **Verification:** Within 5 seconds, Maya receives a "Live globally" push notification.
  4. **Customer View:** A customer in another country clicks a link on Instagram. The storefront loads in under 100ms from the nearest Edge CDN node, completely styled with OHC's Premium Glassmorphism design system.

  ### AI Agent Integration Points
  - **Operations Agent (Ops):** Listens to inventory and price changes. Instead of naive global cache purging, it intelligently identifies which specific URLs and edge nodes need invalidation, optimizing CDN costs and ensuring data freshness.
  - **Marketing Agent:** Autonomously generates and injects localized SEO metadata, schema markup, and optimized WebP images into the edge payload based on the business's current catalog, ensuring optimal search engine performance without user intervention.

  ### Key Design Decisions & Security
  - **Zero-Configuration Edge ISR:** We will adopt an Incremental Static Regeneration pattern at the edge. The system generates static HTML for product pages and serves them from the CDN. Edge workers handle dynamic segments (like current cart state or personalized recommendations).
  - **Zero-Trust Multi-Tenancy at the Edge:** Edge workers must validate tenant boundaries. Even cached data must be strictly segregated by `tenant_id` to prevent cross-tenant data bleed during high-concurrency requests.
  - **Offline-to-Edge Sync:** Changes made on the mobile app while offline (via CRDTs) are synced to the OHC Cloud when online, triggering the edge rebuild process asynchronously, guaranteeing eventually consistent global storefronts.

  ## Implementation Prompt
  Implement the Global Edge-Cached Dynamic Storefronts architecture.
  - **User-Facing Outcome:** Business owners can update their storefronts instantly from their mobile devices. Customers globally experience sub-100ms load times. Zero technical configuration (no CDN settings, no cache purging) is required by the user.
  - **CUJ:** A business owner updates an item price while offline. Upon network connection, the app syncs the change to the cloud. The Operations Agent intelligently invalidates the specific edge caches. A customer on the other side of the world loads the updated product page instantly from the edge.
  - **Acceptance Criteria:**
    - Establish a mechanism for generating and distributing static storefront pages to an edge network.
    - Implement intelligent, targeted cache invalidation triggered by backend data mutations.
    - Ensure strict multi-tenant data isolation at the edge worker level.
    - Integrate the Ops AI Agent for cache management and the Marketing AI Agent for automated SEO metadata generation.
    - All storefront UI served must adhere strictly to the OHC Premium Design System (glassmorphism, clean typography).
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []