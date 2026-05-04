# Shipping & Logistics

## Title
[Shipping] Real-Time Rates and Label Generation

## Problem Statement
Product sellers like Maya (The Home Baker, if shipping non-perishables) or Priya (The Boutique Owner) struggle with calculating shipping costs and printing labels. They need an automated way to charge customers the correct shipping amount and print ready-to-use courier labels.

## Research Report
- **Evaluated Tools**: Shippo, EasyPost, ShipStation.
- **Ease of Use**: Shippo and EasyPost offer excellent APIs. The complexity of package dimensions and weights must be abstracted for the user.
- **Pricing**: Shippo is $0.05 per label + postage. EasyPost is similar.
- **Carrier Coverage**: Excellent global coverage (USPS, FedEx, UPS, DHL, local carriers).
- **Cloud vs Standalone**: Fully supported in both environments via standard REST APIs.

## Design Doc
- **Triggers**: Customer views cart (rate calculation); owner fulfills order (label generation).
- **Actions**: System calculates real-time shipping rates based on cart contents and destination. Upon fulfillment, it purchases the label and retrieves a printable PDF and tracking number.
- **User View**: Customer sees accurate shipping costs at checkout. The owner clicks "Print Shipping Label" on an order, automatically generating the PDF and emailing the tracking link to the customer.

## Implementation Prompt
Implement a shipping management integration using a provider like Shippo or EasyPost. The system must automatically calculate shipping rates at checkout based on standard package sizes. Add a feature to the order management screen allowing the business owner to generate and download a shipping label with one click, automatically updating the order status to "Shipped" and notifying the customer.
- **Acceptance Criteria**: The checkout flow automatically displays accurate, real-time shipping rates based on the customer's address and standard package sizes/weights. The business owner can click a "Print Shipping Label" button on a paid order to generate a printable PDF label. Generating a label automatically updates the order status to "Shipped" and sends an email notification with the tracking number to the customer.

## Priority
P2

## Estimated Scope
Medium
