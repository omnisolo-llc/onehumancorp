issue_title: "[Architecture] Autonomous Expense and Receipt Engine"
issue_description: |
  # [Architecture] Autonomous Expense and Receipt Engine

  ## Problem Statement
  Small business owners—whether they're Maya buying ingredients for cakes, or Carlos purchasing supplies for a job—dread expense tracking. The traditional workflow requires saving crumpled physical receipts, manually transcribing them into bookkeeping software, categorizing them with accounting jargon ("Cost of Goods Sold" vs. "Operating Expenses"), and reconciling them against bank feeds. For OHC users who operate solely from their phones, this is intimidating, time-consuming, and prone to error. They need a "Magic Shoebox": an invisible system where they can simply snap a photo of a receipt or forward an email, and the AI handles data extraction, categorization, ledger entry, and tax document prep automatically.

  ## Research Report

  ### Competitive Landscape
  *   **QuickBooks / Xero / Wave**: These platforms have receipt capture apps, but they still require manual review and categorization mappings that confuse non-accountants. They expect users to understand double-entry bookkeeping.
  *   **Expensify / Dext**: Good standalone receipt OCR tools, but they introduce another subscription and app to juggle. They don't integrate directly with a unified business operating system or directly deduct expenses against specific project quotes or invoices seamlessly.
  *   **Shopify / Wix**: Primarily focused on revenue and storefronts; expense tracking is typically outsourced to third-party apps, leaving a fragmented experience.

  ### The OHC Gap
  Currently, OneHumanCorp focuses heavily on revenue generation (storefronts, booking, invoicing). However, true "business in a box" means managing the full profit lifecycle. Without an autonomous expense engine, our users still need separate accounting software to prepare for tax season or understand their actual profit margins. We need an AI-driven expense system seamlessly linked to the existing ledger architecture, completely invisible to the user beyond capturing the receipt.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      TENANT ||--o{ EXPENSE : logs
      TENANT ||--o{ RECEIPT_IMAGE : uploads
      RECEIPT_IMAGE ||--o{ EXPENSE : substantiates
      EXPENSE ||--o{ LEDGER_ENTRY : triggers

      TENANT {
          string id PK
          string tax_nexus
      }
      RECEIPT_IMAGE {
          string id PK
          string url
          string upload_source "Camera | EmailForward | Manual"
          timestamp captured_at
      }
      EXPENSE {
          string id PK
          float amount
          string currency
          string vendor_name
          date transaction_date
          string ai_category
          string status "Processing | NeedsReview | AutoApproved"
      }
      LEDGER_ENTRY {
          string id PK
          string type "Debit"
          float amount
          timestamp recorded_at
      }
  ```

  ### Mobile UX Flow (375px)
  1.  **Capture**: From the main OHC dashboard card, the user taps a prominent "Add Expense" floating action button. The camera opens instantly (or they can share an image from their gallery/email).
  2.  **Processing**: A non-blocking, translucent glass card appears: *"Scanning receipt from Home Depot..."* The user can close the app or keep working.
  3.  **Result**: A notification pops up: *"Logged $45.20 for Lumber (Supplies). Tap to undo."*
  4.  **Review (Only if needed)**: If the AI confidence is low, a clean card shows the cropped image of the total amount alongside a simple yes/no prompt: *"Is this total $45.20?"* No accounting jargon is ever shown.

  ### AI Agent Integration Points
  *   **Finance Agent (OCR & Extraction)**: Triggers upon image upload. Uses vision models to extract Vendor, Date, Total, Tax, and Line Items.
  *   **Operations Agent (Categorization)**: Maps the extracted vendor and items to plain-English categories based on the user's business type (e.g., categorizing flour as "Ingredients" for Maya, but "Office Supplies" for a consultant).
  *   **Tax Agent (Compliance)**: Automatically tags expenses that are tax-deductible based on the tenant's local tax nexus.

  ### Key Design Decisions
  *   **Zero-Jargon Categorization**: Instead of standard chart-of-account codes, the system uses natural language tags relevant to the user's industry. The AI translates these tags to formal tax codes invisibly in the backend during export.
  *   **Async Processing**: Receipt parsing happens entirely in the background via the agent event mesh. The UI never blocks waiting for OCR.
  *   **Immutable Ledger Linkage**: Every verified expense automatically creates a balanced entry in the unified ledger, ensuring real-time profit tracking without manual reconciliation.

  ## Implementation Prompt
  **Task**: Implement the "Magic Shoebox" Autonomous Expense Engine.
  **Outcome**: As a business owner, I want to snap a photo of a receipt so that my expenses are automatically tracked, categorized, and added to my profit/loss statement without manual data entry.
  **Acceptance Criteria**:
  *   An endpoint to accept receipt image uploads (multipart/form-data) or forwarded emails.
  *   Asynchronous background jobs to process the image, extracting amount, date, and vendor.
  *   An LLM/Vision prompt that categorizes the expense into a plain-English, industry-specific bucket.
  *   The system automatically creates an `EXPENSE` record and a corresponding debit `LEDGER_ENTRY`.
  *   Mobile UI components built for 375px screens showing quick capture and non-blocking status updates.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
