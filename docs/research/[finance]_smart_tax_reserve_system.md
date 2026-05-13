# Smart Tax Reserve System

## Problem Statement
Independent contractors, freelancers, and solo-entrepreneurs frequently fail to save adequately for their quarterly estimated taxes. Because business revenue is often mixed with personal operating funds, they inadvertently spend their tax liability, leading to massive financial stress and potential IRS penalties at tax time.

## Research Report
A major, recurring pain point identified in our deep-dive persona research (particularly for service providers like 'Carlos the Handyman') is profound tax anxiety. While specialized platforms like Catch cater to this specific need, they exist entirely separate from the user's core business operating system, requiring the user to manage yet another application. Integrating this directly into the primary revenue stream provides massive utility.

## Design Doc
### Architecture Vision
- **Entities**: BankAccountConnection, IncomingTransaction, ReserveGoal, TransferLog.
- **UX Flow**:
  1. During financial setup, the user securely links their primary business bank account.
  2. Every time an incoming payment clears from a client, the system automatically calculates the estimated tax liability (e.g., a flat 20% or a dynamic rate based on year-to-date income).
  3. The system visually segregates this calculated money within the UI, or, ideally, initiates an automated ACH transfer to a dedicated, separate reserve account.
- **Mobile UX**: The primary financial dashboard prominently displays a 'Safe to Spend' metric, actively subtracting the reserved tax liability from the raw bank balance to prevent overspending.
- **Agent Integration**: The Accountant Agent manages the complex calculations based on local tax codes and securely orchestrates the automated funds transfers.

## Implementation Prompt
**Outcome**: Construct a financial safety system that automatically calculates and isolates tax liabilities from incoming revenue streams in real-time.
**Critical User Journey**:
1. The user gets paid a $1000 invoice for a completed job.
2. The system instantly calculates and reserves $200 for estimated taxes.
3. The user looks at their dashboard and sees 'Safe to Spend: $800', preventing them from accidentally spending money owed to the government.
**Acceptance Criteria**: The system must accurately estimate self-employment and standard income tax based on the user's localized rates. It must provide clear, undeniable visibility into what funds are reserved versus what is liquid.

## Priority
P1

## Estimated Scope
Large
