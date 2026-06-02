issue_title: "Integrate DoorDash Drive for Autonomous White-Label Local Delivery"
issue_description: |
  # Integrate DoorDash Drive for Autonomous White-Label Local Delivery

  ## Problem Statement
  Food operators like Fatima (The Food Cart) and local retailers like Priya (The Boutique) want to offer local delivery but cannot afford to hire their own drivers. They also hate losing 30% of their revenue to third-party marketplaces like UberEats or DoorDash marketplace apps. They need a way to offer "Delivery" directly on their own OHC storefront, keeping their customer data and margins, while a third-party driver magically appears to deliver the order.

  ## Research Report
  DoorDash Drive is a white-label delivery API that allows businesses to request a Dasher to deliver goods from their location to the customer, without listing the business on the DoorDash app. It charges a flat fee per delivery (typically $7-$10) rather than a percentage of the order. This is perfect for OHC businesses, as they can pass this flat fee onto the customer (or subsidize it) and retain their full profit margin.

  *   **Ease of Use for SMBs:** Zero. The SMB owner just toggles "Enable Local Delivery" and sets a delivery radius and fee. Everything else is handled by the OHC platform.
  *   **Pricing:** Flat fee per delivery. Very predictable. OHC can build a feature to dynamically pass this cost to the buyer at checkout.
  *   **Reputation:** DoorDash has the largest fleet of drivers in the US, ensuring high reliability for local fulfillment.
  *   **Modes Supported:** Cloud (Multi-tenant API keys managed by OHC) and Standalone (User provides their own DoorDash Drive Developer credentials).

  ## Design Doc
  - **Storefront Checkout:** When a customer enters an address within the business's delivery radius, OHC calls the DoorDash Drive API `delivery_quotes` endpoint to get the flat fee and estimated time. This is displayed as a "Local Delivery" shipping option.
  - **Order Confirmation:** Once the order is placed and paid, the OHC "Operations" agent waits until the business owner marks the order as "Ready for Pickup" (or a scheduled prep time elapses).
  - **Dispatch:** The Operations agent calls the DoorDash Drive `deliveries` endpoint to dispatch a driver.
  - **Tracking:** DoorDash sends webhooks to OHC with Dasher status (e.g., "en_route_to_pickup", "en_route_to_dropoff"). The "Customer Success" agent relays these updates to the customer via SMS/Email using a branded tracking link.
  - **SMB Dashboard:** The business owner sees a simple status indicator: "Driver arriving in 5 mins."

  ## Implementation Prompt
  Implement DoorDash Drive as a fulfillment method. Add a "Local Delivery" toggle in the OHC dashboard where owners can set their prep time and delivery radius. During checkout, if the address is within radius, automatically calculate and add the delivery fee. Add a "Request Driver" action to the order management screen that dispatches a Dasher and sends a tracking link to the customer.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
