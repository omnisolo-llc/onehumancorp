issue_title: "Integration: Shippo API for Multi-Carrier Shipping & Label Generation"
issue_description: |
  **Problem Statement**
  For OHC's physical product sellers (like Priya the Boutique Owner or Maya the Home Baker shipping goods), order fulfillment is a huge headache. They currently have to manually copy order details, go to the post office or carrier websites (USPS, FedEx, UPS), buy shipping labels at retail rates, print them, and then manually message customers with tracking numbers. This is slow, error-prone, and doesn't scale. Non-technical users need a seamless, invisible way to get the best shipping rates and print labels directly from their phone, with automatic tracking updates sent to the customer.

  **Research Report: Shippo API**
  *   **Capabilities**: Shippo aggregates 85+ global carriers (USPS, UPS, FedEx, DHL, etc.) behind a single REST API. It handles address validation, real-time rate shopping, label generation, and tracking webhooks.
  *   **Pricing**: Pay-as-you-go model ($0.05 per label) or free for USPS only. They also provide deep discounted carrier rates (up to 90% off USPS) which directly benefits our small business users, adding instant value to the OHC platform.
  *   **Ease of Use for Non-Technical Users**: The user simply clicks "Fulfill Order" in OHC, sees the cheapest shipping option automatically selected by AI, and clicks "Print Label". The label is generated as a PDF or image, ready to print on a standard printer or thermal label printer. The tracking number is automatically emailed to the customer by the Operations agent. No carrier accounts needed for the user (OHC acts as the master account or uses Shippo's default carrier accounts).
  *   **Cloud & Standalone Viability**: Shippo is a reliable, multi-tenant capable SaaS. We can integrate it securely via our Backend using a master API key or OAuth flow for users who already have carrier accounts.
  *   **Competitor Comparison**: Shopify has built-in Shopify Shipping (powered by Shippo/similar). Wix uses third-party apps like ShipStation or Shippo. OHC can differentiate by making it completely invisible and AI-driven (e.g., "The Manager" agent automatically buys the cheapest label when an item is packed).

  **Design Doc**
  *   **Trigger**: An order status changes to "Ready to Ship" or a user clicks "Generate Shipping Label" on an order details screen.
  *   **Action**:
      1.  Backend calls Shippo API to validate the customer's address.
      2.  Backend requests rates based on the stored package dimensions/weight of the products.
      3.  The Operations Agent ("The Manager") automatically selects the most cost-effective option that meets the delivery timeframe.
      4.  Backend purchases the label via Shippo API and retrieves the PDF/PNG label URL and tracking number.
      5.  Customer Success Agent ("The Ambassador") drafts and sends the tracking email/SMS to the customer.
  *   **User Interface**: A simple "Print Label" button appears on the order. No complex carrier settings unless the user wants to dig in.

  **Implementation Prompt**
  Implement the backend integration with Shippo to generate shipping labels and tracking links for orders.
  *   **Acceptance Criteria 1**: A non-technical user can generate a shipping label for an order directly from the OHC mobile or web app with a single click.
  *   **Acceptance Criteria 2**: The system automatically retrieves the cheapest rate for the package dimensions and weight.
  *   **Acceptance Criteria 3**: The user can view and print the generated label (PDF/image).
  *   **Acceptance Criteria 4**: The tracking number is saved to the order, and the order status is updated to "Shipped".

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
