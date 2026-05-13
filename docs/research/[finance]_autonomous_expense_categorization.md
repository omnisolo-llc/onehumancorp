# Autonomous Expense Categorization Engine

## Problem Statement
Tax season represents a period of extreme anxiety and administrative nightmare for SMB owners. They frequently dump physical receipts into a shoebox or improperly mix personal and business expenses on a single credit card. Reconciling this chaotic data takes days of manual effort and often requires hiring an expensive freelance bookkeeper.

## Research Report
Platforms like QuickBooks Online are incredibly powerful, but they are fundamentally designed for use by trained accountants, not tradespeople. Our target SMB owners desperately need an 'invisible bookkeeper'. Research indicates that over 60% of very small businesses do not use dedicated accounting software at all; they simply hand their raw bank statements to a CPA at year-end. By integrating securely via banking APIs like Plaid and applying a specialized LLM to categorize transactions automatically, we can realistically save them 40 hours of administrative labor annually.

## Design Doc
### Architecture Vision
- **Entities**: BankTransaction, ExpenseCategory, VendorProfile, ReconciliationRule.
- **UX Flow**:
  1. The user securely connects their primary business bank account via Plaid during the initial onboarding flow.
  2. A background agent securely syncs new transactions on a daily basis.
  3. The LLM analyzes the raw vendor name string and the transaction amount, accurately categorizing it (e.g., mapping 'Home Depot #4432' to 'Supplies', or 'Mailchimp' to 'Software Subscriptions').
  4. The user periodically reviews any unconfident or ambiguous categorizations in a highly simplified swipe interface (functioning similarly to Tinder, but for expense confirmation).
- **Mobile UX**: A clean, distraction-free list view featuring swipe gestures (swipe right to confirm, swipe left to edit category) to rapidly process unverified transactions.
- **Agent Integration**: An Accountant Agent processes incoming webhooks from Plaid and queries the secure LLM service for precise categorization inference.

## Implementation Prompt
**Outcome**: Build a robust system that automatically ingests and categorizes bank transactions, preparing a clean, tax-ready financial report without requiring tedious manual data entry from the user.
**Critical User Journey**:
1. The user securely links their banking institution.
2. The system accurately categorizes 95% of the past month's expenses automatically in the background.
3. The user quickly swipes through the interface to confirm the remaining 5% of ambiguous transactions.
**Acceptance Criteria**: The system must seamlessly support exporting the categorized data to standard CSV formats utilized by CPAs. It must achieve a baseline >90% categorization accuracy rate on common SMB vendors.

## Priority
P1

## Estimated Scope
Large
