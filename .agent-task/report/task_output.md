issue_title: "Autonomous Payroll and Contractor Ledger"
issue_description: |
  # Title: Autonomous Payroll and Contractor Ledger

  ## Problem Statement
  Small business owners, especially in service and hybrid domains (like Carlos the handyman or Fatima the food cart owner), frequently work with part-time staff, independent contractors, or temporary helpers. Managing payroll, splitting payments for jobs, tracking hours, calculating taxes, and remitting contractor payouts is incredibly complex and high-risk. Current platforms separate business operations (sales/booking) from human resources and payroll (Gusto, QuickBooks Payroll). This forces owners to manually reconcile timesheets and sales data across multiple apps. They need an invisible, unified engine that automatically calculates payouts based on shifts or job completions, tracks tax liabilities, and facilitates 1-tap mobile payouts without requiring them to become payroll experts.

  ## Research Report
  *   **Competitor Analysis**:
      *   **Shopify/Wix**: No native payroll or contractor management capabilities. They focus entirely on the customer transaction.
      *   **Gusto/QuickBooks**: Excellent standalone payroll platforms but disconnected from the daily operational flow (booking, sales, POS).
      *   **Square**: Offers team management and payroll, but it can be clunky, and the integration still feels like separate products bolted together rather than an invisible, unified system.
  *   **The OHC Gap**: To be a true "business in a box," OHC must integrate worker compensation seamlessly. When Carlos completes a $500 job with his assistant, the system should automatically know the assistant's split or hourly rate, queue the payout, and calculate necessary tax withholding or 1099 tracking, presenting Carlos with a simple 1-tap approval.

  ## Design Doc

  ### High-Level Architecture
  ```mermaid
  graph TD;
      JobCompletion[Job Completed / Shift Ended] --> EventMesh[Hybrid Event Mesh];
      EventMesh --> PayrollAgent[AI Payroll & Compliance Agent];

      PayrollAgent --> TimeTracker[(Unified Capacity Mesh)];
      PayrollAgent --> FinanceLedger[(Universal Ledger)];
      PayrollAgent --> TaxEngine[Tax Compliance Engine];

      TaxEngine --> Withholding[Calculate Taxes / 1099 Tracking];
      Withholding --> PayrollAgent;

      PayrollAgent --> DraftPayout[Draft Payout Intent];
      DraftPayout --> AppUI[OHC App: 1-Tap Payout Approval];

      AppUI -->|Owner Approves| TreasuryAgent[AI Treasury Agent];
      TreasuryAgent --> Disbursement[Stripe Connect / ACH Dispatch];
  ```

  ### Key Design Decisions & Invariants
  *   **Unified Event Triggering**: Payout calculations are triggered natively by operational events (e.g., POS shift end, booking completion) rather than relying on manual timesheet entries.
  *   **Zero-Trust Isolation**: Employee and contractor data (PII, tax IDs, banking details) are strictly isolated using SPIFFE/SPIRE Zero-Trust boundaries. Data is encrypted at rest and in transit.
  *   **Mobile-First "1-Tap" UX**: Complex payroll runs are distilled into a clear, unified daily or weekly notification: "Approve $850 payout for 3 staff members? (Taxes automatically withheld)."
  *   **Background Tax Compliance**: The AI Payroll Agent continuously monitors thresholds (e.g., $600 for 1099 contractors) and automatically queues form generation and compliance alerts.

  ## Implementation Prompt
  **Objective:** Build the Autonomous Payroll and Contractor Ledger.
  **User Journey (CUJ):**
  1.  Carlos (Owner) and David (Contractor) finish a plumbing job booked through OHC.
  2.  Carlos marks the job "Complete" in the OHC mobile app.
  3.  The AI Payroll Agent detects the completion, cross-references the job's predefined 70/30 split, and drafts a payout intent of $150 to David.
  4.  Carlos receives a mobile notification: "Job complete. Send $150 to David?"
  5.  Carlos taps "Approve." The AI Treasury Agent processes the payment via Stripe Connect and records the 1099-NEC progress in the background.

  **Acceptance Criteria:**
  - Create the background worker or service responsible for listening to operational events (`ShiftEnded`, `JobCompleted`) and calculating payouts based on contractor agreements or hourly rates.
  - Design the data models for `ContractorAgreement`, `PayoutIntent`, and `TaxLiabilityTracker`. Ensure strict multi-tenant isolation.
  - Develop the "1-tap" approval UI flow for mobile (375px viewport), ensuring all complex tax and calculation details are hidden behind an "Advanced Details" sheet.
  - Ensure the AI Payroll Agent handles the context and memory of previous payments to track annual compliance thresholds invisibly.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
