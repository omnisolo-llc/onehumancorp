issue_title: "[Architecture] Autonomous Multi-Channel Marketplace Syndication Engine"
issue_description: |
  # Title: Autonomous Multi-Channel Marketplace Syndication Engine

  ## Problem Statement
  Small business owners like Priya (Boutique) and Maya (Baker) want to reach customers where they already are—Instagram, TikTok, Google Shopping, and Etsy. However, keeping inventory and product details synced across multiple platforms requires either tedious manual data entry or expensive, complex third-party tools that are too difficult to configure. When Priya updates a dress's price or marks it out of stock, she often forgets to update her TikTok Shop, leading to overselling, frustrated customers, and potential account penalties. They need an invisible, zero-configuration engine that autonomously pushes their core OHC catalog out to all major sales channels and instantly reconciles inventory when a sale occurs anywhere.

  ## Research Report
  *   **Current Architecture Limits:** OHC currently acts as a standalone storefront. There is no native infrastructure to broadcast products to external marketplaces or ingest external orders for centralized fulfillment.
  *   **Competitor Analysis:**
      *   *Shopify:* Offers "Sales Channels," but setting them up requires jumping through API hoops, installing multiple apps, and often dealing with conflicting inventory logic. It is highly manual to configure feeds.
      *   *Wix:* Has basic multi-channel integrations but suffers from slow sync times and rigid mapping requirements.
      *   *Third-Party Syncs (ChannelAdvisor, Feedonomics):* Enterprise-grade pricing and extreme complexity. Completely inaccessible to our core personas.
  *   **Discovery:** OHC requires a unified syndication mesh. When a user creates a product, the platform must autonomously format and push the product data (images, descriptions, variants) to connected platforms via their respective APIs (Meta Graph API, TikTok Shop API, Google Merchant Center) in the background. Inventory must be managed in a unified ledger that treats the OHC storefront and external channels as equal consumers.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      OHC-CATALOG ||--o{ SYNDICATION-ENGINE : "Triggers Product Sync"
      SYNDICATION-ENGINE ||--o{ TIKTOK-SHOP-API : "Pushes Listings"
      SYNDICATION-ENGINE ||--o{ META-GRAPH-API : "Pushes to IG/FB"
      SYNDICATION-ENGINE ||--o{ GOOGLE-MERCHANT : "Updates Feed"
      EXTERNAL-CHANNELS ||--o{ EVENT-MESH : "Sends Order Webhooks"
      EVENT-MESH ||--o{ UNIFIED-LEDGER : "Deducts Inventory"
      UNIFIED-LEDGER ||--o{ OPERATIONS-AGENT : "Triggers Fulfillment"
  ```

  ### UI Wireframes & Mobile UX Flow (375px)
  *   **Global Channels View (OHC Mobile App - 375px):**
      *   **Dashboard Card:** A clean, glassmorphic card on the main dashboard reads "Sales Channels" with icons for Instagram, TikTok, and Google.
      *   **Connection Flow:** A simple toggle switch next to "TikTok Shop". Tapping it opens an OAuth flow. Once authenticated, a toast notification appears: "TikTok Shop connected. Your products are syncing." No feed mapping or technical configuration is shown.
      *   **Product Level Override:** On a specific product detail page, an "Advanced Settings" switch reveals toggles to include/exclude the item from specific channels.

  ### AI Agent Integration Points
  *   **Marketing Agent:** Automatically reformats product descriptions and resizes images to meet the specific requirements of each target platform (e.g., optimizing for TikTok SEO vs. Google Shopping keywords) before the syndication engine pushes the payload.
  *   **Customer Success Agent:** Intercepts order queries originating from external platforms (e.g., an Instagram DM about an order placed on the IG Shop) and handles them using the unified order data context.

  ### Key Design Decisions and Why
  *   **Invisible Formatting:** We hide feed mapping. The AI Marketing Agent transforms our internal data model into whatever format the destination API requires, removing the biggest point of friction for merchants.
  *   **Unified Inventory Ledger:** We treat all sales—whether from the OHC storefront or an external channel—as events that hit a single, authoritative inventory ledger to guarantee zero double-selling.
  *   **Event-Driven Sync:** Product updates fire events that background workers consume to push updates asynchronously, ensuring the main mobile UI remains extremely fast and responsive.

  ## Implementation Prompt
  Implement the Autonomous Multi-Channel Marketplace Syndication Engine. Build the core background workers that listen for product creation/update events on the OHC catalog and push formatted payloads to external APIs (start with Meta Graph API for Instagram/Facebook Shops). Integrate the AI Marketing Agent to autonomously tailor product titles and descriptions for the destination platform. Ensure strict multi-tenant isolation so syndication jobs only access data for the authorized tenant.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
