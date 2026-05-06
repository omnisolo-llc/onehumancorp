# [Payment Processing] Mercado Pago Integration

**Title**: Integrate Mercado Pago for LATAM payment processing

**Problem Statement**: While Stripe is excellent for the US and Europe, small business owners in Latin America face different payment landscapes (e.g., preference for local payment methods, installments, QR codes). They need a trusted, localized payment processor to successfully close sales in their region.

**Research Report**: Mercado Pago is the payment platform for Mercado Libre, the largest e-commerce ecosystem in Latin America.
- **Ease of use**: Very high in LATAM. Widely understood by local consumers and merchants. Supports mobile point-of-sale (QR codes).
- **Pricing**: Varies by country, but generally competitive local rates. No monthly setup fees.
- **Reputation**: The undisputed market leader in Latin America (Argentina, Brazil, Mexico, etc.).
- **Cloud/Standalone**: API driven. Standalone requires internet connectivity.

**Design Doc**:
- **Trigger**: Business owner selects their region (e.g., Argentina, Mexico) and connects their Mercado Pago account.
- **Action**: OHC generates Mercado Pago payment links or QR codes for in-person transactions.
- **User Experience**: The user can generate a "Mercado Pago Link" to send via WhatsApp (the dominant chat app in LATAM), allowing customers to pay with local credit cards, debit cards, or cash at local convenience stores.

**Implementation Prompt**: Add Mercado Pago as a payment provider alternative to Stripe for LATAM users. Allow the generation of a simple shareable payment link that can be pasted into WhatsApp chats.

**Priority**: P2
**Estimated Scope**: Medium