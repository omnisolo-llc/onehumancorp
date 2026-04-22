# Mercado Pago Integration
## Problem Statement
Small business owners operating in LATAM markets need a reliable, localized payment processor to accept payments online (for physical goods, digital products, services, or bookings) as Stripe might not be the preferred or supported option in their specific country (e.g., Argentina, Brazil, Mexico, Colombia, Chile).

## Research Report
**Tool**: Mercado Pago
**Ease of use**: High. Provides SDKs, APIs, and pre-built checkout UI components for seamless integration.
**Pricing**: Transaction fees vary by country and settlement speed (e.g., typically between 3% - 6% + fixed fee). No monthly setup costs.
**Reputation**: The largest and most trusted payment gateway in Latin America, developed by Mercado Libre. High consumer trust and localized payment methods (e.g., Boleto, Pix, OXXO, PSE, credit/debit cards).

## Design Doc
**Cloud Mode**: Integrate via Mercado Pago API. OHC platform processes checkouts by generating a Mercado Pago preference and redirecting the user to the Mercado Pago checkout or embedding the Web Tokenize Checkout.
**Standalone Mode**: Online payments require internet connectivity to reach the Mercado Pago API. If operating a physical POS (Point of Sale), Mercado Pago offers Point devices (card readers) and QR codes that can be integrated via API.
**Triggers**: Customer initiates checkout for a product/service, or an invoice is sent.
**User Experience**: During checkout, customers in LATAM select Mercado Pago and are presented with localized payment options (Pix, Boleto, local credit cards, installments). Once paid, OHC automatically updates the order status via Mercado Pago webhooks.

## Implementation Prompt
Integrate Mercado Pago into the OHC platform as an alternative payment gateway for LATAM users.
**Acceptance Criteria**:
1. Business owners can connect their Mercado Pago account via OAuth from the OHC dashboard.
2. Provide a "Pay with Mercado Pago" checkout option for customers on the storefront.
3. Integrate Mercado Pago Checkout Pro or Checkout API to handle payments.
4. Set up webhooks to automatically update order/booking status in OHC when a Mercado Pago payment succeeds, fails, or is refunded.

## Priority
P2

## Estimated Scope
Medium
