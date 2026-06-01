issue_title: "Tool Integration Research: Printful for Automated Print-on-Demand"
issue_description: |
  # 🔍 Scout: Tool Integration Research - Printful

  ## Problem Statement
  Creative professionals, artists, and influencers (like Maya, looking to sell merch, or a graphic designer) want to monetize their audience by selling physical products (t-shirts, mugs, posters). However, they lack the capital to buy inventory upfront, the space to store it, and the time to manage shipping and fulfillment. Existing e-commerce platforms allow selling products but require the owner to handle the logistics. They need a zero-inventory, fully automated way to design, sell, and fulfill custom merchandise seamlessly from their OHC storefront.

  ## Research Report
  *   **Tool:** Printful API
  *   **Market Position:** The industry leader in white-label print-on-demand (POD) dropshipping. Highly trusted by creators and small businesses.
  *   **Capabilities & Limits:**
      *   **Product Creation:** API allows creating new products by applying user designs to hundreds of base items.
      *   **Real-time Shipping Rates:** Calculates accurate shipping costs at checkout based on the fulfillment center location and customer destination.
      *   **Automated Fulfillment:** When an order is placed on OHC, it's automatically pushed to Printful for printing, packing, and shipping directly to the customer under the merchant's brand.
      *   **API Quality:** Comprehensive, mature REST API with excellent documentation and reliable webhooks for status updates.
  *   **SaaS Viability & Pricing:**
      *   **Pricing Model:** No monthly fees. The merchant only pays for the base product and shipping when an order is placed. The merchant sets the retail price to determine their profit margin.
      *   **Modes:** Suitable for Cloud (multi-tenant) via OAuth integration where OHC connects the tenant's Printful account, or Standalone via API key.
  *   **Reputation & Ease of Use:** Very high reputation for product quality and automated fulfillment. Connecting it to a platform abstracts all the difficult logistics away from the user.

  ## Design Doc
  *   **Trigger:** The integration operates in two phases: Product Creation and Order Fulfillment.
      *   *Creation:* User uploads a design in OHC. The "Operations" agent uses the Printful API to generate mockups and create a syncable product in the OHC catalog.
      *   *Fulfillment:* A customer places an order on the OHC storefront.
  *   **Action:** OHC automatically transmits the order details and shipping information to Printful.
  *   **User Experience (OHC Dashboard):**
      *   A "Merchandise" or "Print-on-Demand" section in the Operations department.
      *   User clicks "Connect Printful" (OAuth flow).
      *   User can browse a simplified catalog of blank items (shirts, mugs), upload their logo/art, and click "Generate Product". The AI creates the product listing, mockups, and descriptions automatically.
      *   When orders come in, the merchant sees them in OHC as "Processing by Printful" and later "Shipped" via webhook updates, with tracking info automatically forwarded to the customer.

  ## Implementation Prompt
  Implement an integration with the Printful API to enable automated print-on-demand dropshipping for OHC merchants.
  *   **Acceptance Criteria 1 (Connection):** A merchant can securely connect their Printful account via OAuth.
  *   **Acceptance Criteria 2 (Product Generation):** Provide a simplified flow where a merchant can select a base product, upload a design image, and the system uses the Printful API to generate product mockups and create a new item in the OHC store catalog.
  *   **Acceptance Criteria 3 (Order Routing):** When a customer purchases a Printful-linked product, the order details (including shipping address) must be automatically sent to the Printful API for fulfillment.
  *   **Acceptance Criteria 4 (Status Sync):** The system must listen for Printful webhooks to automatically update the OHC order status (e.g., to "Shipped") and capture the tracking number.

  ## Priority
  P1 (High) - Unlocks a massive new revenue stream (merchandise) for creative and service-based personas with zero inventory risk.

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
