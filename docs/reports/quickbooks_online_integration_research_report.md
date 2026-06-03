# QuickBooks Online Integration Research Report

## Problem Statement
Small business owners like Priya (Boutique Owner) and Carlos (Handyman) spend hours every week manually copying sales data and expenses from various platforms into their accounting software or manually exporting sales data, refunds, and tax collections into CSV files to send to their accountant. This manual data entry is error-prone, tedious, and often results in miscategorized expenses or missing revenue records. They need an automated "set-it-and-forget-it" way to sync OHC transactions directly into their QuickBooks Online account, automating bookkeeping and tax preparation.

## Competitive Analysis
*   **Strategy**: Direct API integration with the QuickBooks Online Accounting API.
*   **Target Persona**: Priya (Boutique), Carlos (Handyman), Maya (Home Baker).
*   **Advantages**: QuickBooks Online is the dominant accounting software for small businesses in the US and globally. Automating data entry removes one of the biggest administrative burdens from the business owner, reinforcing OHC's value proposition of "Radical Simplicity".
*   **Risks**: The QuickBooks Online API requires robust mapping of income accounts, tax agencies, and item categories, which can be complex to abstract away from the user. OAuth token refresh management is strict.
*   **Pricing**: The API integration is free for developers. Business owners must have an active QuickBooks Online subscription.
*   **Compatibility**: Cloud (Centralized OAuth). Standalone (API Key or Local OAuth flow, requiring a cloud broker for OAuth callbacks but data can sync locally).

## System Design
- **Trigger**: The sync can run on a nightly cron schedule, or trigger instantly upon specific business events (e.g., an order is Paid, Refunded, or a Payout is deposited).
- **Integration Flow**:
  - **Setup**: The business owner navigates to "Finance & Payments" or "Integrations" dashboard in OHC and clicks "Connect QuickBooks". They go through the standard Intuit OAuth 2.0 flow to authorize OHC.
  - **Configuration (Simplified)**: The Finance AI Agent asks a few plain-language questions to map OHC data to their QuickBooks chart of accounts (e.g., "Where should we record sales income?", "Which account holds your collected taxes?"). OHC automatically maps standard accounts (e.g., Sales, Inventory, Processing Fees).
  - **Execution**: When a new transaction occurs (e.g., Priya sells a dress, Carlos finishes a job and gets paid via Tap-to-Pay), OHC creates a "Sales Receipt" or "Invoice" and corresponding "Payment" in QuickBooks. It automatically records any discounts, shipping fees, and sales taxes. Daily summaries or batch jobs can also sync inventory levels and payouts.
  - **Visibility**: The Finance AI Agent includes a brief note in its weekly report, e.g., "I've successfully synced 14 transactions to QuickBooks this week" or alerts the business owner in plain language if an error occurs (e.g., "Hey Priya, your QuickBooks sync paused because your subscription expired").

## Implementation Instructions
Build a native integration with the QuickBooks Online API.
1. Implement the OAuth 2.0 connection flow for Intuit/QuickBooks Online and securely store the access and refresh tokens.
2. Create a background job system (data synchronization workers) to listen to OHC's order, payment, and refund events and map them into their corresponding QuickBooks entities (SalesReceipt, Payment, RefundReceipt, Invoices).
3. Provide a minimal UI in the Finance settings for the user to connect their account and select their default income and tax accounts. The sync must automatically handle mapping line items and associated taxes without requiring manual user intervention per transaction.
- **Acceptance Criteria**: User can successfully authenticate and connect their QuickBooks Online account. New sales processed in OHC automatically appear as matching transactions in the connected QuickBooks account without manual intervention.
- **Priority**: P1
- **Estimated Scope**: Large
