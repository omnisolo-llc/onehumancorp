**Title**: Mercado Pago Integration for LATAM Payments
**Problem Statement**: While Stripe is great, it doesn't support many local payment methods essential in Latin America (like Pix in Brazil or OXXO in Mexico). Business owners in these regions lose sales because customers cannot pay with their preferred local methods.
**Research Report**: Mercado Pago dominates the LATAM market. It supports a vast array of local payment options, settles relatively quickly, and provides a simple checkout experience. Fees vary by country but are competitive for the region. The brand is highly trusted by consumers in LATAM.
**Design Doc**:
- **Trigger**: Customer proceeds to checkout for an invoice or product.
- **Action**: OHC generates a Mercado Pago checkout link or renders their checkout widget.
- **User Experience**: The business owner connects their Mercado Pago account with one click. Their LATAM customers see localized payment options at checkout, and the invoice is automatically marked "Paid" in OHC upon success.
**Implementation Prompt**: Integrate Mercado Pago as an alternative payment provider to Stripe. Build a connection flow for the business owner. When a customer views an OHC invoice, they should be able to pay via Mercado Pago, and the OHC system should listen for the payment success webhook to update the invoice status.
**Priority**: P1
**Estimated Scope**: Large
**Environment**: Works in Cloud; Standalone requires webhook forwarding setup.
