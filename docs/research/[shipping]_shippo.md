# Shipping & Logistics Integration (Shippo)

## Title
Integrate Shippo for Real-Time Shipping Rates and Labels

## Problem Statement
For business owners selling physical goods, calculating accurate shipping costs and generating labels is a major headache. If they charge too little, they lose money; if they charge too much, they lose sales. They need an automated way to get rates and print labels without leaving the OHC app.

## Research Report
- **Tool Evaluated**: Shippo API.
- **Benefits for OHC Users**: Connects to multiple carriers (USPS, UPS, FedEx, DHL, local carriers) to provide real-time rates at checkout and easy label printing from the OHC dashboard.
- **Ease of Use**: OHC abstracts the complexity. The user just enters package dimensions and weight, and clicks "Print Label".
- **Pricing**: Pay-as-you-go per label, or a monthly subscription for higher volume. Very accessible.
- **Reputation**: Reliable, developer-friendly API, broad carrier support.
- **Cloud vs. Standalone**: Ideal for Cloud mode.

## Design Doc
- **User Experience**: During checkout, the customer sees accurate shipping rates based on their address. When fulfilling an order, the business owner clicks "Generate Label", prints it, and sticks it on the box. Tracking info is automatically emailed to the customer.
- **Integration**: Use Shippo API to fetch rates during the checkout flow. Use the API to generate labels and customs forms during fulfillment. Handle tracking webhooks.
- **Triggers**: Checkout process (rate request), Order fulfillment (label request), Tracking status change.
- **Actions**: Display rates, generate PDF label, update order status, send tracking email.

## Implementation Prompt
Integrate the Shippo API to handle shipping logistics. The system must be able to calculate real-time shipping rates at checkout based on package weight/dimensions and the customer's address. It should also allow the business owner to generate and print shipping labels directly from the OHC order fulfillment screen. Acceptance criteria include accurate rate calculation during checkout, successful generation of a printable shipping label, and automated tracking updates sent to the customer.

## Priority
P1

## Estimated Scope
Large
