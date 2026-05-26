issue_title: "Zero-Touch Autonomous Bookkeeping Engine Architecture"
issue_description: |
  # Zero-Touch Autonomous Bookkeeping Engine Architecture

  ## Problem Statement
  Small business owners dread accounting. Managing receipts, categorizing expenses, and calculating tax liabilities requires manual data entry or expensive bookkeeping services. The non-technical business owner needs an invisible process where taking a picture of a receipt or receiving an emailed invoice is enough to completely process and categorize the expense.

  ## Research Report
  - **Market Context**: Most modern accounting solutions are designed for accountants or dedicated administrative staff, expecting manual matching and categorization.
  - **Competitor Analysis**: Products like Shopify offer high-level revenue analytics, while Stripe provides basic tax and ledgering, but neither inherently act as a full, receipt-first automated bookkeeper for field and service workers without external integrations.
  - **Opportunity**: Embedding a localized LLM/VLM agent to process images of receipts, invoices, and bank feed data to automatically categorize expenses against standard chart of accounts, predict tax deductions, and store them directly into our existing `Ledger` capability without user intervention.

  ## Implementation Prompt
  Implement the backend capability for the Zero-Touch Autonomous Bookkeeping Engine.
  1. Create the endpoint and storage mechanism for mobile clients to upload receipt images securely.
  2. Integrate the VLM prompt pipeline to process uploaded images, extract structured data (Vendor, Amount, Tax, Date, Category), and handle errors or low-confidence extractions.
  3. Connect the successful extraction event directly to the multi-tenant `Ledger` to automatically record the expense transaction.
  4. Ensure all database interactions adhere strictly to our multi-tenant isolation rules.
  5. Create the "Finance AI" worker queue to handle processing asynchronously so the mobile client isn't blocked waiting for the VLM response.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
