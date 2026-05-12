# Issue Brief: Razorpay (India Payments)

## Title
Implement Razorpay (India Payments) for Small Business Owners

## Problem Statement
An independent consultant in India needs to accept payments via UPI, which is the standard across the country. Traditional credit card gateways are insufficient.

## Research Report
Razorpay provides seamless access to UPI and local Indian banking methods.

**Persona Impact:** The consultant sends an invoice link. The client taps 'Pay', their Google Pay app opens automatically, and the payment is completed in seconds via UPI.

**Advantages:** Best-in-class checkout experience for Indian consumers.

**Risks:** Indian KYC regulations require significant documentation from the business owner before accepting live payments.

**Pricing Estimate:** Standard competitive transaction fees (typically around 2%).

**Environment:** Supported in both Cloud and Standalone deployments.

## Design Doc
1.  **Seamless Checkout:** Implement the checkout flow so that mobile users are smoothly transitioned to their native UPI apps to complete the payment.
2.  **KYC Assistance:** Provide clear, plain-language instructions within OHC on what documents the user needs to provide to Razorpay.

## Implementation Prompt
Integrate Razorpay to ensure Indian businesses can offer a flawless UPI payment experience to their customers directly from OHC invoices.

## Priority
P1

## Estimated Scope
Medium

### Unique Considerations
The Razorpay checkout modal must be deeply embedded into the OHC invoice view to prevent the customer from feeling like they are being redirected to a suspicious third-party site, maximizing conversion rates.
