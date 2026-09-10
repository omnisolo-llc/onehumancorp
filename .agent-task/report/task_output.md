issue_title: "Architecture Design: Autonomous Predictive Cash Flow & Tax Liability Forecasting Engine"
issue_description: |
  Priority
  P0

  Estimated Scope
  Large

  Problem Statement
  For solo operators (like Maya the baker and Carlos the handyman), managing cash flow, estimating quarterly taxes, and maintaining double-entry ledgers is an administrative nightmare that requires specialized knowledge they do not have. They often find themselves short on cash for supplies or hit with unexpected tax bills at the end of the year. The current platform lacks an integrated, autonomous engine that continuously analyzes incoming revenue, upcoming expenses (e.g., automated POs, payroll), and seasonal trends to project runway and automatically reserve estimated tax liabilities.

  Research Report
  Market Context & Findings
  1. QuickBooks Online / Xero: Require significant manual reconciliation and setup. They offer basic cash flow dashboards but lack predictive, agent-driven insights that actively suggest interventions (e.g., "delay purchasing flour by 2 weeks").
  2. Found / Novo: Modern banking solutions with integrated tax holding, but they only see the bank feed, not the operational context (like pending proposals or upcoming shift schedules).
  3. Stripe Tax / Avalara: Great for sales tax compliance but do not handle holistic business cash flow or quarterly income tax estimates based on comprehensive operational data.
  4. The OmniSolo Advantage: Because OHC orchestrates the entire business (scheduling, POS, CRM, auto-POs), an AI agent can forecast cash flow not just on historical bank data, but on real-time operational intents (e.g., an accepted proposal that hasn't been invoiced yet).

  Operator Context
  - Carlos (Handyman) needs to know if he has enough cash to buy materials for a large job next week while still paying his quarterly taxes.
  - Priya (Boutique Owner) needs AI to analyze seasonal sales dips and recommend exactly how much to hold back for slow months.

  Design Doc
  High-Level System Architecture
  The Autonomous Predictive Cash Flow Engine relies on the Accounting & Tax Agent observing events across the platform via the Distributed AI Job Queue.

  erDiagram
      Tenant ||--o{ BankTransaction : has
      Tenant ||--o{ Invoice : has
      Tenant ||--o{ PurchaseOrder : has
      Tenant ||--o{ TaxEstimate : holds
      Tenant ||--o{ CashFlowProjection : projects
      BankTransaction }|--|| LedgerEntry : generates
      Invoice }|--|| LedgerEntry : generates
      PurchaseOrder }|--|| LedgerEntry : generates

  sequenceDiagram
      autonumber
      participant Ops as Operational Event (Invoice/PO)
      participant Queue as Job Queue (Redis/Pg)
      participant Agent as Accounting & Tax Agent
      participant Ledger as Double-Entry Ledger (PostgreSQL)
      participant Projection as Cash Flow Engine
      participant DB as Database (PostgreSQL RLS)

      Ops->>Queue: Publish Financial Event
      Queue->>Agent: Consume Event
      Agent->>Ledger: Reconcile / Categorize Entry
      Agent->>Projection: Trigger Recalculation
      Projection->>DB: Update 90-Day Cash Flow Projection
      Projection->>DB: Update Estimated Tax Liability

  Core Components
  1. Double-Entry Ledger (PostgreSQL): An immutable append-only ledger enforcing strict multi-tenant isolation via RLS (tenant_id).
  2. Accounting & Tax Agent (Rust/Harness): Continuously listens to operational events (invoices issued, POs approved, payroll runs scheduled) and categorizes them automatically.
  3. Forecasting Engine (Rust): Uses historical data and forward-looking operational events to project cash flow 30/60/90 days out.
  4. Zero-Trust Identity: Agent workers authenticate to the multi-tenant DB using SPIFFE/SPIRE temporary credentials.

  Mobile UX Flow (375px First)
  - Home Dashboard: A simple, clear "Cash Available" card at the top. Below it, a "Safe to Spend" metric that automatically subtracts tax liabilities and upcoming automated bills.
  - Tax Insights Card: "We've reserved $1,200 for your Q3 taxes. You're on track."
  - Alerts: Push notification: "Carlos, buying those materials today will drop you below your safe margin for next week's payroll. Wait until Monday after the Smith invoice clears."
  - Zero Jargon: No terms like "Assets", "Liabilities", "EBITDA", or "Reconciliation" unless toggled in Advanced Settings.

  AI Agent Integration Points
  - Accounting & Tax Agent: Runs on the universal multi-harness platform. Uses standard LLM reasoning (e.g., gpt-5.6-luna) to categorize obscure bank feed descriptions (e.g., "SQ Bobs Hardware" -> "Materials & Supplies").
  - Scheduler & Productivity Agent: Coordinates with the Accounting Agent to delay non-urgent purchase tasks if cash flow is projected to be tight.

  Implementation Prompt
  To the Implementer Swarm:
  Implement the backend core for the Autonomous Predictive Cash Flow & Tax Liability Forecasting Engine. Focus on building the resilient, append-only PostgreSQL double-entry ledger with strict RLS (tenant_id), and the event listener for the Accounting & Tax Agent in the Rust omnichannel backend.

  Acceptance Criteria:
  1. Ledger Reliability: Define the schema for the immutable ledger ensuring every transaction balances and belongs to a specific tenant_id (RLS enforced).
  2. Event Ingestion: Create the Rust consumer that reads from the Redis/PostgreSQL queue, parsing operational events (like an accepted proposal or paid POS transaction) into ledger entries.
  3. Forecasting Logic: Implement the core Rust service that calculates a 90-day projection based on current ledger balance + pending accounts receivable/payable.
  4. Testing: 100% unit test coverage for the ledger math and isolation. Playwright E2E tests validating the "Safe to Spend" projection updates when a new invoice is created.
  5. No implementation of UI jargon: The data output must be clean and ready for the 375px mobile "Safe to Spend" view.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
