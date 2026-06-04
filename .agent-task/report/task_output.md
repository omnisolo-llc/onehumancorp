issue_title: "Integrate Square POS API for Real-Time Offline-to-Online Inventory Sync"
issue_description: |
  ## Problem Statement
  Business owners like Priya (The Boutique Owner) run both a physical storefront and an online store. They face the critical problem of overselling: if a customer buys a dress in-store using the physical cash register, the online store inventory must update instantly to prevent another customer from buying the same out-of-stock item. Manually syncing inventory at the end of the day is error-prone, time-consuming, and leads to angry online customers. They need a zero-touch solution where selling an item anywhere updates the stock everywhere.

  ## Research Report
  Square is the dominant point-of-sale (POS) system for small brick-and-mortar retail and food businesses (like boutique shops and food carts).
  *   **Capabilities:** The Square API provides real-time webhooks for inventory changes (`inventory.count.updated`) and catalog updates. It allows full bidirectional sync.
  *   **User Value:** For Priya, she connects her Square account once. OHC automatically imports her entire product catalog (names, prices, photos, stock levels). When she sells a dress in her physical store via her Square register, Square fires a webhook to OHC, which instantly decrements the online stock. If an online customer buys a dress, OHC updates the Square inventory so the register shows the correct stock.
  *   **Pricing:** Square API is free to use (standard processing rates apply to transactions).
  *   **Modes Supported:** Cloud (OAuth via Square App Marketplace) and Standalone (Personal Access Token).
  *   **Non-technical User Experience:** A single "Connect Square" button. A prompt asks: "Do you want to import your products from Square?" and "Do you want to keep inventory synced automatically?". No API keys, no webhooks to configure.

  ## Design Doc
  1.  **Authentication:** User connects their Square account via OAuth from the OHC Integrations panel. OHC stores the Square merchant access token.
  2.  **Initial Import (The Operations Agent):** OHC fetches the Square Catalog and Inventory APIs. Products are matched by SKU or name. Missing products are created in OHC.
  3.  **Webhook Listener:** OHC registers a webhook endpoint with Square for `inventory.count.updated` and `catalog.version.updated` events.
  4.  **Real-time Sync (Offline to Online):** When an in-store sale occurs, Square sends a webhook. The OHC "Operations" agent receives it, identifies the product, and updates the OHC database stock count. If stock hits 0, the online product is marked "Sold Out".
  5.  **Real-time Sync (Online to Offline):** When an online order is placed via OHC, the Operations agent calls the Square Inventory API to decrement the stock count in the physical store.

  ## Implementation Prompt
  Add a "Square POS" integration in the OHC platform. Implement the OAuth flow to connect a user's Square account. Build an initial synchronization tool that imports the user's Square catalog (products, variants, prices, and stock levels) into OHC. Set up a webhook listener to receive real-time inventory updates from Square so that when an item is sold in the physical store, the OHC online storefront immediately reflects the new stock level. Conversely, when an item is sold online through OHC, update the Square inventory via their API. Ensure the UI clearly shows the user that their inventory is "Synced with Square".
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
