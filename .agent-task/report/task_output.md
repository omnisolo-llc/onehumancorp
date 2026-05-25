issue_title: Implement Autonomous Expense and Financial Reconciliation Mesh
issue_description: "# Issue Brief: Autonomous Expense and Financial Reconciliation\
  \ Mesh\n\n## Title\nImplement Autonomous Expense and Financial Reconciliation Mesh\
  \ for Outbound Money Tracking\n\n## Problem Statement\nWhile OneHumanCorp (OHC)\
  \ excels at inbound revenue generation (storefronts, bookings, invoices), business\
  \ owners like Carlos (handyman) and Maya (baker) face a significant burden tracking\
  \ expenses and outbound money. They buy supplies from hardware stores or grocery\
  \ stores, collect physical and digital receipts, and currently have to manually\
  \ record these expenses or use disconnected, complex software like QuickBooks. They\
  \ need an invisible, zero-friction system where they simply take a photo of a physical\
  \ receipt or forward an email receipt, and the system automatically categorizes\
  \ the expense, reconciles it against connected bank feeds, and generates tax-ready\
  \ P&L reports, all from their phone.\n\n## Research Report\nCurrent SMB tools fall\
  \ short in providing a unified, low-effort experience:\n- **QuickBooks / Xero:**\
  \ Powerful but extremely complex. Requires an understanding of accounting principles\
  \ (chart of accounts, double-entry bookkeeping). Overkill for a one-person bakery\
  \ or handyman service. The mobile apps feel like stripped-down desktop ports.\n\
  - **Ramp / Brex:** Excellent for corporate spend but target larger startups and\
  \ enterprises with physical corporate cards. Not designed for a sole proprietor\
  \ buying $50 of flour at a local market with a personal or basic business debit\
  \ card.\n- **Expensify:** Focuses on employee reimbursement rather than core SMB\
  \ P&L tracking.\n**The OHC Opportunity:** By integrating an AI-driven expense parser\
  \ directly into the unified OHC inbox/dashboard, we eliminate the need for a separate\
  \ app. The \"Finance AI Agent\" can autonomously parse receipts, match them to bank\
  \ feed transactions (via Plaid/Stripe integration), and categorize them for taxes\
  \ without the user ever seeing a spreadsheet.\n\n## Design Doc\n\n### Architecture\
  \ Diagram (Mermaid.js)\n```mermaid\nerDiagram\n    TENANT ||--o{ EXPENSE_RECORD\
  \ : has\n    TENANT ||--o{ BANK_ACCOUNT : connects\n    BANK_ACCOUNT ||--o{ BANK_TRANSACTION\
  \ : contains\n    EXPENSE_RECORD ||--o{ RECEIPT_DOCUMENT : includes\n    EXPENSE_RECORD\
  \ }|--|| VENDOR : paid_to\n    \n    EXPENSE_RECORD {\n        uuid id PK\n    \
  \    uuid tenant_id FK\n        decimal amount\n        string currency\n      \
  \  string category\n        date expense_date\n        string status \"pending_match,\
  \ reconciled, flagged\"\n    }\n    \n    RECEIPT_DOCUMENT {\n        uuid id PK\n\
  \        uuid expense_record_id FK\n        string storage_url\n        string source\
  \ \"email, photo, upload\"\n        json raw_extracted_data\n    }\n    \n    BANK_TRANSACTION\
  \ {\n        uuid id PK\n        uuid bank_account_id FK\n        decimal amount\n\
  \        string description\n        date posted_date\n    }\n```\n\n### UI Wireframes\
  \ / Screen Flow Description (375px first)\n**Screen 1: The \"Add Expense\" Bottom\
  \ Sheet**\n- Triggered by a prominent \"+\" FAB on the dashboard.\n- Options: \"\
  Snap Photo of Receipt\", \"Enter Manually\".\n- Clean, translucent glass UI (macOS\
  \ style).\n- If \"Snap Photo\" is chosen, opens camera.\n\n**Screen 2: AI Processing\
  \ & Confirmation Card**\n- After taking a photo, a Ubiquiti-style modular card appears:\
  \ \"Analyzing receipt...\" with a subtle pulsing animation.\n- Updates in <3 seconds\
  \ to show:\n  - Vendor: Home Depot\n  - Total: $142.50\n  - Category: Supplies (auto-selected)\n\
  - Big primary button: \"Looks Good\", secondary \"Edit\".\n- Hidden behind \"Advanced\
  \ Settings\" (or just auto-handled): Tax deductibility status.\n\n**Screen 3: Financial\
  \ Health Dashboard**\n- A card on the main dashboard showing \"Net Profit this Month\"\
  \ (Revenue - Expenses).\n- Tapping it reveals a simple list of recent expenses with\
  \ their reconciliation status (e.g., a green checkmark if matched to a bank transaction).\n\
  \n### Mobile UX Flow\n1. **Capture:** Carlos buys lumber. He opens the OHC app,\
  \ taps \"+\", snaps a photo of the receipt, and puts his phone away.\n2. **AI Processing\
  \ (Background):** The Finance AI Agent extracts data, creates an `EXPENSE_RECORD`,\
  \ and flags it as `pending_match`.\n3. **Reconciliation (Background):** The system\
  \ continuously polls the connected bank feed. When the $142.50 transaction posts,\
  \ the agent matches it to the receipt and updates status to `reconciled`.\n4. **Insight\
  \ (Actionable):** Carlos sees his monthly profit updated in real-time, knowing the\
  \ expense is fully documented for tax season without touching a spreadsheet.\n\n\
  ### AI Agent Integration Points\n- **Finance AI Agent (Vision/OCR):** Triggered\
  \ upon receipt image upload to extract Vendor, Date, Line Items, Total, and Taxes.\n\
  - **Finance AI Agent (Categorization):** Uses an LLM to map the raw vendor/items\
  \ to standard tax categories (e.g., \"Home Depot\" -> \"Supplies/Materials\").\n\
  - **Operations AI Agent (Reconciliation):** A background worker that fuzzy-matches\
  \ parsed receipts with incoming bank feed transactions based on date, amount, and\
  \ vendor name.\n\n### Key Design Decisions and Why\n- **Receipt-First, not Bank-First:**\
  \ Users think in terms of physical receipts at the point of purchase. By letting\
  \ them capture the receipt immediately, we secure the documentation. The bank match\
  \ can happen later asynchronously.\n- **Zero-Config Categories:** We do not expose\
  \ a \"Chart of Accounts\". The AI categorizes expenses into plain-English buckets\
  \ (Supplies, Travel, Software) that map to tax codes backend.\n- **Multi-Tenant\
  \ Isolation & Zero Trust:** Every expense and receipt is strictly keyed to `tenant_id`.\
  \ Bank feed credentials (e.g., Plaid tokens) are securely vaulted. Access to receipt\
  \ images via URL must require a signed, short-lived token tied to the tenant's active\
  \ session.\n\n## Implementation Prompt\n**To the Implementer Agent:**\nImplement\
  \ the core data models and background processing queue for the Autonomous Expense\
  \ Mesh. \n1. Create the database schemas to support tracking individual expenses,\
  \ storing receipt metadata (linking to blob storage), and tracking bank feed transactions\
  \ for a given tenant. Ensure strict multi-tenant isolation.\n2. Implement the API\
  \ endpoints necessary for the mobile app to upload a receipt image and create a\
  \ pending expense record.\n3. Stub the integration points where the Finance AI Agent\
  \ will be invoked to parse the receipt and where the background reconciliation job\
  \ will run to match expenses to bank transactions.\nAcceptance Criteria: The system\
  \ can accept a receipt upload, securely store its reference, and maintain a ledger\
  \ of expenses separate from inbound revenue, all correctly scoped to a specific\
  \ business owner.\n\n## Priority\nP1 (High - Critical for the complete financial\
  \ picture of the SMB)\n\n## Estimated Scope\nLarge\n"
issue_priority: P1
issue_category: research
issue_type: task
issue_label:
- agent-report
assignees: []
