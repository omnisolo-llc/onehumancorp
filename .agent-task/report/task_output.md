# [Integration] Multi-Carrier Shipping & Fulfillment via Shippo

## Problem Statement
Small business owners (like Carlos, an artisan seller, or Priya, an online boutique owner) face a logistical nightmare when fulfilling orders. They often have to manually copy customer addresses into different carrier websites (USPS, FedEx, UPS, local carriers), guess shipping costs during checkout, and manually send tracking numbers to customers. This process is time-consuming, prone to errors, and prevents them from accessing discounted shipping rates reserved for high-volume enterprise shippers. They need a simple, centralized way to print shipping labels and provide real-time shipping rates and tracking to their customers.

## Research Report
**Market Need:** Shipping logistics is consistently ranked as one of the top pain points for independent e-commerce businesses. A lack of automated tracking updates leads to high customer support volume ("Where is my order?").

**Tool Selection: Shippo**
- **Overview:** Shippo is a multi-carrier shipping API and dashboard that allows businesses to access real-time rates, print labels, and automate tracking.
- **Ease of Use (Non-Technical User):** Shippo offers an extremely user-friendly standalone dashboard, but its API allows platforms (like OHC) to embed this functionality seamlessly. The user simply sees an "Approve & Print Label" button within their OHC dashboard, completely hiding the complexity of carrier negotiation.
- **Pricing:** Shippo has a generous "Starter" tier with no monthly fees, charging only a few cents per label (or free if using their carrier accounts). This is highly viable for bootstrapping businesses. It scales well into volume-based enterprise pricing.
- **Reputation:** Highly rated among Shopify, Wix, and WooCommerce users for reliability and access to discounted USPS/UPS rates.
- **SaaS Viability:** Excellent. Can be operated in a multi-tenant cloud environment via OHC, and also supports API-keys for standalone/local deployments.

## Design Doc
**Integration Trigger:**
1. When a customer adds items to their cart, OHC queries the integration to display real-time shipping options and costs.
2. When an order is marked as "Paid" in OHC, the integration generates a pending shipment record.

**User Actions:**
- The small business owner opens the "Orders" tab in OHC.
- They select an order and click "Create Shipping Label".
- The integration presents the pre-calculated cheapest shipping options (e.g., USPS Ground Advantage).
- The owner confirms, and the integration returns a printable PDF label.

**User Experience:**
- Customers receive automated email/SMS notifications with a tracking link once the label is generated.
- The business owner never has to leave the OHC platform or manually enter package dimensions if product weights are pre-configured.

## Implementation Prompt
Implement an integration that allows OHC users to seamlessly fulfill orders. The user should be able to connect a shipping provider (like Shippo) from the OHC Integrations page. Once connected, the OHC checkout flow should display accurate, real-time shipping rates based on the customer's address and cart weight. In the admin dashboard, the user should be able to view an unfulfilled order, select a box size, and click a button to generate and download a PDF shipping label. Generating the label should automatically mark the OHC order as "Fulfilled" and trigger a tracking notification to the customer.

## Priority
P1

## Estimated Scope
Medium
