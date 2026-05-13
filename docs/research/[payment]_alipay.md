# Title: Alipay Integration for Cross-Border Payments

## Problem Statement
For businesses selling globally or in specific Asian markets, Stripe and PayPal are often insufficient or have low conversion rates. Customers prefer local payment methods. A business owner needs a simple way to accept Alipay to capture sales from Chinese consumers without navigating complex international banking setups.

## Research Report
**Market Analysis & Pain Points:**
- **Friction:** Setting up merchant accounts in foreign jurisdictions is impossible for most SMBs.
- **The Tool:** Alipay (via cross-border acquiring partners or direct integrations) dominates the Chinese market.
- **Reputation & Ease of Use:** While the API is robust, direct integration is complex. However, integrating via an aggregator (like Stripe's Alipay integration or Adyen) simplifies this. For this research, we evaluate direct/specialized Alipay cross-border APIs.
- **Pricing:** Typically a percentage per transaction, often lower than international credit card fees.

**Key Advantages:**
- Opens up a massive new customer base for the business owner.
- High trust factor for specific demographics.

**Integration Risks:**
- Complex KYC (Know Your Customer) requirements for the merchant to get approved.
- Currency conversion and settlement delays.

**Environment Support:**
- **Cloud:** Full support via API.
- **Standalone:** Supported, assuming internet connectivity for the transaction.

## Design Doc
**Trigger:**
User enables "Alipay" in their "Payments" settings and completes the required merchant onboarding flow.

**Action:**
When a customer checks out on an OHC-hosted storefront, they can select Alipay. OHC generates a payment QR code or redirects them to the Alipay app.

**User View:**
The business owner simply sees "Alipay" as an active payment method. In their ledger, Alipay transactions appear seamlessly alongside credit card transactions, converted to their local currency based on the settlement data.

## Implementation Prompt
Add Alipay as a payment method for OHC storefronts.
- Integrate an Alipay-compatible checkout flow (QR code generation and mobile redirect).
- Build the merchant onboarding UI to collect necessary KYC details for Alipay cross-border approval.
- Ensure Alipay transactions are correctly represented in the OHC order management and accounting ledger views.
- (Do not prescribe specific database schemas; ensure the user experience of checking out and viewing the ledger is seamless.)

## Priority
P3

## Estimated Scope
Large
