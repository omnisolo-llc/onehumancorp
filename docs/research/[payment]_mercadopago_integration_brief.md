# Mercado Pago Integration

## Problem Statement
Small business owners in Latin America (LATAM) often cannot use Stripe due to local banking restrictions, currency conversion fees, or the preference of their customers for local payment methods like PIX in Brazil. When a business owner uses OHC to handle online orders, they need a payment processor that seamlessly integrates with local realities, ensuring high conversion rates and fast settlement times to their local bank accounts.

## Research Report
Mercado Pago is the dominant fintech and payment processor in LATAM, deeply integrated into the local e-commerce ecosystem.
- **Benefits for Users:** Allows businesses to accept local payment methods (PIX, Boleto, local credit cards) instantly. Settlement times are often faster than international providers.
- **Ease of Use:** Familiar to almost all LATAM users. The setup involves linking an existing Mercado Pago account.
- **Reputation:** Highly trusted in the region, practically ubiquitous in countries like Argentina, Brazil, and Mexico.
- **Pricing:** Typically around 3% to 5% per transaction depending on the country and settlement speed selected by the user. There are no monthly fees.
- **Environment Compatibility:** Fully compatible with both Cloud and Standalone modes via API and webhooks.

## Design Doc
```mermaid
graph TD
    Customer(Customer) -->|Clicks Pay| Checkout[OHC Checkout Page]
    Checkout -->|Generates Payment Intent| OHC_Backend[OHC Backend]
    OHC_Backend -->|API Call| MP[Mercado Pago API]
    MP -->|Returns Payment URL/QR| OHC_Backend
    OHC_Backend -->|Displays UI| Checkout
    Customer -->|Completes Payment| MP
    MP -->|Webhook Notification| OHC_Backend
    OHC_Backend -->|Updates Order Status| DB[(SIPDB / Postgres)]
    OHC_Backend -->|Alerts Owner| OHC_UI[OHC App Interface]

    classDef premium fill:rgba(255,255,255,0.03),stroke:rgba(255,255,255,0.08),stroke-width:1px,color:#fff,backdrop-filter:blur(20px) saturate(200%);
    class Checkout,OHC_Backend,DB,OHC_UI premium;
```

OHC generates a Mercado Pago payment link or QR code (crucial for PIX). The customer completes the transaction securely on Mercado Pago's infrastructure. Mercado Pago fires a webhook to OHC, confirming the payment, and OHC updates the order status, alerting the business owner.

## Implementation Prompt
Integrate Mercado Pago as an alternative payment provider for OHC checkouts.
- **User Outcome:** Business owners in LATAM can select Mercado Pago in their payment settings. Customers checking out via OHC will see localized payment options (like PIX) and pay in their local currency.
- **Acceptance Criteria:**
  - Secure integration with the Mercado Pago Checkout Pro or API.
  - Webhook listener to process asynchronous payment confirmations.
  - Support for generating QR codes for instant payments (e.g., PIX).
  - Premium checkout UI styling adhering to OHC design standards.

## Priority
P2

## Estimated Scope
Medium
