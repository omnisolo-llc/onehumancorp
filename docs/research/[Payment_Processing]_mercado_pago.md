# Mercado Pago Integration for LATAM Payments

## Title
Enable Payments via Mercado Pago

## Problem Statement
For small businesses operating in Latin America, standard payment gateways like Stripe are often unavailable, expensive, or lack support for local payment methods (like Pix in Brazil). Business owners need a trusted, localized payment processor to easily send invoices and collect payments from their customers without friction.

## Research Report
Mercado Pago is the fintech and payment processing arm of Mercado Libre, the largest e-commerce platform in Latin America. According to Wikipedia, Mercado Libre operates in numerous LATAM countries, including Argentina, Brazil, Mexico, Colombia, and Chile, making Mercado Pago widely recognized and trusted in the region.

It acts as a vital alternative to global providers by supporting local currencies and payment methods preferred by LATAM consumers. The primary advantage is regional ubiquity and consumer trust. Pricing typically involves a percentage per transaction, varying by country. Integrating Mercado Pago ensures OHC is viable for small businesses across South and Central America. It is fully compatible with Cloud and Standalone environments through standard REST APIs.

## Design Doc
A business owner in a supported LATAM country can select Mercado Pago as their primary payment gateway in OHC settings. When they generate an invoice in OHC, the system will generate a Mercado Pago checkout link. The customer receives the invoice, clicks the link, and pays using their preferred local method. Once the payment is successful, Mercado Pago sends a notification back to OHC, automatically marking the invoice as "Paid" and notifying the business owner.

## Implementation Prompt
Integrate Mercado Pago as a payment provider option alongside existing gateways. Allow users to input their Mercado Pago credentials. When an invoice is created, generate a Mercado Pago payment link and embed it in the invoice email. Implement webhook listeners to automatically update the invoice status to "Paid" upon successful transaction completion.

## Priority
P2

## Estimated Scope
Medium
