# Title: Alternative Localized Payment Processing

## Problem Statement
Small business owners in specific markets (e.g., LATAM, India, Asia) or those targeting tourists lose sales when they can only accept global cards via Stripe, as customers prefer local payment methods like Alipay.

## Research Report
- **Tool Evaluated**: Alipay
- **Persona Value**: High for businesses catering to Asian markets or Chinese tourists.
- **Advantages**: Huge user base, widely trusted in Asia, supports mobile QR payments.
- **Risks**: Regulatory complexities for merchants outside of China accepting Alipay.
- **Pricing**: Varies based on region and transaction type.
- **Cloud vs Standalone**: Both compatible via integrations.

## Design Doc
- **Integration Trigger**: User enables Alipay in Payment Settings.
- **Action**: OHC routes checkout payments through the Alipay API/QR code flow.
- **User Interface**: Checkout page displays Alipay as a payment option.

## Implementation Prompt
Integrate Alipay as an alternative payment provider for checkout. Ensure the checkout flow properly redirects to the Alipay app or displays a scannable QR code for the customer, and securely handles the payment success webhook.
- **Acceptance Criteria**: Customer can select Alipay at checkout, complete payment, and OHC marks the order as paid.

## Priority
P2

## Estimated Scope
Medium
