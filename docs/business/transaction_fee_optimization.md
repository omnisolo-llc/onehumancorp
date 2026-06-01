<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# Transaction Fee Optimization: ACH Routing

**Author(s):** Principal Cost Engineer
**Status:** Approved
**Last Updated:** 2026-06-01

## 1. Overview
This document outlines the strategic shift in payment routing to minimize transaction fees on the OHC platform. By intelligently routing high-value transactions away from standard credit card processing and towards ACH (Automated Clearing House) Bank Transfers, we significantly reduce the percentage-based fees levied by our payment processor (Stripe).

## 2. The Problem
Standard credit card processing via Stripe incurs a fee of **2.9% + 30¢** per successful card charge. For small transactions (e.g., a $15 booking), this fee is negligible. However, for high-value transactions (e.g., a $5,000 custom service package or B2B invoice), the percentage fee becomes a substantial cost drain ($145.30 fee).

## 3. The Solution: ACH Bank Transfers
Stripe offers ACH Direct Debit payments, which carry a significantly lower fee structure: **0.8%**, capped at **$5.00** per transaction.

### 3.1 Cost Savings Analysis
Let's compare the fees for a $5,000 transaction:
- **Credit Card (2.9% + 30¢):** $145.30
- **ACH Bank Transfer (0.8% capped at $5):** $5.00

**Total Savings per $5,000 transaction: $140.30**

This represents a massive margin improvement for both OHC (when billing for premium/enterprise tiers) and our merchants (when billing their end-customers for high-ticket items).

## 4. Implementation details
- **Checkout UI Update:** The standard checkout flow (both in the Tauri application and the Next.js web application) now prominently features a "Pay with Bank Transfer (ACH)" option.
- **Smart Routing (Future):** In the future, the checkout engine will dynamically highlight or exclusively offer ACH for transactions exceeding a certain threshold (e.g., > $1,000).

## 5. Trade-offs & Considerations
- **Settlement Time:** ACH transfers typically take longer to settle (3-5 business days) compared to the near-instant authorization of credit cards. This must be clearly communicated to merchants fulfilling physical goods.
- **Dispute Process:** ACH dispute processes differ from credit card chargebacks. Our Legal & Compliance AI Agent handles the generation of appropriate terms to mitigate ACH return risks.

</div>
