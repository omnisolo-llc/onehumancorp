issue_title: "Integrate Gusto Embedded Payroll for Automated Staff Payments and Compliance"
issue_description: |
  ## Title
  Integrate Gusto Embedded Payroll for Automated Staff Payments and Compliance

  ## Problem Statement
  Small business owners like Leo (Music Tutor with junior instructors) and Fatima (Food Cart Operator with part-time staff) spend hours every pay period manually tallying hours from schedules, calculating split tips, withholding correct taxes, and initiating bank transfers. This process is highly stressful, error-prone, and carries significant regulatory and tax compliance risks. They need a system that seamlessly translates their business operations (time clocked, jobs completed, tips earned via Tap-to-Pay) into fully compliant staff paychecks without leaving the OHC platform.

  ## Research Report
  *   **Tool:** Gusto Embedded Payroll (API)
  *   **Market Position:** Gusto is a dominant player in US small business payroll, known for its extreme ease of use and automated tax compliance. Their "Embedded Payroll" API allows platforms like OHC to offer full-service payroll natively.
  *   **Capabilities & Limits:**
      *   **Automated Payroll Runs:** Can trigger payroll dynamically based on OHC timesheets or commission structures.
      *   **Compliance:** Automatically handles federal, state, and local tax filings and W-2/1099 generation.
      *   **Onboarding:** Provides drop-in UI flows for employee onboarding (I-9s, bank details), reducing the integration burden on OHC engineers.
      *   **API Quality:** Extensive, developer-friendly REST API with robust webhook support for lifecycle events.
  *   **SaaS Viability & Pricing:**
      *   **Pricing Model:** Typically a base platform fee plus a per-employee monthly fee. For embedded partners, revenue sharing models exist. This can be packaged as a premium add-on within OHC.
      *   **Modes:** Highly viable for Cloud (multi-tenant) environments. In Standalone mode, merchants would connect their own Gusto account via OAuth.
  *   **Ease of Use:** Exceptional. Non-technical users benefit from an "autopilot" payroll experience where they simply approve pre-calculated totals.

  ## Design Doc
  *   **Trigger:**
      *   **Event 1 (Continuous):** Staff members clock in/out or complete booked services via the OHC App. Tips from Tap-to-Pay are automatically attributed.
      *   **Event 2 (Pay Period):** The OHC "HR Agent" aggregates the hours, commissions, and tips for the defined pay period.
  *   **Action:** OHC presents a single "Payroll Review" card in the activity feed. Upon user approval, OHC pushes the finalized totals to the Gusto API to execute the payroll run, debiting the merchant's account and paying the staff.
  *   **User Experience (OHC Dashboard):**
      *   The business owner sees a "Team & Payroll" tab.
      *   They invite staff via email. Staff use a secure Gusto flow to enter sensitive tax/bank info (abstracted from OHC).
      *   On payday, the owner gets a push notification: "Review Payroll for Nov 1-15: $4,200. Approve?"
      *   1-Tap Approval finalizes the run. All tax compliance is handled invisibly.

  ## Implementation Prompt
  Implement an integration with the Gusto Embedded Payroll API (or Gusto OAuth for standalone accounts) to automate staff compensation based on OHC operational data.
  *   **Acceptance Criteria 1 (Sync):** OHC timesheet data (clock-ins, booked services) and attributed tips must seamlessly sync to Gusto as payroll line items.
  *   **Acceptance Criteria 2 (Approval Flow):** Provide a simple mobile-first UI for the business owner to review the upcoming payroll totals (wages + taxes) before submission.
  *   **Acceptance Criteria 3 (Execution):** Clicking "Approve" successfully triggers a payroll run via the Gusto API.
  *   **Acceptance Criteria 4 (Onboarding):** Leverage Gusto's pre-built UI components or API flows for secure employee self-onboarding (tax forms and direct deposit), ensuring OHC does not store sensitive PII.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
