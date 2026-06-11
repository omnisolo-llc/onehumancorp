issue_title: "Implement Agentic Omnichannel Returns & Exchange Orchestrator"
issue_description: |
  # Research Report: Agentic Omnichannel Returns & Exchange Orchestrator

  ## 1. Problem Statement
  Boutique owners like Priya operate across in-store and online channels, meaning returns and exchanges are deeply complex. When a customer returns an online order in-store, or requests an exchange via Instagram DM, Priya has to manually update inventory, issue refunds via Stripe, recalculate tax, and restock the item. This process is error-prone, disjointed, and time-consuming. Non-technical operators need an autonomous workflow where AI handles the reverse logistics, inventory lock/release, and financial reconciliation invisibly.

  ## 2. Research & Market Context
  - **Shopify:** Handles returns but requires manual approval steps and often third-party apps (e.g., Loop Returns) to handle complex exchanges seamlessly. In-store returns of online orders often cause sync issues.
  - **Wix/GoDaddy:** Basic refund mechanisms. Lacks proactive AI coordination for restocking and customer communication.
  - **OHC Opportunity:** By leveraging the Operations Agent ("The Manager") and Finance Agent ("The Accountant"), OHC can provide a unified, zero-touch return/exchange flow. The system will automatically draft the return label, release the inventory hold, and process the Stripe refund without requiring Priya to manually execute multiple database or API updates.

  ## 3. Design Doc

  ### Architecture Overview

  ```mermaid
  sequenceDiagram
      participant Customer
      participant WorkTriage as Work Triage (AI)
      participant OperationsAgent as Operations Agent
      participant FinanceAgent as Finance Agent
      participant CentralLedger as Central DB (PostgreSQL)
      participant Stripe as Payment Gateway (Stripe)

      Customer->>WorkTriage: Requests return/exchange via DM or Portal
      WorkTriage->>OperationsAgent: Parses intent & identifies Order ID
      OperationsAgent->>CentralLedger: Validates return policy & restocks item (Optimistic Lock)
      OperationsAgent->>FinanceAgent: Triggers refund/exchange difference calculation
      FinanceAgent->>Stripe: Executes Stripe Refund or triggers Payment Link for difference
      FinanceAgent->>CentralLedger: Updates ledger (status: refunded)
      OperationsAgent->>Customer: Sends return label & confirmation
  ```

  ### Mobile UX Flow (375px)
  1. **Customer View:** A responsive web portal or DM integration where the customer selects the item to return, picks a reason, and chooses "Refund" or "Exchange". The UI is simple, fast, and optimized for 375px viewports.
  2. **Owner View (Work Feed):** Priya opens the OHC mobile app. In her prioritized work feed, she sees an actionable card: "Return requested by Sarah for Order #1042. Operations Agent has generated a return label and prepared a $45 refund. Tap 'Approve' to finalize."
  3. **Interaction:** Priya taps the large (44x44px minimum touch target) "Approve" button. The translucent glass UI transitions smoothly to "Processing", and the background agents execute the Stripe refund and inventory restock.

  ### AI Integration Points
  - **Work Triage:** Intercepts return requests from multiple channels (DMs, email, form).
  - **Operations Agent:** Reconciles inventory, marks the item as 'returned', and updates stock availability dynamically.
  - **Finance Agent:** Handles Stripe refund idempotency, tax reversals, and generates any required updated invoices.

  ## 4. Implementation Prompt
  **Feature:** Agentic Omnichannel Returns & Exchange Orchestrator
  **User Persona:** Priya (Boutique Owner)
  **Objective:** Implement the end-to-end flow for handling omnichannel returns.
  **Requirements:**
  1. Create the necessary backend API endpoints to initiate and approve a return/exchange.
  2. Integrate the Operations Agent to handle inventory restock logic upon return approval.
  3. Integrate the Finance Agent to issue Stripe refunds safely using idempotency keys.
  4. Build a mobile-first (375px) UI component in Flutter/PWA that surfaces the return request as an actionable card in the Owner Work Feed.
  5. Ensure strict tenant isolation and write comprehensive Playwright E2E tests validating the entire return lifecycle (Customer request -> Owner Approval -> Inventory Update -> Stripe Refund).
  6. The UI must use the OHC Premium Token library with translucent materials and follow the "grandmother test" for simplicity.

  ## 5. Metadata
  - **Priority:** P1
  - **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []