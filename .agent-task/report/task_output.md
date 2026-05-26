issue_title: "Edge-Rendered Dynamic Storefronts with Offline Support for Low-End Devices"
issue_description: |
  ## Problem Statement
  OneHumanCorp (OHC) is designed for non-technical small business owners like Fatima (a food cart owner taking pre-orders) and Maya (a baker selling custom cakes). Many of these users and their customers rely on low-end mobile devices (e.g., budget Android phones) and frequently operate in areas with poor or intermittent cellular connectivity (e.g., food markets, busy streets, underground transit).

  Currently, if a customer tries to load an OHC storefront (e.g., to view a menu or place a pre-order) on a slow connection, the experience can be sluggish, leading to drop-offs and lost revenue. Furthermore, if Fatima is managing her sold-out toggles and loses connection, the state might not sync immediately. OHC needs an architecture that guarantees instant loading for storefronts and offline capability for critical business operations, ensuring that commerce never stops, regardless of network conditions.

  ## Research Report
  ### Competitive Analysis
  - **Shopify:** Utilizes a globally distributed edge network (Cloudflare/Fastly) to cache static assets and HTML pages, but dynamic inventory checks still require round trips. They offer a robust POS system with offline mode, but the online storefronts struggle in true offline scenarios.
  - **Wix/Squarespace:** Heavy initial payload for site builders. Poor performance on 3G connections. Minimal offline capability for either the merchant or the customer.
  - **Square:** Excellent offline POS capabilities (queuing transactions), but their online ordering pages are standard web apps without deep edge-rendering integration.

  ### The OHC Opportunity
  To provide a genuinely superior experience for the "Maya" and "Fatima" personas, OHC must adopt a true **Edge-Rendered, Offline-First Architecture**. This involves pushing the rendering of the storefront to the absolute edge (closest to the user) and utilizing advanced Progressive Web App (PWA) techniques with local state synchronization (e.g., SQLite in browser/CRDTs) for the merchant dashboard.

  ## Design Doc

  ### High-Level Architecture
  1.  **Edge Rendering Engine:** Storefronts are pre-rendered and cached at edge nodes globally. When Maya updates her cake catalog, an AI Agent instantly triggers a background job to invalidate and re-render the edge cache for her specific storefront.
  2.  **Local State & Synchronization (CRDTs):** The merchant app (used by Fatima) relies on a local, offline-first database. Changes (like toggling an item "Sold Out") are recorded locally as Conflict-Free Replicated Data Types (CRDTs) and queued for background synchronization with the central KAIROS orchestrator when connectivity is restored.
  3.  **Optimistic UI:** The customer-facing storefront utilizes optimistic UI updates for adding items to the cart. Critical actions like checkout still require a connection, but the catalog browsing experience is fully available offline after the initial load.
  4.  **AI Department Coordination:**
      - **Operations Agent:** Monitors the background sync queue. If a conflict arises (e.g., an item sold out while offline, but a customer somehow placed an order), it resolves the conflict based on predefined merchant policies (e.g., automatically issue a refund and send an apology SMS).
      - **Marketing Agent:** Can push updated localized promotions to the edge cache proactively.

  ### Mermaid Architecture Diagram
  ```mermaid
  graph TD
      A[Customer (Low-End Mobile)] -->|Browses Catalog| B(Global Edge Node - Cached Storefront)
      B -->|Edge Render| A
      A -->|Places Order| C{Connection Active?}
      C -- Yes --> D[Gateway API]
      C -- No --> E[Local Service Worker Queue]
      E -->|Background Sync| D
      D --> F[KAIROS Orchestrator]

      G[Merchant (Fatima/Maya)] -->|Toggles Sold Out| H(Local DB / CRDTs)
      H -->|Optimistic UI Update| G
      H -->|Background Sync| D
      F -->|Invalidates Cache| B
      F -->|Triggers AI| I[Operations Agent - Conflict Resolution]
  ```

  ### Mobile UX Flow (375px First)
  - **Storefront (Customer):** Instantly loads a skeleton UI, followed immediately by cached images and text. A subtle, non-intrusive banner indicates "Offline Mode - Browse Only" if connection is lost.
  - **Dashboard (Merchant):** When toggling inventory, the switch turns green instantly (Optimistic UI). If offline, a small, elegant cloud icon with a slash appears in the top navigation bar, indicating "Changes saved locally."

  ### Security & Zero Trust
  - **Multi-Tenant Isolation:** Edge caches are strictly partitioned by Tenant ID. A compromised edge node cannot access data belonging to another merchant.
  - **Identity:** Sync requests from the merchant app require short-lived, cryptographically signed tokens (SPIFFE/SPIRE concepts adapted for mobile clients) to prevent replay attacks on the background queue.

  ## Implementation Prompt
  Implement the Edge-Rendered Dynamic Storefront and Offline-First Merchant Sync architecture.

  **Outcome:**
  1. Customers must be able to load any merchant's storefront in under 1.5 seconds on a simulated 3G connection. Once loaded, browsing the catalog must work completely offline.
  2. Merchants must be able to update inventory states (e.g., mark items sold out) while entirely disconnected from the internet. The UI must reflect these changes instantly. Upon reconnecting, these changes must automatically sync to the backend without user intervention.

  **Acceptance Criteria:**
  - Introduce an edge caching strategy that serves pre-rendered HTML for storefronts.
  - Implement a local-first data store for the merchant dashboard that queues mutations when offline.
  - Establish the background sync mechanism to reconcile local mutations with the central database.
  - Ensure the Operations AI Agent can intercept and resolve any inventory conflicts arising from offline syncs.
  - All screens must strictly adhere to the OHC design system (glassmorphism, mobile-first 375px layout).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
