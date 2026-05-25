issue_title: "Implement Shippo for Multi-Carrier Shipping & Automated Label Generation"
issue_description: |
  # [shipping] Shippo Integration

  ## Title
  Implement Shippo for Multi-Carrier Shipping & Automated Label Generation

  ## Problem Statement
  Small business owners selling physical goods struggle with managing shipments manually. Currently, when they make a sale, they have to copy-paste customer addresses into external carrier websites (like USPS, UPS, FedEx), manually calculate shipping rates, pay for the label, print it, and then manually paste the tracking number back into OHC to notify the customer. This process is time-consuming, prone to data entry errors, and does not allow them to leverage discounted carrier rates, hurting their margins and slowing down fulfillment as their order volume grows. Maya (Artisan Baker) needs a way to fulfill her local and regional shipments in a few clicks without leaving the OHC platform.

  ## Research Report
  Shippo (https://goshippo.com/) is a leading multi-carrier shipping API that allows merchants to connect to over 85+ global carriers (USPS, UPS, FedEx, DHL, etc.) through a single integration.

  - **Ease of Use for Non-Technical Users:** Shippo's interface is very user-friendly. Once integrated into OHC, the complexity of carrier APIs is entirely abstracted away. Small business owners just need to input package weight/dimensions and click "Buy Label".
  - **Pricing:** Shippo offers a pay-as-you-go model (no monthly fee) where merchants only pay 5¢ per label, plus the cost of postage, making it extremely accessible for low-volume sellers. They also pass on deep discounts for USPS and UPS.
  - **Reputation:** Highly rated on G2 and Shopify App Store for reliability and competitive rates.
  - **SaaS Viability:** Suitable for both multi-tenant (Cloud) and private (Standalone) deployments as the OAuth flow and webhook systems are robust.

  ## Design Doc
  **Trigger:**
  1. A new order is placed with a physical shipping address.
  2. The user navigates to the specific order details page within OHC and selects "Fulfill Order".

  **Actions:**
  1. OHC fetches live shipping rates from Shippo based on order weight, dimensions, and destination.
  2. The user selects a shipping rate and purchases the label directly within OHC.
  3. OHC retrieves the generated shipping label (PDF/ZPL) and tracking number from Shippo.
  4. OHC automatically marks the order as "Shipped" and emails the tracking number to the customer.

  **User Experience:**
  The user stays entirely within the OHC platform. They see a list of rates (e.g., "USPS Priority - $8.50"), select one, and click "Print Label". The label opens in a new tab for printing, and the customer is automatically notified.

  ## Implementation Prompt
  Integrate Shippo to enable users to view live shipping rates, purchase shipping labels, and automatically sync tracking information back to OHC orders.

  **Acceptance Criteria:**
  - Users can connect their own carrier accounts or use Shippo's default discounted carrier accounts.
  - When fulfilling an order, users can input package dimensions and weight to see real-time shipping rates from available carriers.
  - Users can select a rate and purchase a label.
  - Upon purchase, the label is provided as a printable document (PDF).
  - The OHC order status automatically updates to "Shipped".
  - The customer receives an automated email containing the tracking number and a link to track the package.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
