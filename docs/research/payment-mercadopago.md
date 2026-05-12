# Accept Local Payments in LATAM via Mercado Pago

**Problem Statement**
Many of my customers in Latin America want to pay using local methods like Pix or Boleto. Standard credit card processors like Stripe don't support these well, and my customers abandon their carts when they can't pay their preferred way.

**Research Report**
Mercado Pago is the dominant payment processor in Latin America, supporting local payment methods like Pix (Brazil), Boleto, and local debit cards. Settlement speeds are fast, often instant for Pix. It is highly trusted by consumers in the region. Pricing is transaction-based (e.g., around 4-5% depending on the country and settlement time). It works securely in Cloud and Standalone via secure API keys and webhooks.

**Design Doc**
Users operating in supported LATAM countries will have an option to enable Mercado Pago in their billing settings. This will allow them to generate payment links or display Mercado Pago checkout options on their invoices. When a customer pays, the invoice status in OHC will automatically update to 'Paid'.

**Implementation Prompt**
Integrate Mercado Pago as a payment provider option. Allow users to connect their Mercado Pago account via API keys. Ensure that invoices generated in OHC can be paid via a Mercado Pago link, and that successful payments update the invoice status automatically. Acceptance criteria: A payment link can be generated, and a test payment successfully marks the invoice as paid.

**Priority:** P2
**Estimated Scope:** Medium
