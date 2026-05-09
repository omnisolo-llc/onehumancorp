# Integration Issue Brief: Payment Processing (Stripe / Mercado Pago)

## Title
Global & LATAM Payment Processing: Stripe & Mercado Pago

## Problem Statement
Small business owners need to get paid quickly and reliably for their services or products. Setting up traditional merchant accounts is complex and time-consuming. They need a simple way to generate payment links or invoices that customers can pay using local, familiar payment methods (e.g., credit cards globally, or specific local methods in Latin America).

## Research Report
*   **Tools Evaluated**: Stripe (Global) and Mercado Pago (LATAM focus).
*   **Ease of Use**: Both offer excellent developer APIs and "no-code" payment link generation for end-users.
*   **Market Position & Reputation**:
    *   **Stripe**: The gold standard globally for developers and online businesses.
    *   **Mercado Pago**: The dominant force in Latin America, essential for reaching customers who prefer local payment methods or installments (meses sin intereses).
*   **Pricing**:
    *   **Stripe**: Typically 2.9% + 30¢ per successful card charge (US pricing). In Mexico, 3.6% + 3 MXN per transaction.
    *   **Mercado Pago**: Varies by country, but generally charges a percentage fee per transaction (comparable to Stripe, often slightly higher but offers local installment options).
*   **Cloud vs. Standalone Compatibility**: Both utilize cloud APIs. OHC can integrate via API keys or OAuth in both Cloud and Standalone modes to generate payment links and listen for webhooks confirming payment.

## Design Doc
*   **Integration Trigger**: The user connects Stripe or Mercado Pago via OAuth in OHC settings.
*   **Action Flow**:
    1.  User creates an invoice or payment request in OHC.
    2.  OHC calls the respective API (Stripe Payment Links or Mercado Pago Checkout) to generate a secure URL.
    3.  User sends the URL to the client.
    4.  Webhook listens for successful payment and updates the invoice status in OHC to "Paid".
*   **User Experience**: The business owner clicks "Request Payment", enters an amount, and instantly gets a link to text/email to the client. They see real-time status updates ("Pending" -> "Paid") within OHC.

## Implementation Prompt
Develop a dual payment processing integration supporting both Stripe and Mercado Pago to cater to global and LATAM users. Create an interface in OHC where users can generate a "Payment Link" by specifying an amount and description. Depending on their connected provider, OHC should call the appropriate API to create the checkout session and return the URL. Implement webhook listeners to capture successful payment events and update the transaction record in OHC's database, notifying the business owner.

## Priority
P0

## Estimated Scope
Large
