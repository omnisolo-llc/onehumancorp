# Title: Mercado Pago Integration for LATAM Payments

## Problem Statement
Small business owners in Latin America (especially Brazil, Argentina, Mexico) cannot rely solely on Stripe due to local payment preferences like Pix (Brazil), Boleto (Brazil), or local installment plans. They need a localized payment gateway to avoid losing customers at checkout.

## Research Report
Mercado Pago is the leading payment processor in Latin America.
- **Ease of use:** Standard API integration. Business owners can connect their account via OAuth.
- **Pricing:** Variable by country, usually ~4% + fixed fee. Very competitive locally.
- **Reputation:** Dominant in LATAM, highly trusted.
- **Key advantages:** Deep market penetration in LATAM, supports local payment methods (Pix, Boleto, installments) natively.
- **Risks:** The API documentation can be inconsistent or poorly translated in some areas. Webhook delays are occasionally reported during peak events.
- **Environment:** Cloud works perfectly. Standalone works as long as webhooks can reach the local network or status can be polled.

## Design Doc
- User goes to "Payments" and connects their Mercado Pago account.
- When generating an invoice or checkout link, Mercado Pago is offered as an option.
- Upon checkout, the customer is redirected to Mercado Pago's secure flow or an inline form.
- Webhooks update the payment status (e.g., "Pending Boleto" -> "Paid") in the OHC dashboard.

## Implementation Prompt
Integrate Mercado Pago as a payment provider. Create an OAuth connection flow. Implement a checkout session that supports local methods like Pix and credit card installments. Ensure webhooks securely update the invoice status from "pending" to "paid".

## Priority
P2

## Estimated Scope
Large
