issue_title: "Scout: Tool Integration Research - Shippo"
issue_description: |
  # Shippo Integration: Streamlined Multi-Carrier Shipping for Small Businesses

  ## Problem Statement
  Small business owners selling physical goods (e.g., Carlos, who runs a local artisanal coffee roasting business) face significant friction when fulfilling orders. They currently have to manually copy order details into separate carrier portals (USPS, UPS, FedEx), guess at the best rates, and manually update customers with tracking numbers. This is time-consuming, error-prone, and distracts from core business activities. They need a simple, unified way to compare shipping rates, purchase labels, and generate tracking links directly within their existing workflows.

  ## Research Report
  **Tool Analyzed:** Shippo (goshippo.com)
  **Value for SMBs:**
  Shippo aggregates shipping rates across dozens of carriers globally (USPS, UPS, FedEx, DHL, etc.) and provides deeply discounted rates, especially for USPS and UPS. It's designed specifically for e-commerce and small business fulfillment.

  **Ease of Use:**
  From an end-user perspective, an integration would mean simply clicking "Fulfill Order" next to a sale, seeing 2-3 rate options (e.g., "Standard - $4.50", "Express - $8.00"), and clicking "Buy Label" to instantly generate a printable PDF and a tracking link. It removes the need for the user to understand complex carrier APIs or negotiate their own rates.

  **Pricing:**
  Shippo has a very SMB-friendly pricing model. They offer a "Starter" tier with no monthly fee; users only pay for the cost of postage plus a small per-label fee (currently around $0.05 per label) if using connected carrier accounts, and often $0 for labels bought through Shippo's default discounted carrier accounts. This makes it highly viable for low-volume sellers.

  **Reputation & Technical Viability:**
  Shippo is a well-established player with robust, well-documented REST APIs and reliable webhooks.
  - **Cloud (Multi-tenant):** Shippo supports OAuth 2.0, allowing OHC to act as an integrated platform where users can connect their own Shippo accounts securely.
  - **Standalone:** A standalone user could easily provide an API token from their Shippo dashboard.

  **Competitors Evaluated:**
  - **EasyPost:** Excellent API, but often geared slightly more towards enterprise/developers. Shippo's UI and default carrier discounts are slightly more SMB-friendly out-of-the-box.
  - **ShipStation:** Excellent web interface for users, but their API is more meant for importing orders *into* ShipStation rather than building a shipping experience *inside* another app (like OHC).

  ## Design Doc
  **Trigger:**
  A user marks an order as "Ready to Ship" within the OHC platform, or a webhook is received indicating a new paid order.

  **Actions:**
  1. **Rate Fetching:** OHC sends the package dimensions, weight (if known or using a default), origin, and destination to the Shippo API to fetch available shipping rates.
  2. **Label Purchase:** The user selects a rate in the UI. OHC calls the Shippo API to purchase the label for that specific rate.
  3. **Artifact Generation:** Shippo returns a URL to a printable PDF label and a tracking number.
  4. **Customer Update:** OHC automatically saves the tracking number to the order record and triggers a notification (e.g., email/SMS) to the customer.

  **User Interface Impact:**
  The user will see a new "Shipping" section on order detail pages. They will have a setup flow to either link an existing Shippo account or create a new one to access default rates. The actual label purchase happens in 2-3 clicks without leaving OHC.

  ## Implementation Prompt
  Implement an integration with the Shippo API to enable in-app shipping label purchases.
  **Acceptance Criteria:**
  - A user can connect their Shippo account via API key (Standalone) or OAuth (Cloud).
  - When viewing an order with physical items, the user can request shipping rates based on standard package sizes.
  - The user is presented with a list of available shipping rates (Carrier, Service Level, Cost).
  - The user can purchase a label by selecting a rate.
  - Upon purchase, the system stores the Tracking Number and the URL to the printable label PDF.
  - The order status automatically updates to "Shipped".

  ## Priority
  P1 (High) - Fulfilling physical goods is a core operational need for many SMBs, and manual shipping is a major pain point.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []