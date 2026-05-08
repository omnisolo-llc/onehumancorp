# Title: Shippo Integration for Seamless Shipping

## Problem Statement
Small business owners selling physical goods struggle with calculating accurate shipping rates at checkout and manually generating shipping labels. Taking packages to the post office and waiting in line is a massive waste of time.

## Research Report
Shippo provides a unified API for multiple carriers (USPS, FedEx, UPS, DHL).
- **Ease of use:** Business owner just connects their Shippo account.
- **Pricing:** Pay-as-you-go per label or a monthly subscription for volume.
- **Reputation:** Highly reliable, developer-friendly.
- **Key advantages:** Access to discounted USPS rates immediately, multi-carrier support out of the box, and automated tracking updates.
- **Risks:** Handling edge cases for international shipping (customs forms) can be complex. Rate limits on the API might affect high-volume stores.
- **Environment:** Cloud works perfectly. Standalone works perfectly via outbound API calls.

## Design Doc
- User goes to "Shipping" and connects Shippo.
- During customer checkout, OHC pings Shippo to display live shipping rates based on package weight and destination.
- When an order is paid, the user clicks "Generate Label" in OHC.
- OHC purchases the label via Shippo and provides a printable PDF.
- Tracking numbers are automatically emailed to the customer.

## Implementation Prompt
Integrate Shippo API to handle real-time shipping rates and label generation. Add a "Shipping Info" section to physical products to store weight/dimensions. Create an endpoint that queries Shippo for rates during checkout. Add a "Buy Label" button in the order management view that returns a printable PDF link.

## Priority
P1

## Estimated Scope
Large
