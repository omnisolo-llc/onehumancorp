# Alipay Integration Issue Brief

## Title
Integrate Alipay to Support Cross-Border and Asian Markets

## Problem Statement
Small businesses operating in or selling to Asian markets (especially China) struggle with payment conversions when only offering Western payment methods like Stripe or PayPal. They need to offer Alipay to capture these sales effectively.

## Research Report
- Alipay is the dominant digital wallet in China, with massive usage globally among Chinese tourists and expats.
- Supporting Alipay significantly increases conversion rates for businesses targeting this demographic.
- Pricing: Transaction fees are generally competitive, though cross-border settlement can have currency conversion costs.
- Competitors: WeChat Pay (similar dominance, slightly different use cases).
- Integration: Can be integrated directly via Alipay Global API, or often routed through an aggregator like Stripe. Direct integration offers better rates but is technically more complex.
- Cloud/Standalone: Requires secure payment gateway handling. Cloud mode is strongly preferred for security and compliance. Standalone might require a proxy or specialized secure enclave.

## Design Doc
- In the "Payments" settings, users can enable Alipay as a payment method for their storefront or invoices.
- During checkout, customers selecting Alipay are presented with a QR code to scan with their mobile app, or redirected to the Alipay web portal.
- The OHC backend verifies the payment notification via Alipay's secure webhook/callback system before marking the order as paid.

## Implementation Prompt
Implement an Alipay payment gateway integration. Add Alipay to the checkout payment options. Handle the generation of the payment QR code or redirect URL. Implement the secure callback endpoint to receive payment confirmations from Alipay and update the corresponding order status in OHC.

## Priority
P3

## Estimated Scope
Large
