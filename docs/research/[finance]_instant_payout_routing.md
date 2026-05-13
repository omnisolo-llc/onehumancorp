# Instant Payout Routing Architecture

## Problem Statement
Poor cash flow management is the number one reason small businesses fail. Waiting the standard 2-3 business days for payment processors like Stripe or Square to clear payouts can literally mean the business owner cannot afford to buy necessary supplies for tomorrow's job.

## Research Report
Market data strongly suggests SMBs are highly willing to pay a premium fee (e.g., 1% to 1.5% of the total volume) for instant access to their cleared funds. Providing this feature natively within the OHC ecosystem dramatically increases platform lock-in and overall merchant satisfaction.

## Design Doc
### Architecture Vision
- **Entities**: MerchantLedgerBalance, VerifiedPayoutMethod, InstantTransferRequest.
- **UX Flow**:
  1. A merchant successfully completes a $500 job and the client payment clears.
  2. The merchant immediately taps a prominent 'Get Funds Now' button located on the primary dashboard.
  3. The funds are instantly pushed to their linked business debit card via a supported RTP (Real-Time Payments) network.
- **Mobile UX**: A highly prominent 'Instant Payout' button featuring a clear, unambiguous display of the associated convenience fee.
- **Agent Integration**: The Treasury Agent evaluates real-time fraud risk and securely manages the complex RTP API call.

## Implementation Prompt
**Outcome**: Implement a critical financial feature allowing merchants to instantly transfer their available ledger balance to a debit card for a small convenience fee.
**Critical User Journey**:
1. A merchant urgently requires liquid cash for supplies or payroll.
2. The merchant initiates an instant payout request via the mobile app.
3. The funds successfully arrive in their connected bank account within seconds.
**Acceptance Criteria**: The architecture must deeply integrate with a specialized financial provider that explicitly supports Visa Direct or Mastercard Send networks. The UI must clearly and legally display all associated fees before execution.

## Priority
P1

## Estimated Scope
Medium
