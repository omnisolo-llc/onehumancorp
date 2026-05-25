issue_title: "Integrate Shippo for Multi-Carrier Shipping and Automated Label Generation"
issue_description: |
  # [Shipping] Shippo Integration

  ## Problem Statement
  Small business owners selling physical goods struggle with the complexity and cost of managing shipping. They often have to manually copy order details into carrier websites, figure out which carrier is cheapest, and then manually copy tracking numbers back to the customer. This process is highly error-prone, extremely time-consuming, and limits their ability to scale. A non-technical small business owner (like Maya or Leo) needs an automated way to compare shipping rates across multiple carriers, purchase and print shipping labels directly from their dashboard, and automatically notify customers with tracking information, all without needing a separate standalone tool or deep logistics knowledge.

  ## Research Report
  - **Market Need**: E-commerce enablement is incomplete without robust shipping logistics. Competitors like Shopify and Wix have deep, native integrations with shipping aggregators. Small businesses repeatedly list shipping costs and label generation as major pain points on platforms like r/smallbusiness and ecommerce forums.
  - **Tool Evaluated**: **Shippo**
  - **Capabilities & Limits**:
    - Shippo provides an API that abstracts away the complexities of integrating with USPS, UPS, FedEx, DHL, and over 80 other regional and global carriers.
    - Features include multi-carrier rate comparison, label generation (PDF, ZPL), tracking webhooks, and returns management.
    - Good developer documentation and reliable API SLA.
  - **SaaS Viability**:
    - Pricing is friendly for SMBs: they offer a "Pay As You Go" free tier where users only pay $0.05 per label (plus postage). This allows OHC to offer a powerful feature with zero upfront cost.
    - Shippo supports OAuth for multi-tenant (Cloud) platforms, meaning OHC users can easily connect their own Shippo accounts or OHC can act as a platform partner. For Standalone environments, owners can provide their own API key.
  - **Ease of Use**: The value proposition is extremely high. Non-technical users will just see "Buy Shipping Label" on their order management page, select a package size, choose the cheapest rate, and click print.

  ## Design Doc
  - **Triggers**:
    - User views a "Fulfilled/Unfulfilled Order" in the OHC dashboard.
    - User clicks a new "Generate Shipping Label" action on the order details page.
  - **Actions**:
    - **Rate Fetching**: OHC sends order weight, dimensions, and destination address to Shippo. Shippo returns available carrier rates.
    - **Label Purchase**: User selects a rate. OHC requests label purchase from Shippo.
    - **Fulfillment Update**: Shippo returns the tracking number and a link to the printable label. OHC updates the order status to "Shipped" and stores the tracking number.
    - **Customer Notification**: OHC triggers an email/SMS to the customer with the tracking number.
    - **Tracking Updates**: OHC listens for Shippo webhooks on tracking events to update internal order status or notify customers.
  - **User View**: A clean modal or integrated panel on the order screen showing rate comparisons. Once purchased, a button to "Print Label" appears alongside the tracking number.

  ## Implementation Prompt
  Implement a Shippo integration to handle automated shipping label generation for e-commerce orders.
  - **User-Facing Outcome**: On the order details page, a merchant can click "Buy Label", see a list of rates from different carriers for the shipment, purchase the label, and download it as a PDF for printing. The order should automatically update its status with the tracking number and notify the buyer.
  - **Acceptance Criteria**:
    - The platform can authenticate with Shippo (OAuth for Cloud, API key for Standalone).
    - A merchant can request shipping rates for an existing order by inputting basic package dimensions and weight.
    - The merchant is presented with shipping options sorted by price/speed.
    - Purchasing a label deducts the cost and provides a downloadable/printable label URL.
    - Tracking numbers are saved to the order and an automated notification is sent to the customer.
    - Webhooks from Shippo correctly update the package tracking status in the OHC dashboard.

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
