# [shipping-logistics] Automated Shipping & Label Generation

**Title:** Integrate Automated Shipping Rates and Label Generation

**Problem Statement:**
E-commerce small business owners struggle with calculating accurate shipping costs at checkout and manually printing shipping labels. Guessing shipping costs leads to lost margins, and manual label entry is time-consuming and error-prone.

**Research Report:**
* **Tools Evaluated:** Shippo, EasyPost, ShipEngine.
* **Ease of Use:** Platforms like Shippo provide a unified API that abstracts away dozens of individual carriers (USPS, FedEx, DHL), making it extremely easy for a business owner to access discounted rates without negotiating individual contracts.
* **Key Advantages:**
  - Real-time rate calculation at checkout.
  - 1-click label purchasing and printing from the dashboard.
  - Automatic tracking number generation and status updates for the customer.
* **Risks:**
  - Handling edge cases like international customs forms or oversized packages.
* **Pricing Estimate:** Pay-as-you-go (e.g., $0.05 per label) + cost of postage.
* **Environment Support:** fully supported in Cloud mode. Standalone mode can connect to the API via external network requests.

**Design Doc:**
* **Trigger:** The business owner configures package dimensions and connects a shipping provider in the "Shipping" settings.
* **Actions:** During checkout, OHC calls the API to fetch live rates. When an order is paid, the owner sees a "Print Label" button in the order details. Clicking it deducts the postage cost and generates a printable PDF.
* **User Experience:** The owner manages fulfillment entirely within the OHC Order view. They click "Fulfill", print the label, and the system automatically emails the tracking link to the customer.

**Implementation Prompt:**
Integrate a shipping API aggregator (like Shippo or EasyPost) to handle automated fulfillment. The checkout flow should support dynamic rate calculation based on cart weight/dimensions. Within the OHC admin dashboard, implement a workflow allowing the merchant to purchase and generate a shipping label PDF for an order with a single click, automatically updating the order status to "Shipped" and storing the tracking number.

**Priority:** P2
**Estimated Scope:** Large