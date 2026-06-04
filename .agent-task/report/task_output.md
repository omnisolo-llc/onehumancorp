issue_title: "Implement Shippo API Integration for Multi-Carrier Shipping"
issue_description: |
  ## Problem Statement
  Small business owners who sell physical products (like Priya the boutique owner or Maya the home baker) need an efficient, cost-effective way to manage shipping. They are currently forced to use external carrier sites, manually copy-paste addresses, and guess at the cheapest shipping rates. This process is time-consuming, error-prone, and frustrating for non-technical users who just want to fulfill their orders quickly. They need a simple, centralized way to compare rates across carriers (USPS, UPS, FedEx), print labels, and automatically notify customers with tracking info—all without leaving the OHC platform.

  ## Research Report
  ### Market Need & Competitor Analysis
  - Platforms like Shopify and Wix offer built-in shipping solutions (e.g., Shopify Shipping), which is a key differentiator for e-commerce businesses.
  - Users expect to manage their entire order lifecycle in one place.
  - We evaluated leading shipping APIs, focusing on Shippo due to its robust ecosystem and developer-friendly design.

  ### Tool Evaluation: Shippo API
  - **Capabilities:**
    - Access to 40+ shipping carriers globally (USPS, UPS, FedEx, DHL, etc.) with up to 90% discounted rates.
    - End-to-end functionality: Rate comparison, label generation, address validation, returns, and real-time tracking.
    - Designed for multi-tenant platforms: Supports sub-accounts (`/shippo-accounts`), allowing OHC to provision individual shipping accounts per tenant.
    - Robust Webhook support (`/webhooks`) for events like `track_updated`, `transaction_created`, ensuring real-time syncing of order status.
  - **Pricing:**
    - *API Starter:* Free up to 30 labels/month, then 7¢ per label. Address validation is 2¢ (US) / 8¢ (Intl). Tracking is 2¢/track.
    - *API Premier:* Custom volume discounts for high-volume platforms.
  - **Non-Technical User Experience:**
    - To a user like Maya or Priya, the experience will be entirely seamless. They click "Fulfill Order," see the cheapest rate for their package size, and click "Buy Label." The complexity of carrier accounts and API calls is entirely hidden.

  ## Design Doc
  ### High-Level Architecture
  - **Tenant Provisioning:** When a user sets up their store, OHC's Operations Agent provisions a Shippo sub-account via the Shippo Accounts API.
  - **Order Fulfillment Flow:**
    1. Customer places an order on the tenant's OHC storefront.
    2. Tenant reviews the order in their dashboard and selects "Fulfill."
    3. OHC backend calls Shippo's Rating API using the tenant's sub-account credentials, pre-filling the customer's address (validated via Address API).
    4. Tenant selects a rate and purchases the label (Transactions API).
    5. OHC saves the label URL and tracking number.
  - **Tracking & Notifications:** OHC registers webhooks with Shippo for `track_updated` events. When a package ships or is delivered, Shippo notifies OHC, and the Customer Success Agent automatically emails the customer.
  - **Data Model:** Store Shippo sub-account IDs on the `Tenant` table, and tracking/label URLs on the `Order` table.

  ## Implementation Prompt
  - Integrate the Shippo API to enable multi-carrier rate comparison and label purchasing natively within OHC.
  - The UI must allow a non-technical business owner to click "Fulfill" on an order, view the top 3 cheapest shipping options, and click "Buy Label" to generate a printable PDF label.
  - Integrate address validation to prevent failed deliveries.
  - Automatically update the order status to "Shipped" and generate a tracking link when the label is purchased.
  - **Acceptance Criteria:**
    - A user can view shipping rates for an order.
    - A user can purchase a label and download the PDF.
    - The customer address is validated before purchase.
    - The backend handles Shippo sub-account creation per tenant to isolate billing and data.
    - Webhook endpoints are created to receive tracking updates.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
