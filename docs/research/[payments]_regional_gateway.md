# 💳 Payment Processing: Regional Gateway

## Title
Regional Payment Gateway Integration Framework

## Problem Statement
While Stripe is powerful, it does not dominate every market. Business owners in regions like LATAM or India need local payment methods to avoid high failure rates and excessive fees. A single dependency on Stripe limits OHC's global reach. We need a flexible payment architecture that allows plugging in regional payment gateways (e.g., Mercado Pago, Razorpay) seamlessly.

## Research Report
- **Goal**: Evaluate regional payment providers and design an abstraction layer for multi-gateway support.
- **Tools Evaluated**:
    - **Mercado Pago (LATAM)**: Essential for South America. Supports local cards, boleto, and Pix.
    - **Razorpay (India)**: Dominant in India. Supports UPI, local cards, and net banking.
    - **Paystack (Africa)**: Crucial for African markets.
    - **Adyen**: Global provider, but enterprise-focused and complex for small businesses to onboard.
- **Recommendation**: Create an internal **Payment Abstraction Interface**. The system should initially support Stripe (primary) but be architected so that adding Mercado Pago or Razorpay requires only implementing the interface methods (CreateCharge, ProcessWebhook, Refund). This abstraction ensures that payment gateways function perfectly in both Cloud and Standalone modes.
- **User Impact**: A business owner in Brazil can easily select Mercado Pago during onboarding. Their customers can pay using Pix directly on the OHC checkout page, leading to higher conversion rates.

## Design Doc
- **Component**: `PaymentGatewayProvider`
- **Responsibilities**:
    - Abstract payment operations: Checkout Sessions, Payment Intents, Refunds, and Webhook verification.
    - Route transactions to the configured gateway for the tenant.
    - Standardize payment status codes (Pending, Succeeded, Failed, Refunded) across different providers.
- **Integration Point**: The OHC Checkout UI will request payment options from this service, and the service will handle the provider-specific SDK/API calls.

## Implementation Prompt
Implement a Payment Gateway abstraction layer. Define a Go interface for core payment operations (Charge, Refund, VerifyWebhook). Refactor the existing Stripe integration to implement this interface. Create a stub or minimal implementation for a regional gateway (e.g., Mercado Pago) to prove the abstraction works. Ensure the tenant configuration allows selecting the active payment gateway.

## Priority
P1

## Estimated Scope
Large
