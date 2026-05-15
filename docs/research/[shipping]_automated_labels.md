# Automated Shipping Label Generation

## Problem Statement
E-commerce businesses waste hours manually copying customer addresses into carrier websites to print shipping labels. Calculating accurate shipping costs at checkout is also difficult, leading to lost margins.

## Research Report
**Competitive Landscape:**
1. **EasyPost / Shippo:** Great APIs for aggregating multiple carriers (USPS, FedEx, UPS). Negotiated rates are a huge plus.
2. **Sendle:** Excellent for small businesses sending small parcels, carbon neutral, flat rates.
3. **Direct Carrier APIs:** Too complex for OHC to maintain individually.

**Evaluation:**
- **Ease of Use:** User needs a 'Buy Label' button next to an order. The system should auto-fill dimensions based on product data.
- **Pricing:** Shippo/EasyPost charge pennies per label, highly affordable.
- **Cloud vs Standalone:** Works identically in both. Standalone might require the user to bring their own Shippo API key.

## Design Doc
- **Trigger:** An order is marked as 'Paid'. User views the order details.
- **Action:** User clicks 'Create Label'. OHC queries Shippo for rates, user selects one, and OHC generates the PDF label and tracking number.
- **User Experience:** 1-click label printing from the order dashboard. Automatic email to customer with tracking link.

## Implementation Prompt
Integrate the Shippo API for order fulfillment. On the order details page, provide a UI to input package weight/dimensions (defaulting to saved product specs) and fetch shipping rates. Allow the user to purchase the label, which triggers a download of the PDF label and updates the order status to 'Shipped', attaching the tracking number. Automatically email the customer the tracking info.

## Priority
P1

## Estimated Scope
Medium
