issue_title: "Universal Autonomous Payroll & Contractor Mesh"
issue_description: |
  ## Problem Statement
  Small business owners like Carlos (the handyman) and Priya (the boutique owner) frequently hire temporary contractors, seasonal staff, or part-time employees. The current process of onboarding them, tracking their hours or project completion, and running compliant payroll is a massive, stressful administrative burden. They have to switch to specialized software (like Gusto or ADP) that feels overwhelming and disconnected from their core operations, manually input time tracked on paper or a separate app, and worry about constantly changing tax compliance. They need an invisible, zero-friction system integrated directly into OneHumanCorp that automatically tracks work, computes correct payouts (including tax withholdings or contractor 1099 compliance), and executes payments—all while abstracting away the accounting complexity.

  ## Research Report
  *   **Gusto / ADP / Paychex:** Built primarily as standalone HR and payroll platforms. Excellent compliance, but they require significant manual setup (charts of accounts, onboarding workflows) and aren't natively integrated into the operational heartbeat of a tiny business (like when a job on the OHC calendar is completed).
  *   **Square Payroll / Shopify:** Offer some integrated payroll, but often still feel like a separate module where the merchant has to "run payroll" manually.
  *   **OneHumanCorp (OHC) Differentiation - "Invisible Operations":** OHC eliminates the concept of "running payroll". By deeply integrating with the Universal Capacity and Inventory Ledger, and the Autonomous Treasury Wallet, payroll and contractor payouts happen autonomously. When Carlos's sub-contractor completes a job (marked done in the OHC app), the AI Operations Agent validates the work, calculates the pre-agreed split or hourly rate, and queues the payment in the Treasury layer. The merchant only needs a one-tap approval.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      STAFF_MEMBER ||--o{ TIME_ENTRY : "Logs"
      STAFF_MEMBER ||--o{ JOB_COMPLETION : "Performs"
      TIME_ENTRY }|--|| PAYROLL_ENGINE : "Processes"
      JOB_COMPLETION }|--|| PAYROLL_ENGINE : "Triggers"

      PAYROLL_ENGINE {
          string spiffe_identity "Zero Trust execution"
          string tenant_id "Multi-tenant isolation"
      }

      PAYROLL_ENGINE ||--o{ COMPLIANCE_AGENT : "Validates (Tax, Overtime)"
      PAYROLL_ENGINE ||--o{ TREASURY_LEDGER : "Creates Payout"
      TREASURY_LEDGER ||--o{ NOTIFICATION_ROUTER : "1-Tap Approval"
  ```

  ### AI Agent Integration Points
  *   **Operations Agent:** Monitors schedule adherence and job completions to automatically draft timecards or piece-rate invoices.
  *   **Compliance/Legal Agent:** In the background, ensures workers are correctly classified (W2 vs 1099), tracks nexus for state taxes based on geo-location of the work, and holds funds in escrow if required.
  *   **Finance Agent:** Automatically updates the P&L and reserves tax liabilities in the merchant's Treasury Wallet when a payout is queued.

  ### Mobile UX Flow (375px First)
  1. **The "Team" Tab:** A clean, unified card layout showing active staff/contractors and their current status (On Shift, Scheduled, Off).
  2. **One-Tap Onboarding:** Merchant taps "Add Team Member". Instead of forms, they just send a secure, expiring SMS link. The staff member completes their own onboarding (banking, tax info) on their phone.
  3. **Autonomous Payout Approval:** On Friday at 5 PM (or right after a job), a sticky notification appears: "Carlos worked 20 hours. Approve $500 payout? (Taxes auto-withheld)". A simple swipe right executes the payment.
  4. **"Grandmother Test" pass:** No mention of "Form W-4", "Direct Deposit ACH", or "Tax Liabiltiy". Just "Who worked, what did they earn, swipe to pay."

  ### Key Design Decisions
  *   **Event-Driven Ledger:** Payroll is an emergent property of operational events (timecards, jobs done), not a batch process.
  *   **Zero-Trust Isolation:** Every contractor/employee gets a scoped SPIFFE identity, ensuring they only see their own pay and schedule data, heavily isolated within the tenant boundary.
  *   **Embedded Finance:** Payments are orchestrated directly via OHC's Treasury Ledger, bypassing the need for a third-party payroll provider API for the actual funds flow, allowing instant payouts.

  ## Implementation Prompt
  **User-Facing Outcome:** Build the underlying mesh that allows a merchant to add a worker and have their pay calculated and queued automatically based on their activity in the app.
  **CUJ (Critical User Journey):** As a merchant, I want to invite a contractor via SMS, and when they mark a booked service as "Complete", I want to see a pre-calculated payout ready for my 1-tap approval.
  **Acceptance Criteria:**
  *   Implement the core data entities for Worker Profiles, Payout Agreements (hourly vs fixed), and Work Events.
  *   Create an event listener that catches `JobCompleted` or `ShiftEnded` events and drafts a `PayoutIntent`.
  *   Ensure the `PayoutIntent` reserves the correct amount from the merchant's simulated ledger balance.
  *   Provide a secure, multi-tenant endpoint for the merchant to approve the `PayoutIntent`.
  *   Do not implement third-party tax API integrations yet; use a mocked service for tax calculations.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
