# [Payment] OHC Tool Integration Research Brief: Global Payment Processing

## Title
Accept Localized Payments Globally (Beyond Stripe)

## Problem Statement
While Stripe is excellent, many small business owners in emerging markets (LATAM, India, Southeast Asia) cannot use it, or their customers prefer local payment methods (e.g., UPI in India, Pix in Brazil, GrabPay in SE Asia). Limiting OHC to Stripe shuts out a massive portion of the global SMB market who need to accept payments seamlessly online.

## Research Report
To support a truly global user base, we must offer payment gateways that specialize in regional, alternative payment methods (APMs).

**Evaluated Tools:**

1. **Razorpay**
    *   **Focus:** India-first, but expanding globally.
    *   **Pros:** The undisputed leader in India. Supports UPI, Netbanking, local wallets, and cards. Essential if we have users targeting the Indian market.
    *   **Cons:** Primarily focused on businesses with Indian entities.
    *   **Pricing:** 2% per transaction for standard domestic cards.
    *   **Modes:** Works in Cloud and Standalone via API.

2. **Mercado Pago**
    *   **Focus:** LATAM (Brazil, Mexico, Argentina, etc.).
    *   **Pros:** Massive penetration in LATAM. Supports Pix, Boleto, and local installments.
    *   **Cons:** Regional fragmentation.
    *   **Pricing:** Variable percentage based on country and payment method.
    *   **Modes:** Works in Cloud and Standalone via API.

3. **dLocal**
    *   **Focus:** Cross-border payments for emerging markets.
    *   **Pros:** One integration to access hundreds of local payment methods across LATAM, Asia, and Africa.
    *   **Cons:** Typically targets enterprise merchants, not necessarily small self-serve SMBs.
    *   **Pricing:** Custom enterprise pricing.
    *   **Modes:** Works in Cloud and Standalone via API.

**Recommendation:**
Instead of integrating dozens of regional gateways, OHC should adopt a "Payment Orchestration" strategy or integrate a highly versatile global gateway. If we must pick one to complement Stripe immediately, **Razorpay** is critical for the Asian market, and **Mercado Pago** for LATAM.

For this brief, we will focus on an architectural change: abstracting the payment layer so multiple providers can be plugged in.

## Design Doc
**Integration Approach: Payment Provider Abstraction Layer**

1.  **Configuration:**
    *   Business owner goes to "Payments" in OHC.
    *   They see options: "Connect Stripe", "Connect Razorpay", "Connect Mercado Pago".
    *   Selecting one links their OHC account to the chosen payment provider.

2.  **Checkout Experience (User View):**
    *   When a customer clicks "Pay" on an OHC invoice or storefront, OHC checks the active provider for that business.
    *   OHC routes the transaction securely to the selected payment gateway.
    *   The customer sees the appropriate local checkout UI.

3.  **Fulfillment:**
    *   OHC receives a notification from the payment gateway that the transaction was successful.
    *   OHC marks the invoice as Paid, and triggers fulfillment.

## Implementation Prompt
**Objective:** Refactor the payment subsystem to support multiple payment gateways, laying the groundwork for regional providers.

**Acceptance Criteria:**
1.  Establish a standardized mechanism to handle payment session creation and webhook processing across different providers.
2.  Create a configuration model to store tenant-specific credentials for different gateways.
3.  Implement a mock payment provider for local testing.
4.  Refactor the existing checkout flow to dynamically resolve the payment provider based on the tenant's configuration.

## Priority
P1

## Estimated Scope
Large
