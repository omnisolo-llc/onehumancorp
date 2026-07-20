issue_title: "Agentic Automated Receivables & Smart Invoicing"
issue_description: |
  # Agentic Automated Receivables & Smart Invoicing

  ## Problem Statement
  Small business operators, particularly independent professionals and agency principals like Nora, spend an excessive amount of time chasing unpaid invoices, negotiating payment terms, and reconciling bank transfers. Traditional platforms (e.g., QuickBooks, FreshBooks) offer static recurring invoices and generic email reminders, but they require manual setup for each client and lack situational awareness. When an invoice goes past due, the owner has to awkwardly intervene, risking the client relationship. The gap is the absence of an autonomous, context-aware receivables system that dynamically adjusts follow-ups based on client history, drafts personalized reminders, and automatically reconciles incoming payments without manual data entry.

  ## Research Report
  - **Market Context**: Platforms like Xero and QuickBooks dominate SMB accounting, but their invoicing modules are rigid. They send templated "Invoice #1024 is overdue" emails that often end up in spam or alienate clients. Stripe Invoicing is powerful but lacks a natural language agent to negotiate or politely remind clients via SMS or WhatsApp.
  - **The OHC Opportunity**: By integrating the Finance Agent and Customer Success Agent directly into the invoicing ledger, OHC can offer "Smart Receivables." The platform doesn't just send an invoice; it actively works to get it paid. If a traditionally prompt client is late, the agent drafts a gentle check-in. If a chronic late-payer is overdue, the agent can automatically suggest stricter terms or a slight late fee for future engagements, all while keeping the owner (Nora) informed via the unified Agent Feed.
  - **Competitor Analysis**:
    - *Shopify*: Primarily B2C retail; weak B2B invoicing without expensive apps.
    - *QuickBooks*: Manual workflows; reminders are robotic and lack AI personalization.
    - *HoneyBook*: Good for freelancers, but lacks deeply autonomous agentic negotiation and multi-channel follow-up (e.g., WhatsApp).

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  graph TD
      A[Owner / Nora] -->|Creates Project/Proposal| B(OHC Invoicing Engine)
      B --> C[PostgreSQL Ledger]
      C --> D{Payment Due Trigger}
      D -->|On Time| E[Payment Gateway / Stripe]
      D -->|Overdue| F[Finance Agent]
      F --> G[Customer Success Agent]
      G -->|Drafts Personalized Reminder| H[Agent Feed Action Card]
      H -->|Nora Approves| I[Omnichannel Delivery: SMS/Email/WhatsApp]
      E --> J[Auto-Reconciliation]
      J --> K[Update Ledger & Notify Owner]
  ```

  ### Mobile UX Flow (375px First)
  1. **Invoice Creation (Mobile)**: Nora taps "New Invoice" on her 375px viewport. The Finance Agent pre-fills line items based on the approved project proposal. Large, touch-friendly inputs (>=44x44px) allow for quick adjustments.
  2. **Agent Feed Alert**: Three days after the due date, Nora receives an Action Card in her Agent Feed: "Acme Corp is 3 days late on Invoice #102. They are usually prompt. Should I send this gentle check-in?"
  3. **One-Tap Approval**: The card displays the drafted message. Nora taps a primary, high-contrast "Approve & Send" button.
  4. **Payment & Reconciliation**: Acme Corp pays via the embedded Stripe link. The app immediately sends a push notification to Nora with a playful "Ka-ching" animation, updating the project's financial health score.

  ### AI Agent Integration Points
  - **The Finance Agent**: Monitors the `invoices` and `ledger_reserves` tables. Identifies anomalies in payment patterns and flags overdue accounts.
  - **The Customer Success Agent**: Ingests context from the CRM and past communication history to draft highly empathetic, tailored payment reminders. It avoids aggressive language for VIP clients while being firm with repeat offenders.

  ### Key Design Decisions
  - **Multi-Tenant Row-Level Security (RLS)**: All invoice and payment records must strictly enforce `tenant_id` isolation at the PostgreSQL level.
  - **Zero Trust / Ephemeral Links**: Invoice payment links sent to clients must be cryptographically signed, stateless, and expire after a configurable duration to prevent unauthorized access to the business's financial data.
  - **Invisible Automation**: The system defaults to drafting and asking for approval via the Agent Feed rather than sending automatically, ensuring Nora remains in control of sensitive client communications until she explicitly enables "Auto-Pilot" for trusted clients.

  ## Implementation Prompt
  **Target Persona**: Nora (Agency Principal)
  **Feature**: Agentic Automated Receivables
  **Acceptance Criteria**:
  1. Implement a CRON-like background worker (using the existing `ohc_job_queue` pattern) that scans for overdue invoices daily.
  2. Integrate the Finance and Customer Success Agents to generate personalized reminder drafts based on the client's payment history.
  3. Surface these drafts as Action Cards in the Owner's mobile-first Agent Feed.
  4. Upon owner approval, dispatch the message via the configured omnichannel webhook (Email/SMS).
  5. Ensure all database interactions for this feature are covered by strict RLS policies. Do not prescribe the exact database schema changes; let the implementer define the necessary `invoice_reminders` or `agent_actions` tables.

  ## Priority
  P1

  ## Estimated Scope
  Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
