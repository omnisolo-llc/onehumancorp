# 🔍 Scout: Tool Integration Research - DoorDash Drive API

## Title
Implement DoorDash Drive API for White-Label Local Delivery

## Problem Statement
Local business owners, particularly in food and beverage (like Fatima's food cart) and local retail (like Maya's bakery or Priya's boutique), often lose customers who want delivery but are outside the business's immediate capability to deliver themselves. Joining the main DoorDash marketplace takes a huge 30% cut of their margins, which is prohibitive for small operations. They need a way to offer fast, reliable local delivery directly from their own website (commission-free on their own orders) without hiring delivery staff, so they can increase sales volume while keeping their hard-earned profits.

## Research Report
DoorDash Drive (https://developer.doordash.com/) is a white-label fulfillment API that allows businesses to request a Dasher to deliver an order placed on their own platform.

- **Ease of Use for Non-Technical Users:** Extremely easy once integrated. The business owner simply toggles "Enable Local Delivery" in OHC. When a local order comes in, the Dasher is automatically requested, and tracking is handled invisibly. The owner just hands the package to the Dasher.
- **Pricing:** DoorDash Drive charges a flat fee per delivery (typically $7-$10 depending on distance) rather than a percentage commission. The business owner can choose to absorb this cost, pass it entirely to the customer as a "Delivery Fee," or split it. This is vastly superior to the 30% marketplace commission.
- **Reputation:** Industry leader in North America for local fulfillment with high reliability and a massive driver network.
- **SaaS Viability:** Excellent. The API uses a standard RESTful architecture with webhook support for delivery status updates (e.g., Dasher assigned, picked up, delivered), easily supported in OHC's multi-tenant cloud environment.

## Design Doc
**Trigger:**
1. Customer enters a delivery address at checkout that is within the local delivery radius (e.g., 5 miles).
2. The user selects "Local Delivery" at checkout.
3. Once the order is confirmed and marked "Ready for Pickup" by the business owner (or automatically based on prep time).

**Actions:**
1. OHC backend automatically calls the DoorDash Drive API `Create Delivery` endpoint, passing pickup address, drop-off address, and order details.
2. DoorDash dispatches a Dasher.
3. OHC listens to DoorDash webhooks to receive real-time status updates and tracking links.
4. OHC updates the order status and sends the DoorDash tracking link to the customer via SMS/email.

**User Experience:**
For the business owner: They see an order labeled "Local Delivery - Dasher Arriving in 10 mins." They prepare the item and hand it to the Dasher.
For the customer: They get an SMS with a live tracking link, just like a regular DoorDash order, but they ordered directly from the small business's site.

## Implementation Prompt
Integrate the DoorDash Drive API to enable white-label local delivery for physical products and food orders.

**Acceptance Criteria:**
- Business owners can enable "Local Delivery via DoorDash" in their shipping/delivery settings.
- Business owners can configure how much of the flat delivery fee is passed to the customer.
- Customers within the delivery radius see "Local Delivery" as an option at checkout, with the calculated fee.
- When an order is ready, a delivery request is automatically dispatched to DoorDash Drive.
- The business owner dashboard shows real-time Dasher status (Assigned, Arriving, Picked Up).
- The customer receives an automated notification with the DoorDash tracking link.
- Webhooks correctly update the order status in OHC to "Delivered" once completed.

## Priority
P1

## Estimated Scope
Medium
