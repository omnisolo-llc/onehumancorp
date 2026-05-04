<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Savings Report: Payment Routing Optimization

**Author(s):** Principal Cost Engineer & Miser
**Status:** Approved
**Last Updated:** 2026-03-24

## 1. Overview
The Payment Routing Optimization feature minimizes transaction fees by dynamically selecting the payment method based on the transaction value. It ensures that high-value transactions are routed through ACH instead of Card to avoid percentage-based fees.

## 2. The Problem
Stripe charges a 2.9% + 30¢ fee for card transactions. For high-value transactions (e.g., $1,000+), this fee becomes substantial. E.g., for a $1,000 transaction, the fee is $29.30.

## 3. The Solution
We implemented a dynamic routing mechanism within the `StripeClient::create_payment_intent` and the `Tracker::create_payment_intent` in the Billing Engine.

*   **Logic:**
    *   If `amount_cents >= 100000` (i.e., >= $1,000), the system generates a `pi_ach_test` Payment Intent.
    *   If `amount_cents < 100000`, the system generates a standard `pi_card_test` Payment Intent.

*   **Cost Savings via ACH:**
    *   ACH transactions cost a flat 0.8%, capped at $5.00.
    *   For a $1,000 transaction, the ACH fee is $5.00 (capped).
    *   **Savings:** $29.30 (Card) - $5.00 (ACH) = **$24.30 per $1k transaction.**

## 4. Impact Analysis
This feature provides an immediate financial uplift. For businesses processing invoices for services (e.g., freelance consultants, large catering orders), this automated routing keeps more money in the business owner's pocket.

</div>
