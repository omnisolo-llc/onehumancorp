**Title**: Mercado Pago Integration for LATAM Payment Processing

**Problem Statement**:
Small business owners in Latin America (especially Mexico, Brazil, Argentina) cannot effectively use Stripe, which has limited presence or higher barriers in these regions. They need a local, trusted payment provider to accept credit cards, debit cards, bank transfers, and cash payments (like OXXO in Mexico or PIX in Brazil) to sell their services online.

**Research Report**:
- **Tool**: Mercado Pago (Payment Processing).
- **Ease of Use**: High. It is the dominant payment platform in Latin America, familiar to both merchants and consumers.
- **Pricing**: Pay-as-you-go per transaction. For Mexico, fees typically range from 3.49% to 4.49% + a small fixed fee per transaction for cards. No monthly fees.
- **Reputation**: Highly trusted, backed by Mercado Libre.
- **Compatibility**: Works well via API for both Cloud and Standalone modes.

**Design Doc**:
- **Trigger**: Business owner generates an invoice or payment link in OHC.
- **Action**: OHC calls the Mercado Pago API to create a checkout preference and generates a payment link.
- **User Interface**: When sending an invoice to a customer, the owner can select "Mercado Pago" as the payment method. The customer receives a link to a secure Mercado Pago checkout page. Once paid, the invoice status in OHC automatically updates to "Paid".
- **Integration Flow**: User enters their Mercado Pago Access Token and Public Key in the OHC Settings -> Payments section.

**Implementation Prompt**:
Add Mercado Pago as an alternative payment gateway alongside the existing payment options. Allow business owners to enter their Mercado Pago API credentials. Update the invoicing module to generate Mercado Pago checkout links. Implement a webhook listener to automatically mark invoices as paid when Mercado Pago confirms the transaction.

**Priority**: P1
**Estimated Scope**: Medium
