issue_title: "Integrate QuickBooks Online for Automated Accounting Sync"
issue_description: |
  # Title
  Integrate QuickBooks Online for Automated Accounting Sync

  # Problem Statement
  Owners like Nora (Agency Principal) and Carlos (Field Service Owner) run their daily operations through OHC, generating quotes, sending invoices, and collecting payments. However, when tax season arrives or when they review their monthly P&L, they or their bookkeepers are forced to manually enter OHC transactions into their accounting software. This double data entry is time-consuming, prone to human error, and delays financial visibility. They need OHC to automatically push completed sales, invoices, and expenses directly into their accounting system so their books are always up to date without lifting a finger.

  # Research Report
  - **Market Context**: In competitor ecosystems (HubSpot Marketplace, Square App Marketplace, Wix App Market), QuickBooks Online (QBO) is consistently one of the top 3 most installed and requested integrations. Over 70% of small businesses in the US rely on QuickBooks for their accounting.
  - **Tool Evaluation (QuickBooks Online API)**: Intuit provides a mature REST API for QBO with OAuth 2.0 authentication. It allows creating invoices, sales receipts, payments, and customers.
  - **Ease of Use for Owners**: The setup process for a non-technical user involves clicking "Connect to QuickBooks", signing into their Intuit account, and authorizing OHC. After this one-time OAuth flow, sync happens transparently in the background.
  - **Pricing & Reputation**: QBO API access is free for developers, though production access requires an app approval process. QBO itself is a paid SaaS product for the owner, widely trusted by accountants and bookkeepers.

  # Design Doc
  - **Triggers**: When an invoice is marked as "paid" or a Stripe payment succeeds in OHC, a background job is triggered to sync this transaction to QBO. Similarly, when a new client accepts a proposal, the customer record can be synced.
  - **User Experience**: The owner will see a new "Accounting" section in the OHC Settings. They click "Connect to QuickBooks" which redirects them to Intuit's secure login. Once connected, OHC displays a simple toggle: "Sync payments and invoices automatically."
  - **Agent Interaction**: The Finance & Decision Assistant can query sync status and let the owner know if an invoice failed to sync to QBO (e.g., due to an expired connection), prompting them to reconnect.

  # Implementation Prompt
  - Create the OAuth 2.0 authorization flow connecting an OHC tenant to a QuickBooks Online company.
  - Build a background worker that listens to OHC invoice and payment events and pushes them to QBO as Sales Receipts or Invoices/Payments.
  - Add a UI settings page in the Flutter/Next frontend where the owner can initiate the QBO connection and view the connection status.
  - Ensure any transient API errors (e.g., rate limits) from Intuit are retried gracefully, and persistent errors are bubbled up to the Finance Assistant for owner notification.
  - Acceptance Criteria: A non-technical owner can connect their QBO account via OAuth. When they collect a payment in OHC, a corresponding record appears in their QBO ledger automatically within a few minutes.

  # Priority
  P1

  # Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
