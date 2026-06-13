# Playwright E2E Test Outline for POS/In-Person Payments

## Business Context
- **Persona:** Carlos (Freelance Handyman) / Priya (Boutique Owner)
- **Goal:** They have a physical product or service and need to accept a payment in person. They use their phone/desktop OHC app to ring up the item, generate a Stripe Terminal token, and complete a payment intent.
- **Verification:** The transaction completes successfully, reflects in their unified financial dashboard, and deducts inventory (if physical).

## Test Case 1: End-to-End In-Person Payment Flow
1. **Login:** Log in as Carlos (or Priya) on the OHC app.
2. **Navigate:** Go to the "Operations" dashboard -> "POS / In-Person" tab.
3. **Select Product:** Add a product/service to the cart (e.g., "Plumbing Fix" - $50.00).
4. **Initiate Payment:** Click the "Charge $50.00" button.
5. **Verify Request:** Ensure the UI calls `/api/v1/payments/terminal/token` to get a connection token.
6. **Simulated Reader Flow:** Simulate a Stripe Terminal tap (since we simulate Stripe SDKs, we verify the `create_payment_intent` call).
7. **Complete Intent:** Confirm `/api/v1/payments/terminal/intent` is called with correct `amount_cents: 5000` and `currency: usd`.
8. **Final State:** Assert the UI shows a "Payment Successful" screen.
9. **Dashboard Verification:** Navigate to "Finance & Payments" -> verify the $50 transaction appears in the daily report.
