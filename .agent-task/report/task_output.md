issue_title: "Enable White-Label Local Delivery via Uber Direct / DoorDash Drive"
issue_description: |
  **Problem Statement**
  Fatima (Food Cart Operator) and Maya (Home Baker) want to offer local delivery to their customers, but they cannot afford to hire their own drivers, and they despise the 30% commission taken by marketplace apps like UberEats or DoorDash. They want their customers to order directly from their OHC storefront, pay a flat delivery fee, and have a courier automatically dispatched to pick up the order and deliver it—all without leaving the OHC platform or knowing a third party is involved.

  **Research Report**
  - **Market Demand:** Local businesses are desperately trying to move away from high-commission aggregator marketplaces to direct-to-consumer (D2C) channels. However, the biggest barrier to D2C for food/local goods is fulfilling the delivery.
  - **Competitor Landscape:**
    - Square offers "On-Demand Delivery" powered by Uber/DoorDash.
    - Shopify has local delivery options but usually requires third-party apps like Zapiet to integrate with courier networks.
  - **Tool Capabilities (Uber Direct / DoorDash Drive):**
    - These are the B2B/API arms of the major delivery networks.
    - They provide "Delivery as a Service" (DaaS).
    - Webhooks for real-time tracking (courier assigned, at pickup, dropped off).
    - Automatic dispatching.
  - **SaaS Viability:**
    - *Cloud (Multi-tenant):* Can be seamlessly integrated. OHC acts as the master account, billing the business owner (or passing the cost to the consumer).
    - *Standalone (Local/Private):* The user could provide their own Uber Direct API credentials.
  - **Pricing:** Typically a flat fee per delivery (e.g., $7-$10 depending on distance), unlike the percentage-based commission of the marketplace. This fee can be dynamically calculated via API and passed on to the buyer at checkout.
  - **Ease of Use:** From the SMB's perspective, they just toggle "Enable Local Delivery" and set a delivery radius. The AI Agent handles the rest.

  **Design Doc**
  - **Trigger/Setup:**
    - In the "Operations" dashboard, the user toggles "Enable Local Delivery".
    - They specify their physical pickup address and maximum delivery radius (e.g., 5 miles).
    - They choose whether to absorb the delivery cost or pass it to the customer.
  - **User Experience (SMB Owner):**
    - When an order is placed, it appears in the OHC dashboard.
    - Once the owner marks the order as "Ready for Pickup", OHC automatically calls the Uber Direct API to dispatch a driver.
    - The dashboard shows the driver's ETA and status.
  - **Customer Experience:**
    - The customer sees "Local Delivery" as an option at checkout with an exact price and ETA.
    - The customer receives an SMS (via OHC Communications) with a live tracking link for the driver.
  - **Actions:**
    - Real-time quote generation at checkout based on delivery address.
    - Automatic courier dispatch upon order fulfillment.
    - Status synchronization back to the OHC order dashboard.

  **Implementation Prompt**
  Integrate a white-label local delivery API (such as Uber Direct or DoorDash Drive). Add a configuration pane allowing the business owner to define a delivery radius and pricing strategy (pass-through vs. flat rate). During the checkout flow, implement a dynamic quote API call to calculate the delivery cost based on the customer's address. Hook into the order fulfillment lifecycle: when an order is marked ready, automatically dispatch a courier. Subscribe to webhooks to track the courier's progress and update the order status in OHC, notifying the customer via the AI Ambassador.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
