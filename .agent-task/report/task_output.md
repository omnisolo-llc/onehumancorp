issue_title: "[architecture]_autonomous_tip_distribution_and_payroll_ledger"
issue_description: |
  # Architecture Brief: Autonomous Tip Distribution and Payroll Ledger

  ## Problem Statement
  For service-based solopreneurs and small teams like **Fatima (Food Cart Operator, 50)**, collecting, reconciling, and distributing tips is a manual, error-prone, and stressful process. Tips processed via credit cards often mix with gross revenue, making tax time complicated. For those with part-time helpers, splitting tips based on shift hours requires complex spreadsheets or expensive external payroll software. Existing platforms like Square offer tip pooling but require complex manual shift management and are tightly coupled to expensive proprietary hardware. The platform needs a zero-friction, automated tip management system.

  ## Research Report
  - **The "Tip Accounting Friction":** Analysis of food & beverage and service SMBs shows that owners spend up to 3 hours weekly just reconciling tips from revenue and distributing them to staff.
  - **Competitor Gaps:**
    - **Shopify POS:** Lacks native, complex tip-pooling without third-party apps, which adds monthly overhead.
    - **Wix/Squarespace:** Extremely limited in-person POS capabilities, virtually no native tip distribution for staff.
    - **Square:** Has tip pooling, but it requires manual shift-end reporting and is English-first, confusing non-native speakers like Fatima.
  - **The OHC Opportunity:** By utilizing the KAIROS Hybrid Event Mesh and the AI Finance Agent, OHC can instantly identify tip portions of every transaction, route them to an isolated tip ledger (via `tenant_id` and `worker_id`), and autonomously execute instant payouts at the end of each shift without the owner ever touching a spreadsheet.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  erDiagram
      TENANT ||--o{ SHIFT : manages
      SHIFT ||--o{ TRANSACTION : includes
      TRANSACTION ||--o{ TIP_LEDGER : allocates
      TIP_LEDGER ||--o{ PAYOUT : generates
      WORKER ||--o{ TIP_LEDGER : receives

      TRANSACTION {
          string id PK
          string tenant_id FK
          float base_amount
          float tip_amount
      }
      TIP_LEDGER {
          string id PK
          string transaction_id FK
          string worker_id FK
          float allocated_amount
      }
  ```

  ### Core Components
  1. **Real-time Tip Ledger**: A high-throughput database table (`TIP_LEDGER`) that instantly segregates tip funds from gross revenue the moment a payment clears via the Stripe Terminal or online checkout.
  2. **Shift-Aware Event Mesh**: The Operations Agent monitors active shifts. When a transaction occurs, the event mesh immediately calculates the tip split based on active workers and records the liability.
  3. **Autonomous Finance Payouts**: The Finance & Payments Agent automatically triggers Stripe Connect or instant virtual card payouts at the end of the shift, sending an SMS to the worker (e.g., "You earned $45 in tips today!").

  ### Mobile UX Flow (375px)
  1. **Daily Summary**: Fatima opens the app and sees a clean Dashboard Card: "Today's Revenue: $400 | Tips: $65".
  2. **1-Tap Payout**: She clicks "Close Shift".
  3. **AI Confirmation**: The Finance Agent displays a Glassmorphism modal: "Shift closed. $65 in tips automatically sent to your linked account."

  ### AI Agent Integration
  - **Finance & Payments Agent**: Manages the ledger isolation and triggers Stripe API calls for payouts. Generates a simple text-based weekly report: "You collected $200 in tips this week. It has been set aside from your taxable revenue."
  - **Operations Agent**: Tracks when the food cart is "Open" or "Closed" to bound the shift timing without manual punch-ins.

  ## Implementation Prompt
  Implement the backend data models and gRPC API layer for the Autonomous Tip Ledger. Ensure row-level security by `tenant_id`. Create the `AllocateTip` and `ExecuteShiftPayouts` handlers which natively interface with the existing Stripe integration layer and the Event Mesh. Provide a basic Flutter frontend view for the "End of Shift" modal adhering to the 375px mobile-first constraint, using Translucent Glass design tokens. Add unit tests achieving 100% coverage for the tip calculation logic and write one Playwright E2E test verifying the shift-closure tip payout UI flow.

  ## Priority
  P1

  ## Estimated Scope
  Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
