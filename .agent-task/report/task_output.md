issue_title: "Native QuickBooks Online Integration for Automated Accounting Sync"
issue_description: |
  **Problem Statement**
  Small business owners like Priya (Boutique) and Carlos (Handyman) struggle with bookkeeping and tax preparation. They rely on manually exporting sales data, refunds, and tax collections into CSV files to send to their accountant or manually enter into QuickBooks. This error-prone process takes hours each month and often results in miscategorized expenses or missing revenue records. They need an automated "set-it-and-forget-it" way to sync OHC transactions directly into their QuickBooks Online account.

  **Research Report**
  - **Strategy**: Direct API integration with the QuickBooks Online Accounting API.
  - **Target Persona**: Priya (Boutique), Carlos (Handyman), Maya (Home Baker).
  - **Advantages**: QuickBooks Online is the dominant accounting software for small businesses in the US and globally. Automating data entry removes one of the biggest administrative burdens from the business owner, reinforcing OHC's value proposition of "Radical Simplicity".
  - **Risks**: The QuickBooks Online API requires robust mapping of income accounts, tax agencies, and item categories, which can be complex to abstract away from the user. Oauth token refresh management is strict.
  - **Pricing**: The API integration is free for developers. Business owners must have an active QuickBooks Online subscription.
  - **Compatibility**: Cloud (Centralized OAuth). Standalone (API Key or Local OAuth flow).

  **Design Doc**
  - **Trigger**: The sync can run on a nightly cron schedule, or trigger instantly upon specific business events (e.g., an order is Paid, Refunded, or a Payout is deposited).
  - **Integration Flow**:
    - **Setup**: The business owner navigates to "Finance & Payments" settings in OHC and clicks "Connect QuickBooks". They go through the standard Intuit OAuth flow.
    - **Configuration (Simplified)**: The Finance AI Agent asks a few plain-language questions to map OHC data to their QuickBooks chart of accounts (e.g., "Where should we record sales income?", "Which account holds your collected taxes?").
    - **Execution**: When a transaction occurs (like a customer paying an invoice), OHC creates a "Sales Receipt" or "Invoice" and corresponding "Payment" in QuickBooks. It automatically records any discounts, shipping fees, and sales taxes.
    - **Visibility**: The Finance AI Agent includes a brief note in its weekly report, e.g., "I've successfully synced 14 transactions to QuickBooks this week."

  **Implementation Prompt**
  Build a native integration with the QuickBooks Online API. Implement the OAuth2 connection flow and securely store the access and refresh tokens. Create a background job system that maps OHC order, payment, and refund events into their corresponding QuickBooks entities (SalesReceipt, Payment, RefundReceipt). Provide a minimal UI in the Finance settings for the user to connect their account and select their default income and tax accounts. The sync must automatically handle mapping line items and associated taxes without requiring manual user intervention per transaction.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
