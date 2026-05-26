issue_title: "Architectural Gap: Autonomous Embedded Payroll & Contractor Payouts"
issue_description: |
  # Research Report: Autonomous Embedded Payroll & Contractor Payouts

  ## Problem Statement
  Many OneHumanCorp (OHC) merchants, such as Maya the baker or Carlos the handyman, rely on subcontractors, part-time help, or tipped employees. Currently, managing payroll, split tips, and contractor payouts involves a fragmented workflow: calculating hours manually, splitting tips at the end of the day, writing checks, and tracking tax liabilities externally. This manual overhead prevents small business owners from scaling and creates friction. OHC needs a built-in, zero-touch embedded payroll and contractor payouts engine that automates wage calculations, tip splitting, and tax compliance invisibly.

  ## Research Report
  **Market Analysis:**
  - **Shopify/Wix:** Rely on third-party apps like Gusto or QuickBooks. While powerful, they require complex configuration, mapping of accounts, and separate subscriptions, failing the "grandmother test."
  - **Square:** Provides built-in payroll, but it is often a separate module with its own complex onboarding flow.
  - **Toast:** Excellent for restaurant tip splitting, but highly specific to F&B and expensive.

  **Opportunity:** OHC can integrate an autonomous embedded payroll engine directly into the core platform, managed by the AI Finance and HR Departments. By treating contractor and employee payouts as a first-class citizen tied to the Universal Ledger, OHC can offer merchants automated tip splitting, instant shift payouts, and zero-touch tax compliance without any third-party integrations.

  ## Design Doc

  ### Architecture Diagram

  ```mermaid
  erDiagram
      MERCHANT ||--o{ WORKER : employs
      WORKER }|--|| PAYOUT_SESSION : participates_in
      MERCHANT ||--o{ PAYOUT_SESSION : initiates
      PAYOUT_SESSION ||--o{ LEDGER_ENTRY : generates

      MERCHANT {
          uuid id
          string name
      }

      WORKER {
          uuid id
          uuid merchant_id
          string name
          string role
          float hourly_rate
          boolean receives_tips
      }

      PAYOUT_SESSION {
          uuid id
          uuid merchant_id
          datetime shift_start
          datetime shift_end
          float total_tips
          string status
      }

      LEDGER_ENTRY {
          uuid id
          uuid payout_session_id
          uuid worker_id
          float amount
          string type "wage, tip, tax_withholding"
      }
  ```

  ### AI Agent Integration
  - **HR/Finance Agent:** Continuously monitors `PAYOUT_SESSION` completion. When a shift ends, it calculates wages, splits tips based on configured rules (e.g., hours worked or role), and generates `LEDGER_ENTRY` records.
  - **Compliance Agent:** Automatically calculates estimated tax withholdings and sets aside funds in a reserved ledger account, preparing end-of-year 1099 or W-2 reports invisibly.

  ### UI Wireframes & Mobile UX Flow (375px viewport)
  1. **Merchant Dashboard (Team View):**
     - Clean, translucent card layout showing "Active Shift" and "Pending Payouts".
     - A single prominent "End Shift & Payout" button.
     - List of workers with visual status (Green = Paid, Yellow = Pending).
  2. **Worker View (Mobile Web/App):**
     - Large, high-contrast typography showing "Earnings Today" and "Available to Withdraw".
     - "Instant Payout" button to transfer funds to a connected debit card.
  3. **Advanced Settings (Hidden):**
     - Tax configuration, custom tip split rules, and ledger export.

  ### Key Design Decisions
  - **Zero-Config Tip Splitting:** The system auto-proposes tip splits based on shift hours and roles, requiring only a single tap to approve from the merchant.
  - **Multi-Tenant Isolation:** `WORKER` and `PAYOUT_SESSION` data strictly enforces tenant boundaries. Ledger entries are cryptographically tied to the merchant's master ledger.
  - **Mobile-First Design:** All shift management and payout approvals can be done with a single thumb on a 375px screen.

  ## Implementation Prompt
  **Task for Implementer:**
  Build the underlying data model and backend services for the Autonomous Embedded Payroll Engine.
  1. Create the database schemas for Workers, Payout Sessions, and Ledger Entries, ensuring strict multi-tenant isolation.
  2. Implement the API endpoints to start/end shifts and record tips.
  3. Integrate the HR/Finance Agent to trigger payout calculation logic when a shift ends.
  4. Ensure endpoints support high-concurrency for end-of-day shift closures across multiple merchants.

  ## Priority
  P1 (High)

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
