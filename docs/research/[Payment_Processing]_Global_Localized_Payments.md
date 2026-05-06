## [Payment Processing] Global Localized Payments
**Title**: Integrate Mercado Pago & Alipay for Local Market Penetration

**Problem Statement**: While Stripe is excellent in the US and Europe, business owners in LATAM, India, or Asia struggle with high fees, lack of local payment methods (like PIX or Alipay), or lack of support altogether. They need payment processors that their local customers actually use.

**Research Report**:
- **Persona Context**: Merchants in emerging markets (e.g., Brazil, Mexico, China) who sell online or via chat and need to collect payments seamlessly.
- **Solution Evaluated**: Mercado Pago for LATAM, Alipay for China. Both dominate their respective markets.
- **Ease of Use**: For the business owner, it's a standard OAuth or API key setup. For their customers, it's the familiar, trusted local checkout experience.
- **Advantages**: Drastically increases conversion rates in these regions. Faster local settlement.
- **Risks**: Fragmented API designs. Handling currency conversions and localized dispute/refund processes can be complex.
- **Pricing Estimate**: Standard local transaction fees (varies by region, typically 1.5% to 3.5%).
- **Cloud/Standalone Support**: Fully supported in both modes. Standalone users manage their own API credentials.

**Design Doc**:
- **Triggers**: A user generates an invoice or payment link in OHC.
- **Actions**: OHC calls the respective payment gateway to generate a localized checkout URL or QR code.
- **User Interface**: In settings, users can enable local payment methods alongside or instead of Stripe. When creating a bill, they can generate a Mercado Pago payment link or an Alipay QR code to send to the customer.

**Implementation Prompt**:
Add support for generating payment links via Mercado Pago and Alipay. Users must be able to configure these providers in their billing settings. When they create an invoice, allow them to choose the provider and generate a shareable checkout link or QR code for their customers.

**Priority**: P1
**Estimated Scope**: Large
