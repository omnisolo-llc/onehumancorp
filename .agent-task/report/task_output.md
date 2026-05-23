issue_title: "[Architecture] Zero-Friction Autonomous Cashflow and Expense Reconciliation Engine"
issue_description: |
  # Zero-Friction Autonomous Cashflow and Expense Reconciliation Engine

  ## Problem Statement
  Small business owners—like Maya the baker and Carlos the handyman—operate with extremely tight margins and limited time. Tracking expenses, categorizing receipts, and reconciling cashflow is currently a tedious, manual process requiring complex software (like QuickBooks) that they don't understand or have time for. This leads to lost tax deductions, cashflow crises due to invisible spending, and end-of-month panic. They need an invisible, autonomous financial teammate that handles reconciliation in the background using natural behaviors (snapping photos or forwarding emails) and provides simple, actionable insights.

  ## Research Report
  Current SMB platforms completely fail at this:
  *   **Shopify:** Focuses almost entirely on revenue (sales, checkouts). Expense tracking requires expensive 3rd-party accounting apps.
  *   **Wix/Squarespace:** Provide basic revenue reporting but no native expense or cashflow reconciliation tools.
  *   **QuickBooks/Xero:** Highly capable but feature extreme complexity, desktop-first design, and require accounting knowledge to set up properly (chart of accounts, bank feeds).

  **The OHC Advantage:** By leveraging the Agentic Teammate Model (specifically the Finance Agent), OHC can turn expense tracking into a zero-touch experience. The user merely "tosses" receipts into the system (via phone camera or email), and the AI handles OCR, categorization against the ledger, tax tagging, and anomaly detection.

  ## Design Doc

  ### Key Design Decisions
  1.  **Mobile First (375px) & Natural Inputs:** The primary interface for expenses is the smartphone camera and the native email client. There is no complex "Add Expense" form to fill out.
  2.  **Conversational Insights:** Dashboards are replaced with conversational SMS/Push notifications (e.g., "Maya, you spent $150 on flour this week. Your cashflow is healthy.").
  3.  **Autonomous Ledger Integration:** Every expense automatically hits the universal ledger, ensuring real-time multi-tenant data consistency without manual entry.
  4.  **Invisible Tax Preparation:** The AI automatically tags deductible expenses and generates a plain-language tax summary for end-of-year filing.

  ### Architecture Diagram (ER & Component Interaction)

  ```mermaid
  erDiagram
      TENANT ||--o{ EXPENSE_RECORD : incurs
      EXPENSE_RECORD {
          uuid id
          uuid tenant_id
          decimal amount
          string currency
          string vendor_name
          string category
          date transaction_date
          string receipt_image_url
          string tax_deductible_status
          boolean is_reconciled
      }
      EXPENSE_RECORD ||--o{ LEDGER_ENTRY : triggers
      LEDGER_ENTRY {
          uuid entry_id
          uuid expense_id
          decimal debit_amount
          decimal credit_amount
          timestamp created_at
      }
  ```

  ```mermaid
  sequenceDiagram
      participant User
      participant App_UI as Mobile App / Email
      participant Finance_Agent as Finance AI Agent
      participant OCR_Engine as Vision / OCR Service
      participant Ledger as Universal Ledger
      participant Notification as Notification Engine

      User->>App_UI: Snaps photo of receipt / forwards email
      App_UI->>Finance_Agent: Upload receipt payload
      Finance_Agent->>OCR_Engine: Extract vendor, amount, date, line items
      OCR_Engine-->>Finance_Agent: Parsed JSON data
      Finance_Agent->>Finance_Agent: Categorize & determine tax deductibility
      Finance_Agent->>Ledger: Create reconciled Expense Record & Ledger Entry
      Ledger-->>Finance_Agent: Acknowledge commit
      Finance_Agent->>Notification: Trigger cashflow insight (if threshold met)
      Notification-->>User: SMS: "Logged $45 for Home Depot. Weekly materials budget: 80% remaining."
  ```

  ### Mobile UX Flow (375px)
  1.  **Home Screen:** Prominent "Snap Receipt" FAB (Floating Action Button) on the dashboard.
  2.  **Camera View:** Native camera interface opens immediately. User snaps photo.
  3.  **Processing State:** A subtle skeleton loader appears briefly with text "Finance agent categorizing...".
  4.  **Confirmation Toast:** A brief, transient success message "Receipt logged: $45.00 at Home Depot". No further action required.
  5.  **Insights View:** A simple feed showing plain-language summaries: "This week's spending: $120. Top category: Supplies."

  ## Implementation Prompt
  **To the Implementer Swarm:**
  Design and implement the `Autonomous Cashflow and Expense Reconciliation Engine`.
  *   **User-Facing Outcome:** The user must be able to upload an image of a receipt via the mobile UI or forward an email. The system must autonomously parse the image/email, extract the transaction details, categorize the expense, and update the business's ledger.
  *   **Acceptance Criteria:**
      *   Endpoint/Service must accept image payloads and email payloads.
      *   Finance Agent must successfully extract `amount`, `vendor`, `date`, and `category` using vision/parsing models.
      *   The extracted data must be securely committed to the Universal Ledger.
      *   The system must trigger a background notification (SMS/Push format) summarizing the expense if it exceeds predefined thresholds or if requested.
      *   Multi-tenant isolation must be strictly enforced—an expense must never leak across tenant boundaries.

  **Priority**: P1
  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
