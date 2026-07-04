issue_title: "Architecture Design: Autonomous Smart Invoicing & Accounts Receivable"
issue_description: |
  # Mission Queue Protocol: Autonomous Smart Invoicing & Accounts Receivable

  ## 1. Problem Statement
  Small business owners and freelancers (e.g., Nora the Agency Principal, Carlos the Field Service Owner) spend an exorbitant amount of time managing invoices, chasing payments, and reconciling accounts. Traditional platforms like Shopify focus on point-of-sale or cart checkout but fail to address the asynchronous B2B or service-based invoicing lifecycle. Existing tools (like QuickBooks or FreshBooks) require manual configuration, dedicated login portals, and active management. The pain points include:
  - Forgotten follow-ups on overdue invoices.
  - Manual data entry from proposals to invoices.
  - Lack of a unified view of cash flow and outstanding balances.

  ## 2. Research Report
  - **Market Context:** Traditional accounting software is complex and built for accountants, not operators. Modern SMBs want software that does the work for them.
  - **Competitor Analysis:**
    - *Stripe Invoicing:* Powerful API, but requires a developer or another tool to trigger and manage smartly.
    - *QuickBooks/FreshBooks:* Comprehensive but passive; they alert you when something is overdue, but don't take action unless explicitly set up via complex workflows.
    - *Shopify:* Geared towards immediate checkout; B2B features are gated behind expensive tiers.
  - **The OHC Opportunity:** OHC can leapfrog these by integrating an autonomous Accounts Receivable agent. Instead of the owner logging in to click "Send Reminder," the Finance Assistant monitors the ledger and proactively drafts/sends localized reminders, reconciles payments automatically, and provides a daily cash flow summary in plain language.

  ## 3. Design Doc

  ### Architecture Diagram (Conceptual)
  ```mermaid
  graph TD
      A[Work Intake / Proposal] -->|Operations Agent| B(Draft Invoice)
      B -->|Owner Approval| C[Active Ledger]
      C -->|Finance Agent Monitors| D{Invoice Status}
      D -->|Due in 3 Days| E[Draft Gentle Reminder]
      D -->|Overdue| F[Draft Firm Reminder + Payment Link]
      D -->|Paid| G[Reconcile & Notify Owner]
      E -->|Auto-send or Approve| H[Customer]
      F -->|Auto-send or Approve| H
      G -->|Decision Assistant| I[Daily Cash Flow Summary]
  ```

  ### Data Model & Invariants (PostgreSQL)
  - `Invoice`: Represents a billable event (amount, currency, due date, status).
  - `LineItem`: Specific products/services on the invoice.
  - `PaymentEvent`: Webhook responses from Stripe/Payment provider linked to the invoice.
  - `LedgerEntry`: Immutable record of debits/credits for strict reconciliation.
  - *Multi-Tenant Invariant:* All tables must include `tenant_id` enforced via RLS.

  ### Mobile UX Flow (375px)
  1. **Triage Feed:** The owner opens the app and sees a card: "3 Invoices Overdue. Tap to review drafted reminders."
  2. **Review Screen:** A clean, bottom-sheet modal shows the drafted message by the Finance Agent and the outstanding amount.
  3. **Action:** One large 44x44px button: "Approve & Send" or "Edit".
  4. **Creation Flow:** A minimalist form utilizing native mobile keyboards to add line items. No complex tax configuration visible by default (handled by the agent based on locale).

  ### AI Agent Integration
  - **Finance Assistant:** Monitors the `Invoice` table. Triggers on `due_date` proximity. Uses tenant-scoped memory to determine the appropriate tone for the reminder (e.g., VIP client vs. first-time client).
  - **Operations Assistant:** When a project/task is marked 'Complete' in the work feed, it signals the Finance Assistant to draft an initial invoice based on the original quote.

  ## 4. Implementation Prompt
  **Target Persona:** Nora (Agency Principal) who needs to track milestone payments without becoming a full-time bookkeeper.
  **Task:** Implement the foundation for the Autonomous Invoicing system.
  **Acceptance Criteria:**
  - Create the underlying database schemas (`Invoices`, `LineItems`) with row-level security.
  - Build the CRUD API endpoints for invoicing (internal gRPC / external REST).
  - Implement the Mobile-first (375px) UI for viewing a list of active invoices and their statuses.
  - Integrate a background job (using PostgreSQL SKIP LOCKED) that simulates the Finance Assistant identifying an overdue invoice and generating a notification/draft reminder in the owner's Work Triage feed.
  - Ensure zero mock data in the UI; use actual database records and a documented seed script.
  - Add at least one Playwright E2E test verifying the flow from creating an invoice to seeing it in the overdue state (via time manipulation or test fixtures).

  ## 5. Priority & Scope
  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []