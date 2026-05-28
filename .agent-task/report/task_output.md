issue_title: "Invisible Autonomous Payroll & Contractor Payout Engine"
issue_description: |
  # [architecture] Invisible Autonomous Payroll & Contractor Payout Engine

  ## Title
  Invisible Autonomous Payroll & Contractor Payout Engine

  ## Problem Statement
  Small business owners like Carlos (the handyman) and Fatima (the food cart operator) rely on a mix of full-time staff, part-time helpers, and independent contractors to keep their businesses running. Tracking hours, calculating payouts for specific jobs, remembering to pay people on time, and managing complex tax filings (like 1099s or W-2s) is a massive source of anxiety and administrative overhead. They spend their weekends in spreadsheets instead of growing their business or resting. The current process is highly manual, prone to errors, and requires specialized knowledge that these owners lack. They need an invisible system that automatically logs work, calculates fair payouts based on agreed-upon terms, and handles the actual transfer of funds and tax reporting without them having to click a single button.

  ## Research Report
  *   **Gusto / Rippling:** Excellent tools for traditional businesses, but they are separate platforms requiring manual data entry, integration setup, and a steep learning curve. They are not built natively into the daily operational flow (e.g., job completion, shift clock-out) of a micro-business.
  *   **Square Payroll:** Good integration with Square Point of Sale, but primarily focused on hourly shift workers rather than a flexible mix of gig contractors, piece-rate workers, and salaried staff.
  *   **OneHumanCorp (OHC) Differentiation - "Invisible Operations":** OHC will embed payroll directly into the core business ledger. When Carlos's AI agent marks a plumbing job as "Complete" and receives payment, the system autonomously calculates the subcontractor's cut, routes the funds immediately via the multi-party ledger, and logs the transaction for end-of-year tax generation. No manual payroll runs required.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      WORK_EVENT ||--o{ PAYROLL_ENGINE : "Triggers (Shift End / Job Complete)"
      PAYROLL_ENGINE }|--|| MULTI_PARTY_LEDGER : "Instructs Funds Transfer"

      PAYROLL_ENGINE {
          string spiffe_identity "Zero Trust execution"
          string tenant_id "Multi-tenant isolation"
      }

      PAYROLL_ENGINE ||--o{ TAX_COMPLIANCE_AGENT : "Generates Tax Records"
      PAYROLL_ENGINE ||--o{ NOTIFICATION_ROUTER : "Alerts Worker/Owner"

      WORKER_PROFILE ||--o{ PAYROLL_ENGINE : "Provides terms (Hourly/Split)"

      TAX_COMPLIANCE_AGENT }|--|| DOCUMENT_VAULT : "Stores W-2/1099"
  ```

  ### UI Wireframes & 375px Baseline
  **Core Layout: macOS-style Translucent Glass + Ubiquiti UniFi Modular Dashboard Cards**
  *   **Global Viewport:** 375px width (Mobile First). No horizontal scrolling.
  *   **App Bar:** Blurred glass top nav with the business logo.
  *   **Dashboard Feed:**
      *   Instead of a traditional "Run Payroll" screen, the owner sees a feed of completed actions.
      *   **Action Card:** "Plumbing Job #402 Complete. Subcontractor Joe credited $150."
      *   Each card has a frosted glass background (`rgba(255, 255, 255, 0.05)` with `backdrop-filter: blur(10px)`).
      *   **Advanced Settings (Hidden behind toggle):** Allows setting custom split percentages, overriding autonomous payouts, and downloading generated tax forms.

  ### Mobile UX Flow
  1. **Worker logs action:** A helper clocks out, or a contractor marks a task complete.
  2. **AI Verification:** The Operations AI agent verifies the completion (e.g., matching GPS location or checking task status).
  3. **Autonomous Payout:** The Payroll Engine calculates the payout based on the worker's profile terms.
  4. **Notification:** The business owner receives a push notification: "Payroll processed for today's shifts: $450 total. View details." The worker receives a notification: "You've been paid $150 for Job #402."
  5. **No manual intervention:** The owner only intervenes if there is an anomaly or dispute.

  ### AI Agent Integration Points
  *   **Operations Agent:** Tracks time, task completion, and verifies work quality before triggering payment.
  *   **Finance Agent:** Calculates exact payouts, handles tax withholdings, and interfaces with the Multi-Party Ledger to execute the transfer.
  *   **Legal/Compliance Agent:** Ensures all payouts meet local labor laws and automatically generates necessary tax documents at year-end.

  ### Key Design Decisions
  *   **Event-Driven Payroll:** Moving away from bi-weekly "payroll runs" to continuous, event-driven payouts based on job completion or shift end, improving cash flow for workers.
  *   **Zero-Trust Isolation:** Strict SPIFFE identity verification ensures that only authorized events can trigger the Payroll Engine to release funds.
  *   **Invisible Tax Compliance:** Tax calculations and form generation happen entirely in the background, abstracting the complexity from the owner.

  ## Implementation Prompt
  Design and implement the Invisible Autonomous Payroll & Contractor Payout Engine. The system must listen for work completion events (e.g., shift ended, job marked complete) and automatically calculate payouts based on predefined worker agreements. It must interface with the existing multi-party ledger to execute fund transfers securely. Furthermore, the system should autonomously generate necessary compliance records (for future tax form generation) without requiring the business owner to manually initiate a payroll run. The core user journey is: Worker completes a task -> AI verifies -> System calculates and transfers payment -> Owner is notified. Ensure strict multi-tenant data isolation and mobile-first visibility.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
