issue_title: "Autonomous Plain-Language Bookkeeping & Tax Prep Engine"
issue_description: |
  # Autonomous Plain-Language Bookkeeping & Tax Prep Engine

  ## Title
  Autonomous Plain-Language Bookkeeping & Tax Prep Engine

  ## Problem Statement
  Small business owners, such as Carlos (handyman) or Fatima (food cart operator), experience extreme "Financial Fog" and anxiety around taxes. They rely on spreadsheets or a shoebox of receipts because tools like QuickBooks assume a knowledge of double-entry bookkeeping, charts of accounts, and reconciliation. These non-technical users want to snap a photo of a receipt, or have their bank feed ingested, and have an AI instantly categorize it, match it, and provide plain-language financial health summaries (e.g., "You have $1,200 set aside for taxes"). They need an invisible accountant that does the heavy lifting, eliminating the dreaded end-of-year tax scramble without exposing them to complex financial jargon.

  ## Research Report
  *   **User Pain Point:** "Financial Fog" ranks #9 among top SMB pain points. Solopreneurs lack the time and financial literacy to manage traditional accounting software.
  *   **Competitor Analysis:**
      *   **Shopify:** Excellent at tracking sales revenue, but lacks built-in comprehensive expense tracking or receipt ingestion for offline purchases.
      *   **QuickBooks/Xero:** Highly capable but fail the "Grandmother Test". They are designed for accountants, presenting intimidating UI elements like "Chart of Accounts" and "Journal Entries" directly to the business owner.
      *   **Wix/Squarespace:** Provide basic revenue reports but no true bookkeeping capabilities.
  *   **OHC Advantage ("Invisible Autonomy"):** OHC shifts the paradigm from a software tool the user must operate to an "Invisible AI Accountant". By combining the Operations Agent (for receipt OCR via mobile) and the Finance Agent (for categorization and ledger updating), OHC autonomously maintains a multi-tenant `FinancialLedger`. The Business Advisory Agent then surfaces this data in simple, actionable English, replacing complex dashboards with a conversational interface.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ TRANSACTION : "Generates"
      TRANSACTION ||--o| RECEIPT_IMAGE : "Contains"
      TRANSACTION }|--|| FINANCIAL_LEDGER : "Recorded In"

      FINANCIAL_LEDGER {
          string tenant_id "Multi-tenant isolation"
          string transaction_id
          float amount
          string category
          boolean tax_deductible
          string status
      }

      TRANSACTION_EVENT ||--o{ ACCOUNTANT_AGENT : "Triggers"
      ACCOUNTANT_AGENT ||--o{ FINANCIAL_LEDGER : "Categorizes & Updates"
      ACCOUNTANT_AGENT ||--o{ ADVISORY_AGENT : "Feeds Data To"

      ADVISORY_AGENT ||--o{ MOBILE_UI : "Surfaces Plain-Language Briefing"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First). Seamless one-handed operation.
  *   **Expense Capture Flow:**
      *   A prominent, floating "Camera" action button at the bottom of the Financial tab.
      *   User snaps a photo of a receipt. A skeleton loading state appears on a frosted glass card (`rgba(255, 255, 255, 0.05)` with `backdrop-filter: blur(10px)`).
      *   Within seconds, the card transforms to show the parsed data: "Home Depot - $45.20 - Categorized as 'Supplies'".
      *   A single primary button: `[Approve]` and a secondary `[Edit]`.
  *   **Financial Briefing Widget:**
      *   A card on the home dashboard replacing traditional pie charts.
      *   Displays plain language text: "You made $400 more this week. We've set aside $120 for estimated taxes. You have 3 uncategorized expenses to review."

  ### Mobile UX Flow
  1. **Action:** Carlos buys lumber at Home Depot. He opens the OHC app and taps the floating camera icon to take a photo of the receipt.
  2. **Background Processing:** The image is uploaded asynchronously to the KAIROS Orchestrator. The Operations Agent performs OCR, and the Finance Agent categorizes it as "Materials & Supplies".
  3. **Review:** A push notification or an Action Feed card appears: "✨ Accountant Agent categorized your $45.20 Home Depot receipt. No action needed unless incorrect."
  4. **End of Month:** The Advisory Agent sends a plain-language summary detailing total revenue, expenses, and estimated tax liabilities, entirely bypassing traditional accounting views.

  ### AI Agent Integration Points
  *   **Operations Department:** Handles the OCR and ingestion of physical receipt images via the mobile interface.
  *   **Finance Department (The Accountant):** Semantically analyzes the transaction context to correctly categorize the expense and update the `FinancialLedger`.
  *   **Business Advisory Department:** Translates the raw ledger data into a plain-language briefing for the user dashboard.

  ### Key Design Decisions (Why, not How)
  *   **Event-Driven Ledger:** All financial changes must be recorded in an immutable, append-only `FinancialLedger` to ensure auditability and prevent data corruption, even if offline transactions sync later.
  *   **Jargon-Free Interface:** All accounting terms (ledger, reconciliation, chart of accounts) must be completely abstracted away. The user only interacts with "Expenses", "Income", and "Taxes".
  *   **Strict Multi-Tenant Isolation:** Financial data is the most sensitive information on the platform. The `FINANCIAL_LEDGER` and API routing must enforce SPIFFE identity checks and tenant-level isolation boundaries at the database level.

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Your goal is to build the backend architecture and mobile-first UI for the "Autonomous Plain-Language Bookkeeping Engine". This engine will allow users like Carlos to capture receipts via their phone, have an AI automatically categorize the expense, and view plain-language financial summaries without ever seeing a traditional accounting dashboard.

  **Customer User Journey (CUJ):**
  1. User navigates to the Financial tab on their mobile device (375px viewport).
  2. User uploads or takes a photo of a receipt.
  3. The system processes the image, categorizes the expense, and presents an "Approve" card in the Action Feed.
  4. The user's daily briefing updates to reflect the new expense and recalculated estimated tax liability.

  **Acceptance Criteria:**
  *   **Mobile Parity:** The UI must adhere to the 375px Translucent Glass design tokens and feel instantaneous, using optimistic UI patterns for image uploads.
  *   **AI Coordination:** The backend must orchestrate a workflow where an uploaded receipt triggers an event that the Operations Agent and Finance Agent process to update the `FinancialLedger`.
  *   **Data Isolation:** Implement strict multi-tenant boundary checks on all financial ledger queries.
  *   **Plain Language Translation:** The system must generate a human-readable text summary of the business's current financial state (e.g., "Estimated taxes: $X"), avoiding accounting jargon. Do not expose specific database schemas or API contracts; focus on the business logic and agent handoffs.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []