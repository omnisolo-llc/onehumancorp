# Integrate DoorDash Drive for White-Label Local Delivery

**Title:** Integrate DoorDash Drive for White-Label Local Delivery
**Problem Statement:** Business owners like Fatima (The Food Cart Operator) and Maya (The Home Baker) want to offer local delivery to their customers but cannot afford to hire their own delivery drivers. They also want to avoid the 30% commissions charged by third-party marketplace apps (like UberEats or DoorDash marketplace), preferring customers order directly from their OHC-powered storefront.

**Research Report:** DoorDash Drive API offers white-label delivery as a service. Businesses can integrate DoorDash's fleet of Dashers into their own website. The customer orders on the OHC storefront, pays a flat delivery fee (which the business owner can pass on or subsidize), and DoorDash handles the logistics without listing the business on the main DoorDash app.
- *Key Advantages:* Flat-rate pricing (no percentage commissions), access to a massive existing driver fleet, real-time tracking webhooks, allows businesses to keep their customer data.
- *Key Risks:* Only available in regions where DoorDash operates. Handling delivery disputes (missing items, cold food) can be complex for the business owner.
- *Modes Supported:* Cloud (Multi-tenant Platform Account).

**Design Doc:**
- In OHC settings, the user enables "Local Delivery powered by DoorDash Drive".
- During checkout, if the customer's address is within the configured delivery radius, they see a "Local Delivery" option with a dynamically calculated flat fee.
- When the order is marked "Ready for Pickup" by the OHC "Operations" agent (or manually by Fatima/Maya), OHC calls the DoorDash Drive API (`POST /drive/v2/deliveries`) to request a Dasher.
- DoorDash Drive webhooks update the OHC dashboard and the "Customer Success" agent sends real-time tracking SMS/emails to the customer.

**Implementation Prompt:** Add "Local Delivery" settings in the OHC dashboard, allowing users to define a delivery radius and flat fee. Integrate the DoorDash Drive API to calculate delivery availability at checkout. In the order management view, add a "Request Courier" button that dispatches a DoorDash driver and shares a tracking link with the customer.

**Priority:** P1
**Estimated Scope:** Large
