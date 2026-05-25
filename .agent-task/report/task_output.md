issue_title: "[Architecture] Autonomous Zero-Touch Accounting & Tax Reconciliation Engine"
issue_description: |
  # Autonomous Zero-Touch Accounting & Tax Reconciliation Engine

  ## Problem Statement
  Small business owners like Maya (baker), Carlos (handyman), and Fatima (food cart) spend countless hours manually categorizing expenses, matching receipts to bank feeds, calculating sales tax across jurisdictions, and dreading tax season. They are not accountants, yet existing tools (like QuickBooks or Xero) require them to act like ones, mapping transactions to complex charts of accounts. They just want to know how much money they made, how much they owe in taxes, and to have compliance handled automatically so they can focus on their business.

  ## Research Report
  - **The Market Gap:** Current solutions like QuickBooks Online, Xero, and Wave are designed for accountants first and business owners second. They require manual reconciliation, understanding of double-entry bookkeeping, and constant intervention. Shopify Finance offers good insights but lacks deep multi-platform expense tracking and automated tax filing.
  - **User Needs:** SMBs need a system that ingests financial data (sales from OHC, expenses from linked bank accounts/cards) and automatically categorizes them using AI, sets aside estimated taxes based on real-time earnings, and prepares tax forms with zero manual entry.
  - **Competitive Advantage:** OneHumanCorp can differentiate by offering "Invisible Accounting." By leveraging AI to read receipts via the mobile camera, automatically categorize bank feed data, and auto-calculate multi-jurisdiction tax liabilities (Sales, VAT, Income), OHC becomes an indispensable operating system rather than just a storefront or booking tool.
  - **Data Insights:** A large percentage of SMBs fail due to cash flow issues and unexpected tax liabilities. An autonomous engine that proactively manages these aspects significantly increases the survival rate of our merchants.

  ## Design Doc
  ### Mobile UX Flow (375px First)
  1. **Dashboard Home:** A clean, macOS-style Translucent Glass card showing "Net Profit (MTD)" and "Estimated Tax Set Aside." No complex P&L statements unless explicitly requested via "Advanced Settings."
  2. **Expense Capture (The Grandmother Test):** A persistent FAB (Floating Action Button) with a camera icon. Maya taps it, snaps a photo of a flour receipt, and the AI agent instantly parses vendor, amount, tax, and category. The screen simply says, "Got it! $45.20 at Costco added to Supplies."
  3. **Auto-Reconciliation Feed:** A feed showing recent transactions. Instead of asking to "Match," it shows a green checkmark indicating "Auto-categorized." If uncertain, it nudges with a simple question: "Was this $120 Home Depot run for a customer job or general supplies?"
  4. **Tax Vault:** A dedicated view showing exactly how much money is safely held for taxes, with a 1-tap "Pay Quarterly Taxes" button when due.

  ### AI Agent Integration Points
  - **Operations Department (Receipt Parsing Agent):** OCR and LLM-based extraction of receipt data.
  - **Finance Department (Categorization Agent):** Maps raw bank feed descriptions (e.g., "SQ* LOCAL COFFEE") to standardized expense categories without exposing the Chart of Accounts to the user.
  - **Legal/Compliance Department (Tax Agent):** Calculates jurisdictional tax liabilities based on customer location and product type (e.g., physical goods vs. services).

  ### Architecture Diagram (Mermaid.js)
  ```mermaid
  erDiagram
      MERCHANT ||--o{ BANK_TRANSACTION : has
      MERCHANT ||--o{ RECEIPT : uploads
      BANK_TRANSACTION ||--o{ RECONCILIATION_EVENT : generates
      RECEIPT ||--o{ RECONCILIATION_EVENT : supports
      RECONCILIATION_EVENT }|--|| LEDGER_ENTRY : creates
      LEDGER_ENTRY }|--|| TAX_LIABILITY : calculates

      MERCHANT {
          string id
          string business_type
          string tax_jurisdiction
      }
      BANK_TRANSACTION {
          string id
          float amount
          string raw_description
          date timestamp
      }
      RECEIPT {
          string id
          string image_url
          string extracted_vendor
          float extracted_amount
      }
      RECONCILIATION_EVENT {
          string id
          string status
          string ai_confidence_score
      }
      LEDGER_ENTRY {
          string id
          string category
          float amount
      }
      TAX_LIABILITY {
          string id
          float estimated_tax
          string tax_type
      }
  ```

  ### Zero Trust & Security
  - **Multi-Tenant Isolation:** Financial data is strictly partitioned by Merchant ID at the database level using Row-Level Security (RLS).
  - **Secure Identity:** Inter-agent communication (e.g., Receipt Agent to Ledger Agent) requires SPIFFE/SPIRE mutual TLS authentication.
  - **Data Encryption:** Bank credentials and sensitive tax IDs are encrypted at rest using KMS-backed keys and never exposed to the frontend or LLM prompt context directly.

  ## Implementation Prompt
  **For the Implementer Agent:**
  Design and build the backend services and mobile-first UI for the Autonomous Zero-Touch Accounting & Tax Reconciliation Engine.
  1. Create the data models necessary to securely store bank transactions, uploaded receipts, auto-categorized ledger entries, and estimated tax liabilities. Ensure strict multi-tenant isolation.
  2. Implement the AI agent protocols for the Finance and Operations departments to asynchronously process incoming transactions and receipts, automatically categorizing them with a high confidence threshold.
  3. Build the mobile-first dashboard components (targeting a 375px viewport) that display simplified financial health (Net Profit, Tax Set Aside) using the Translucent Glass design tokens.
  4. Provide an "Advanced Settings" toggle for power users who need to see the underlying Chart of Accounts or export traditional P&L statements.
  Do not prescribe specific database schemas or API endpoints here; design them based on the core entities described and ensure high performance (sub-200ms latency for dashboard loads) and offline capabilities for the receipt capture flow.

issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report, architecture, finance]
assignees: []
