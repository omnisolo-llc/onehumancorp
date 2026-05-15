# [Payment] Razorpay Integration (India)

## Title
Native Indian Payment Integration with Razorpay

## Problem Statement
Rohan (Handmade Crafts) in India cannot easily use Stripe for local customers who prefer UPI, RuPay, or local net banking. He needs a trusted local payment gateway that feels native to Indian customers, avoiding the high failure rates and friction associated with international payment processors in the Indian market.

## Research Report
- **Strategy**: Direct API integration with Razorpay.
- **Target Persona**: Rohan (Indian SMB owner).
- **Advantages**: Deep support for UPI (India's primary payment method), local cards, and net banking. Includes features like Razorpay Magic Checkout for higher conversion. Trusted by millions of Indian merchants.
- **Risks**: Stringent regulatory KYC requirements in India for the merchant.
- **Pricing**: Competitive local pricing (~2% per transaction for domestic).
- **Ease of Use**: Indian customers are highly familiar with the Razorpay checkout interface.
- **Compatibility**: Cloud & Standalone.

## Design Doc
- **Integration with OHC**:
    - Merchant chooses "India" as their region during setup, prompting Razorpay activation.
    - OHC uses the Razorpay Orders API to initiate payments.
    - Checkout widget supports UPI QR codes and local bank redirects natively.
    - The "Accountant" AI agent reconciles INR transactions and tracks local tax (GST) compliance.
- **User View**: A checkout screen that features UPI prominently, making payment instant for the customer.

## Implementation Prompt
Implement Razorpay as a native payment provider for the Indian market. Ensure the checkout flow supports UPI, local cards, and net banking. Normalize Razorpay webhooks into the standard OHC order and fulfillment system. Ensure the merchant can view transaction details in INR within the OHC dashboard.

## Priority
P1

## Estimated Scope
Large
