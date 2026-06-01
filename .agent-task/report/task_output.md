issue_title: "Implement Autonomous AI Zero-Touch Receipt and Expense Intelligence Engine"
issue_description: |
  # Autonomous AI Zero-Touch Receipt and Expense Intelligence Engine

  Small business owners dread tracking expenses, managing receipts, and estimating taxes. They need an automated, invisible bookkeeping engine that acts as a virtual CFO.

  ## Problem Statement
  Small business owners consistently struggle with the administrative burden of tracking receipts, logging expenses, and maintaining financial records. The "Automation Expectation" trend highlights that anything that can be automated, must be. This engine will automatically ingest, process, and categorize receipts through omnichannel inputs without requiring manual intervention from the business owner.

  ## Research & Market Gap
  - **QuickBooks Self-Employed**: Requires manual sorting (Tinder for expenses). Too much friction.
  - **Shopify/Wix**: Track COGS manually; don't handle general business expenses (gas, tools, software).
  - **Stripe / Square**: Good at transaction processing and payouts, but they do not automatically handle receipt OCR or expense categorization.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ EXPENSE_RECORD : "has"
      EXPENSE_RECORD {
          string id PK
          string tenant_id FK
          float amount
          string vendor
          string category
          datetime date
      }
      RECEIPT_INGESTION ||--o| EXPENSE_RECORD : "generates"
  ```

  ### Mobile UX Flow (375px First)
  1. **Expense Capture:** The user snaps a picture of a receipt (or forwards an email).
  2. **Invisible Processing:** A bottom sheet slides up briefly: "AI is categorizing your $45.20 expense at Home Depot... Done. Marked as 'Supplies'."
  3. **Dashboard Update:** The dashboard simple "Money Out" and "Tax to Save" metrics update immediately.

  ### AI Agent Integration
  - **Finance & Payments ("The Accountant"):** Triggers when a photo is uploaded. Uses LLMs (multimodal vision) to infer the business purpose and categorize the expense (e.g., "Home Depot" + handyman profile = "Job Materials/Supplies").

  ### Performance & Security
  - **Zero-Trust Isolation:** Expenses and Receipts are strictly partitioned by `tenant_id`.
  - **Offline Tolerance:** Draft receipts captured offline are queued locally and synced when connection is restored.
  - **Latency:** Receipt parsing and categorization must complete within 5 seconds.

  ## Implementation Prompt
  Implement the Autonomous AI Zero-Touch Receipt and Expense Intelligence Engine. Build the robust backend data models (`Receipt`, `ExpenseRecord`, `ExpenseCategory`). Implement the asynchronous ingestion pipeline where an uploaded image triggers a multimodal AI task to parse the receipt. Develop logic to categorize it according to tax-friendly buckets and recalculate real-time P&L. Ensure all operations are strictly tenant-isolated and fail gracefully if OCR confidence is low, putting the expense into a "Needs Review" queue. Design the API endpoints to support offline-first mobile sync for image uploads.

  ## Priority
  P0

  ## Estimated Scope
  Large
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
