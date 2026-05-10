# Payment Processing: Mercado Pago (LATAM focus)

## Problem Statement
Stripe doesn't support many countries in Latin America well. Business owners in these regions need a localized payment gateway that supports local currency, installments, and local payment methods (like Pix in Brazil or OXXO in Mexico).

## Research Report
Mercado Pago is the dominant player in LATAM e-commerce.
- *Ease of Use*: Simple checkout integration, widely recognized by local consumers.
- *Pricing*: Variable by country, usually a percentage + flat fee per transaction.
- *Reputation*: Highly trusted in LATAM, excellent support for local payment methods.

## Design Doc
- *Trigger*: User selects "Mercado Pago" as their payment provider in OHC Store Settings.
- *Action*: OHC generates Mercado Pago checkout preferences and redirects buyers to their secure checkout page, then listens for IPN (Instant Payment Notification) webhooks.
- *User Interface*: In Settings -> Payments, an option to connect Mercado Pago. In the storefront, the checkout button says "Pagar con Mercado Pago".

## Implementation Prompt
Implement Mercado Pago as an alternative payment gateway. Users should be able to connect their account via access tokens. The storefront checkout must be able to generate a Mercado Pago payment link and handle the return webhook to mark orders as paid.

## Priority
P1

## Estimated Scope
Medium

## Environment Support
Cloud, Standalone.
