# Issue Brief: Shipping & Logistics via Shippo

## Title
Automated Shipping Labels & Real-time Tracking

## Problem Statement
"I spend my evenings at the post office waiting in line." Small business owners like Maya and Priya need to skip the line. They need to print labels at home, get the best rates, and have their customers automatically notified of tracking—without ever leaving the OHC app.

## Research Report
- **Tool**: Shippo API.
- **Ease of Use**: High. Aggregates 85+ carriers (USPS, UPS, FedEx, DHL, etc.) into one API.
- **Persona Fit**:
    - **Maya (Baker)**: Gets real-time rates for cake delivery (if using a carrier).
    - **Priya (Boutique)**: Prints shipping labels for her online orders in bulk.
- **Cloud vs. Standalone**:
    - **Cloud**: Primary for label generation and rate lookup.
    - **Standalone**: Can use the Tracking API to show status of local shipments.
- **Pricing**: $0.05 per label (Starter) or $10/mo for bulk. Access to deeply discounted carrier rates (up to 89% off).
- **Competitive Analysis**: Pirate Ship is great for individuals, but Shippo has a much better "Integrator" API for platforms like OHC.

## Design Doc
- **Integration**: "The Manager" (Operations Agent) handles address validation and label generation.
- **User Experience**:
    - Order arrives -> AI validates the address.
    - User taps "Print Label". OHC fetches the cheapest rate via Shippo.
    - Customer gets a "Premium" tracking email with the OHC brand.

## Implementation Prompt
Integrate the Shippo API to provide real-time shipping rates and label generation. Implement address validation during the checkout flow to reduce delivery failures. Wire "The Manager" agent to proactively suggest the most cost-effective shipping method for every order.

## Priority
P1 (High)

## Estimated Scope
Medium
