issue_title: "Integrate Shippo for Multi-Carrier Shipping & Label Printing"
issue_description: |
  # Problem Statement
  Priya, our boutique owner, currently handles online orders by manually entering customer addresses into USPS or FedEx websites, guessing package weights, and paying retail shipping rates. When Maya ships a batch of custom cookies across state lines, she has to copy-paste tracking numbers one by one into Instagram DMs to update her customers. Both users waste hours on logistics instead of growing their business. They need an automated, seamless way to get discounted shipping rates, print labels instantly, and automatically notify customers with tracking info—without ever leaving the OHC app.

  # Research Report
  - **Tool Evaluated**: Shippo (https://goshippo.com/)
  - **Why Shippo**:
    - **Ease of Use**: Exposes a clean REST API that abstracts away the complexity of integrating with 85+ individual carriers (USPS, UPS, FedEx, DHL, etc.).
    - **SMB Fit**: Offers a "Starter" plan with no monthly fee (pay-as-you-go, just $0.05 per label plus postage), which is perfect for micro-businesses like Maya and Priya. It also provides immediate access to heavily discounted USPS and UPS rates out-of-the-box, without requiring users to negotiate their own carrier accounts.
    - **Capabilities**: Supports address validation (crucial for reducing delivery errors), multi-carrier rate shopping, label generation (PDF/ZPL), and tracking webhooks.
  - **Competitor Analysis**:
    - *Shopify*: Has native "Shopify Shipping" powered by similar backend aggregators, offering strong discounts and 1-click label printing.
    - *Wix/Squarespace*: Rely heavily on third-party apps like Shippo or ShipStation.
    - *EasyPost*: A strong alternative, but Shippo tends to have a slightly more SMB-friendly UI and out-of-the-box rate discounts without requiring developer-heavy configuration for the end user.
  - **SaaS Viability**: Shippo operates as a robust cloud SaaS. Its API handles high concurrency and provides webhooks for async tracking updates, aligning well with OHC's cloud multi-tenant architecture.

  # Design Doc
  - **User Experience**:
    - When a customer places an order requiring physical shipping, the OHC "Operations" AI Agent detects it and adds it to the "To Ship" queue.
    - The business owner (Maya/Priya) taps the order, enters or confirms package dimensions/weight (or selects a saved preset like "Standard Cookie Box"), and taps "Get Shipping Rates".
    - The app calls the OHC backend, which in turn calls the Shippo API to fetch real-time, discounted rates from USPS/UPS.
    - The user selects a rate and taps "Buy Label".
    - The backend purchases the label via Shippo, deducts the cost from the user's OHC balance or charges their card, and returns a printable PDF label.
    - Simultaneously, the OHC "Customer Success" AI Agent drafts and sends an email/SMS to the customer with the tracking link (powered by Shippo webhooks).
  - **Integration Architecture**:
    - **Backend (Go)**: A new `ShippingService` module.
      - `GetRates(orderID, packageDetails)`: Calls Shippo `/shipments/` endpoint to create a shipment and retrieve rates.
      - `PurchaseLabel(rateID)`: Calls Shippo `/transactions/` to buy the label.
      - `WebhookHandler`: Listens for Shippo tracking updates and updates the OHC order status in PostgreSQL.
    - **Frontend (Flutter)**: Order details screen updated with a "Fulfillment" card showing real-time rates, a "Print Label" button, and tracking status.

  # Implementation Prompt
  Create a unified shipping integration that allows business owners to seamlessly purchase and print discounted shipping labels directly from an order details page.
  1. Add a "Fulfillment" section to the Order Details UI where users can input package weight/dimensions.
  2. Implement backend logic to connect to a shipping aggregator (like Shippo) to fetch live shipping rates based on the order's destination address.
  3. Allow the user to select a rate and purchase a shipping label. Display the resulting label as a downloadable/printable PDF.
  4. Automatically transition the order status to "Shipped" and store the tracking number when a label is purchased.
  5. *Acceptance Criteria*: A non-technical user must be able to view an unfulfilled order, get a list of shipping rates, buy a label, and see the tracking number—all with less than 3 taps.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
