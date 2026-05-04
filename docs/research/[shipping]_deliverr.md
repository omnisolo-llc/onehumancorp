### Title
Integrate Deliverr (Flexport) for Automated E-commerce Fulfillment

### Problem Statement
Small business owners selling physical products (e.g., boutique clothing, handmade crafts) spend too much time picking, packing, and shipping boxes. They lack the volume to negotiate cheap shipping rates and the infrastructure to offer Amazon Prime-like 2-day delivery, putting them at a competitive disadvantage. They need an automated fulfillment solution that integrates directly with their online storefront.

### Research Report
**Tool Evaluated:** Deliverr (now part of Flexport)
**Overview:** Deliverr was founded in 2017 to provide Amazon-like 2-day fulfillment for independent merchants across platforms like Shopify, Walmart, and eBay. It was acquired by Shopify in 2022 for $2.1 billion and subsequently sold to Flexport in 2023. Flexport now serves as the official logistics partner for Shopify.
**Key Features & Advantages:**
- Asset-light fulfillment network placing inventory close to demand.
- Enables "2-day" fast-shipping badges on storefronts, increasing conversion rates.
- Transparent, all-in-one pricing structure.
**Risks:** The transition from Deliverr to Shopify Logistics to Flexport has involved significant structural changes. Integration should rely on the most stable, current API endpoints provided by Flexport for merchant fulfillment.
**Ease of Use:** High for the merchant. They send bulk inventory to the network, and the system handles individual order fulfillment automatically.
**Pricing:** Variable based on item size/weight and fulfillment speed.
**Deployment:** API integration for order routing and tracking.

### Design Doc
**Integration Trigger:** A business owner enables "Automated Fulfillment" in the OHC "Operations" settings and connects their Flexport/Deliverr account.
**Action:** OHC automatically pushes new physical product orders to the fulfillment network and syncs inventory levels back to the OHC database.
**User Experience:**
- **Business Owner:** After sending bulk inventory to the fulfillment centers, the owner does nothing when an order arrives. OHC automatically routes the order, and the Operations AI agent updates the order status to "Shipped" once the tracking number is generated.
- **Customer:** Sees a "2-Day Delivery" promise at checkout. Receives automated tracking emails once the package is dispatched.

### Implementation Prompt
Implement a fulfillment integration with Flexport (formerly Deliverr) to automate order shipping for merchants selling physical goods.

**Acceptance Criteria:**
1. Implement an authentication flow for merchants to connect their fulfillment account.
2. Build an automated sync that pushes new, paid orders containing physical products to the fulfillment API.
3. Build a webhook listener or polling mechanism to retrieve tracking numbers and shipment statuses from the fulfillment network.
4. Update the OHC order record with the tracking information and trigger the standard OHC customer notification flow.
5. Provide a dashboard view in OHC for the merchant to see their inventory levels currently held at the fulfillment centers.

### Priority
P2

### Estimated Scope
Large
