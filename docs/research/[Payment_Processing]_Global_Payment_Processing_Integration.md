# Issue Brief: Global Payment Processing Integration

**Category**: Payment Processing

## Problem Statement
Stripe isn't enough for global users. Business owners in LATAM or Asia need Mercado Pago, Paytm, or Alipay. Without local payment options, they lose sales.

## Research Report

### Tool Evaluations

**1. Mercado Pago (LATAM focus)**
- **Market Fit**: Absolutely critical for Argentina, Brazil, Mexico, and Colombia. Small businesses rely on Mercado Pago for QR code payments, installment plans (cuotas), and local debit cards that Stripe does not support.
- **Pricing**: Varies by country, typically 3-5% plus fixed fees.
- **Webhook Reliability**: They use Instant Payment Notifications (IPNs). These can sometimes be delayed, requiring our system to handle asynchronous order fulfillment robustly.
- **Mode Compatibility**: In Standalone mode, receiving IPNs requires the OHC Cloud proxy to forward the webhook securely to the local instance.

**2. Paytm / Razorpay (India focus)**
- **Market Fit**: Essential for the Indian market to support UPI (Unified Payments Interface), NetBanking, and local wallets.
- **Pricing**: Razorpay offers excellent developer APIs and competitive local pricing (around 2%).
- **Integration**: Razorpay's checkout SDK can be embedded directly into the OHC Storefront.

**3. Alipay / WeChat Pay (Asia focus)**
- **Market Fit**: Mandatory for businesses targeting Chinese consumers.
- **Integration**: Can often be routed through Stripe's international payment methods, but direct integration offers lower fees for high-volume merchants.

**Summary Recommendation**: Implement a pluggable payment architecture. Start by building the Mercado Pago integration first, as LATAM has a massive gap in unified SMB tooling compared to the US.


## Design Doc
Create a unified Payment Provider interface in OHC. Support plugins for Mercado Pago, Paytm, etc. Standardize checkout flow, currency conversion, and webhook handling for payment success/failure.

## Implementation Prompt
Build a checkout settings page where users can enable different payment gateways depending on their region. Ensure the checkout page dynamically updates based on the selected gateway.

## Priority
P0

## Estimated Scope
Large
