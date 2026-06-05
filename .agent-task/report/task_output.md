issue_title: "Integrate Shippo API for 1-Click Shipping Label Generation & Rate Calculation"
issue_description: |
  ## Problem Statement
  Small business owners selling physical goods (like Maya the Home Baker or Priya the Boutique Owner) currently have to manually fulfill orders. When an order comes in, they must manually copy the customer's address, navigate to a separate carrier site (USPS, UPS) or go to the post office, purchase a label, and then manually email the tracking number to the customer. This disconnected process is slow, prone to data entry errors, and lacks professional tracking transparency for the end customer.

  ## Research Report
  ### Market Need
  Competitors like Shopify (Shopify Shipping), Wix, and Squarespace all provide built-in shipping label generation. SMBs view integrated shipping as a table-stakes feature for e-commerce. A seamless shipping experience saves hours per week and unlocks discounted carrier rates.

  ### Tool Evaluated: Shippo API
  - **Capabilities:** Shippo aggregates dozens of carriers (USPS, UPS, FedEx, DHL) into a single REST API. It handles address validation, real-time rate shopping, label generation (PDF/PNG), and tracking webhooks.
  - **User-First Value:** The business owner never sees "Shippo" or "API". They simply see a "Buy Label" button on their order screen, get discounted rates, and can print the label directly from their phone or computer.
  - **Pricing Viability:** Shippo offers a pay-as-you-go tier ($0.05 per label plus postage) with no monthly fees, making it perfect for our free-tier and low-volume users. It scales to flat monthly fees for higher volume.
  - **Cloud/Standalone Support:** Fully supports multi-tenant cloud usage via API keys and webhooks.

  ## Design Doc
  **Department:** Operations ("The Manager") & Customer Success ("The Ambassador")

  **Flow:**
  1. **Trigger:** An order containing physical products is placed and paid for.
  2. **Rate Shopping:** The Operations Agent silently calls Shippo with the store's origin address, customer's destination address, and product weight/dimensions to fetch live rates.
  3. **UI Presentation:** On the Order Details screen, the owner sees a "Fulfillment" section with the cheapest/fastest shipping options.
  4. **Action:** The owner taps "Buy Label". The Operations Agent hits Shippo to purchase the label and stores the Tracking Number and Label PDF URL.
  5. **Follow-up:** The Customer Success Agent automatically drafts and sends a shipping confirmation email/SMS to the customer containing the tracking link.

  ## Implementation Prompt
  Implement an end-to-end shipping fulfillment flow for physical orders using the Shippo API.
  - **User Facing Outcome:** Add a "Buy Shipping Label" flow to the order details screen. The user should be able to select a package size, see a list of carrier rates, purchase the label, and download it as a PDF directly from the OHC interface.
  - **Acceptance Criteria:**
    - Seamlessly validate the customer's destination address before rate calculation.
    - Display real-time rates from at least USPS and UPS.
    - Generate a printable shipping label.
    - Automatically update the order status to "Shipped" and surface the tracking link.
    - Ensure the UI remains simple, mobile-friendly (375px), and completely hides the underlying API mechanics.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
