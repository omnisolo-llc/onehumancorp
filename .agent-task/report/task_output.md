issue_title: "Implement Shippo API for Automated Multi-Carrier Shipping & Label Generation"
issue_description: |
  # 📦 Shipping & Logistics Integration: Shippo

  ## Problem Statement
  **The Problem:** Small business owners selling physical products (like Maya the Baker or Priya the Boutique Owner) face a massive operational bottleneck when fulfilling orders. Currently, when an order is placed on an OHC storefront, the owner must manually transcribe the customer's address into a separate shipping provider, manually calculate shipping rates to charge the customer, purchase the label, and manually update the order in OHC with the tracking number. This manual data entry is error-prone, time-consuming, and confusing for non-technical users.

  **The Need:** Non-technical owners need an invisible "Operations Agent" that automatically calculates real-time shipping rates at checkout, auto-generates shipping labels when an order is confirmed, and automatically sends tracking updates to the customer, all without leaving the OHC platform.

  ## Research Report
  **Market Landscape:**
  Competitors like Shopify and Wix offer built-in shipping solutions (Shopify Shipping) or robust app ecosystems integrating with ShipStation, Shippo, and EasyPost.
  - **ShipStation:** Extremely powerful but has a complex, dated UI aimed at larger warehouses. High learning curve.
  - **EasyPost:** Developer-first API, great technical reliability, but lacks a pre-built user-friendly interface for merchant disputes or simple manual overrides.
  - **Shippo:** Offers a modern, developer-friendly API (REST) combined with a highly intuitive web dashboard for edge cases. It abstracts away carrier-specific complexities and provides heavily discounted rates for USPS, UPS, and DHL out-of-the-box.

  **Why Shippo for OHC?**
  Shippo aligns perfectly with our "Radical Simplicity" value.
  - **Ease of Use for Merchants:** OHC can handle 95% of shipping tasks automatically via API. For the 5% of edge cases (e.g., complex international returns, carrier disputes), the merchant can be securely deep-linked to Shippo's simple dashboard.
  - **Pricing:** Shippo offers a "Pay-as-you-go" tier (Starter plan is free, pay only $0.05 per label plus postage). This fits perfectly with OHC's commitment to a useful free tier for new, low-volume businesses.
  - **Capabilities:** Supports real-time rate calculation (critical for checkout), automatic label generation (PDF/PNG), tracking webhooks, and automatic customs forms for international orders.

  ## Design Doc
  **Integration Trigger:**
  1. **Checkout Flow:** When a customer enters their shipping address in the OHC storefront, the platform makes a real-time request to the Shippo API to fetch shipping rates and displays them.
  2. **Order Fulfillment:** When a business owner clicks "Fulfill Order" in their OHC dashboard, OHC requests a shipping label from Shippo.

  **System Actions (Invisible to User):**
  - **Operations Agent (`OperationsAgent`):** Intercepts the `OrderConfirmed` event. It packages the origin address, destination address, and package dimensions/weight, then calls the Shippo API to purchase a label.
  - **Document Storage:** The returned label (PDF) is downloaded, stored in OHC's GCS/MinIO bucket, and linked to the order record in the OHC database.
  - **Customer Success Agent (`CustomerSuccessAgent`):** Listens for Shippo's tracking webhooks (e.g., `TRANSIT`, `DELIVERED`) and automatically drafts/sends email or SMS updates to the customer.

  **User Experience (Business Owner):**
  - The owner sees a clean "Fulfill Order" button on the order details page.
  - Clicking it simply displays a generated shipping label ready to print, and the order status automatically updates to "Shipped" with a tracking number attached. No carrier accounts needed, no API keys to configure.

  ## Implementation Prompt
  Implement the Shippo integration to automate shipping rate calculation and label generation for physical product orders.

  **Acceptance Criteria:**
  1. **Configuration:** Provide a simple UI in the OHC integrations dashboard for the owner to enable shipping (e.g., a simple toggle and default box size settings). Do not expose API keys or complex carrier settings.
  2. **Checkout Integration:** The storefront checkout must automatically calculate and display shipping rates based on the customer's address and cart contents.
  3. **Label Generation:** The order details view in the OHC dashboard must include a one-click action to purchase and generate a printable shipping label.
  4. **Tracking Updates:** Implement a webhook listener to receive tracking updates from Shippo and automatically update the order status in OHC, notifying the customer.
  5. **UI Polish:** All new UI elements must adhere to the OHC Premium Token library (Glassmorphism, 20px blur).
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []