# Integrate Paytm for Regional Payment Processing (India)

## Problem Statement
Small business owners in India need to accept payments quickly and reliably. Global tools like Stripe are not always optimized for the local market, where UPI (Unified Payments Interface) and local wallets are dominant. Owners need a payment solution that their customers already trust and have installed on their phones.

## Research Report
*   **Tool:** Paytm (or Razorpay as an alternative for the Indian market)
*   **Problem Solved:** Processes payments via UPI, local credit/debit cards, and mobile wallets in India.
*   **Ease of Use:** High for the end consumer. For the business owner, account setup requires local KYC (Know Your Customer) compliance, which can be a friction point but is unavoidable.
*   **Pricing:** Generally 1-2% per transaction, competitive for the region. Free for certain UPI transactions.
*   **Reputation:** Ubiquitous in India; highly trusted by consumers.
*   **Environment:** Works well in Cloud mode. Standalone mode works, but webhook callbacks for payment success must be handled carefully if the standalone instance is not publicly accessible.
*   **Advantages:** Native support for UPI (critical for India); fast settlement times.
*   **Risks:** Strict KYC requirements for onboarding; API documentation can sometimes be inconsistent.

## Design Doc
1.  **Trigger:** User selects "India" during setup or chooses to "Add Payment Method" in settings.
2.  **Action:** OHC prompts the user to connect a Paytm Business account or provides a guided flow to create one.
3.  **User Interface:** When generating an invoice or a checkout link, OHC automatically creates a Paytm payment gateway link or a dynamic QR code. The owner can show this QR code to in-person customers or send the link via WhatsApp.
4.  **Reconciliation:** Once a payment is successful, the OHC dashboard immediately marks the corresponding order/invoice as "Paid".

## Implementation Prompt
Implement regional payment processing for the Indian market using Paytm (or Razorpay). The business owner must be able to link their merchant account to OHC. Provide a feature to generate payment links and dynamic QR codes for specific transaction amounts. These links/codes should be easily shareable. The system must listen for payment success webhooks/callbacks and automatically update the status of the relevant order or invoice to "Paid" in the OHC dashboard, giving the owner instant confirmation without needing to check their bank app.

## Priority
P2

## Estimated Scope
Large
