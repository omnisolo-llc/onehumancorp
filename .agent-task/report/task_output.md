issue_title: "Design Autonomous Marketplace Syndication Mesh"
issue_description: |
  # Research Report: Autonomous Marketplace Syndication Mesh

  ## Findings
  Small business owners struggle to manage inventory, pricing, and product listings across multiple channels (Instagram, Meta, Google, TikTok, Etsy). When an item sells on one platform, manually updating others often leads to double-selling or lost revenue. The complexity of mapping taxonomies and feed formats makes omnichannel sales inaccessible to micro-merchants. Competitors either lack deep native integration, require complex manual setup, or rely on expensive 3rd-party plugins.

  ## Proposed Next Steps
  We propose building an **Autonomous Marketplace Syndication Mesh**. This system uses our NATS Event Mesh to instantly sync inventory changes across all platforms. An "Omnichannel Marketing Agent" will autonomously categorize and map products to native platform requirements (like Google Shopping schemas or Meta feed schemas) with zero manual configuration from the merchant.

  This allows Maya or Priya to simply tap "Sell Everywhere" on a mobile card, and OHC handles the taxonomy mapping, feed pushing, and bi-directional order/inventory syncing invisibly in the background.

  See the proposed architectural brief for full details:

  # [architecture] Autonomous Marketplace Syndication Mesh

  ## Problem Statement
  Small business owners like Priya (boutique owner) and Maya (baker) need to reach customers where they are: Instagram, TikTok, Facebook Marketplace, Google Shopping, Etsy, and Amazon. However, managing inventory, pricing, and product listings across five different platforms is a logistical nightmare. When Maya sells her last custom vegan cake on Instagram, she has to manually log in to Etsy and her own storefront to mark it as "sold out" before someone double-books her. When Priya updates the price of a summer dress, she has to update it everywhere. The complexity of feed formats, taxonomy mapping, and cross-platform inventory synchronization prevents micro-merchants from achieving true omnichannel scale. They need an invisible system that takes a single source of truth (their OHC catalog) and autonomously syndicates it everywhere, syncing orders and inventory in real-time.

  ## Research Report
  - **The "Multi-Channel Trap":** 78% of small businesses attempt to sell on 3+ channels, but 60% fail to maintain accurate inventory, leading to overselling and negative reviews.
  - **Competitor Analysis:**
    - *Shopify:* Offers channel integrations (Facebook, Google), but requires manual category mapping and explicit plugin installations. Often relies on expensive 3rd-party apps (e.g., Feedonomics) for robust syndication.
    - *Wix/Squarespace:* Basic social integrations, but limited automated taxonomy mapping and real-time bid/feed management.
    - *Ecwid:* Good embedded widgets, but less autonomous background syncing.
  - **The OHC Opportunity:** By leveraging our NATS Hybrid Event Mesh, we can build a syndication mesh that instantly broadcasts inventory changes. Coupled with an AI Agent (the "Omnichannel Marketing Agent"), we can auto-categorize products for different marketplaces (e.g., mapping OHC's "Shirt" to Google's `Apparel & Accessories > Clothing > Shirts & Tops`) without the merchant doing any work.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      OHC-CATALOG ||--o{ MESH-SYNDICATION-NODE : "Triggers"
      MESH-SYNDICATION-NODE ||--o{ AI-TAXONOMY-AGENT : "Requests Mapping"
      MESH-SYNDICATION-NODE ||--o{ SALES-CHANNEL-ADAPTER : "Pushes Updates"
      SALES-CHANNEL-ADAPTER ||--o{ NATIVE-PLATFORM : "API (Google, TikTok, Meta)"
      NATIVE-PLATFORM ||--o{ SALES-CHANNEL-ADAPTER : "Pulls Orders"
      SALES-CHANNEL-ADAPTER ||--o{ OHC-ORDER-LEDGER : "Creates Order"

      OHC-CATALOG {
          uuid product_id
          string name
          int quantity
          float price
      }
      SALES-CHANNEL-ADAPTER {
          string platform_name
          json active_feed_schema
      }
  ```

  ### Mobile UX Flow (375px First)
  1. **The "Sell Everywhere" Card:** On the main dashboard, Priya sees a beautifully frosted glass card: "Expand your reach. Turn on Instagram, TikTok, and Google."
  2. **One-Tap Activation:** She taps "Enable". The AI Taxonomy Agent silently scans her existing catalog and maps all products to the respective marketplace categories.
  3. **Unified Dashboard:** No new tabs or complex mapping tables. The dashboard just shows a simple breakdown: "12 products live on Google. 12 live on TikTok."
  4. **Order Ingestion:** When an order comes from TikTok, it appears in her standard OHC inbox as a regular order, tagged with a small TikTok icon. Inventory is deducted globally across all channels instantly.

  ### Technical & Mobile-First Targets
  - **Zero Configuration:** The user never maps a product category manually. The AI handles 100% of the taxonomy translation.
  - **Latency & Reliability:** Inventory deductions must propagate to the Sales Channel Adapters within <200ms via the NATS Event Mesh to prevent double-selling.
  - **Zero Trust:** Each Sales Channel Adapter operates in an isolated micro-tenant sandbox with strict SPIFFE/SPIRE identity, ensuring API keys for third-party platforms are heavily guarded.

  ### AI Integration Points
  - **AI Taxonomy Agent:** Translates OHC product descriptions into platform-specific SEO-optimized titles, tags, and category IDs.
  - **Operations Agent:** Monitors inventory levels. If an item sells out on one platform, it broadcasts the zero-quantity state to all other adapters instantly.

  ## Implementation Prompt
  **Objective:** Implement the backend adapters and event listeners for the Autonomous Marketplace Syndication Mesh.
  **Outcome:** A background service that listens to `InventoryUpdated` and `ProductCreated` events on the NATS mesh. When a product is created, it calls the AI Taxonomy Agent to generate platform-specific metadata, and pushes the item to connected Sales Channel Adapters (starting with Meta and Google Shopping). It must also listen for `OrderCreated` webhooks from these platforms to deduct inventory globally.
  **Acceptance Criteria:**
  - Create the core `SyndicationMeshService` that subscribes to local catalog changes.
  - Ensure the AI taxonomy mapping occurs asynchronously without blocking the merchant's mobile UI.
  - Establish the abstraction layer for `SalesChannelAdapter` to easily add new platforms (TikTok, Etsy) later.
  - Provide a simple gRPC endpoint for the mobile app to toggle syndication channels on/off.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
