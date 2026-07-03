issue_title: "Implement Autonomous Billing & Invoice Recovery Agent"
issue_description: |
  # Research Report: Autonomous Billing & Invoice Recovery Agent

  **Author:** Principal Product Researcher
  **Status:** Published
  **Date:** 2024-07-02

  ## 1. Problem Statement
  Small business owners and freelancers (like Nora, the Agency Principal, and Carlos, the Handyman) spend a disproportionate amount of time chasing unpaid invoices, managing deposits, and reconciling payments. They lack the resources for a dedicated accounts receivable department, leading to cash flow issues and damaged client relationships due to awkward manual follow-ups. Existing tools (Stripe Billing, Quickbooks) provide the rails but still require the owner to manually initiate reminders, negotiate payment plans, or identify at-risk clients.

  ## 2. Research Report
  - **Market Context:** Delayed payments are a top stressor for SMBs. Tools like Stripe send automated dunning emails, but these are often rigid, impersonal, and easily ignored by clients.
  - **Competitive Landscape:**
      - **Stripe / Square:** Excellent payment processing and basic recurring billing, but the automated emails lack conversational intelligence and negotiation capabilities.
      - **Quickbooks / Xero:** Powerful accounting, but they act as passive systems of record rather than proactive agents.
      - **Dedicated AR SaaS (e.g., Chaser, Kolleno):** Powerful but complex and expensive, targeting mid-market rather than micro-SMBs.
  - **The Gap:** OHC needs a "Finance Agent" that doesn't just send a generic "Invoice Overdue" email, but acts as a polite, persistent, and intelligent accounts receivable clerk. It should handle deposits, remind clients of upcoming milestones, gently follow up on overdue invoices, and escalate to the owner only when necessary or when an anomaly (e.g., a client asking for a payment plan) occurs.

  ## 3. Design Doc

  ### Architecture
  ```mermaid
  sequenceDiagram
      participant Cron as Scheduler (Postgres Job Queue)
      participant FA as Finance Agent (LLM)
      participant Ledger as Central Ledger
      participant Stripe as Payment Gateway
      participant Client as End Customer
      participant Owner as OHC Agent Feed

      Cron->>FA: Trigger Daily AR Check
      FA->>Ledger: Query Unpaid & Overdue Invoices
      Ledger-->>FA: Return Invoice List
      loop Each Overdue Invoice
          FA->>FA: Analyze Client History & Invoice Age
          FA->>FA: Draft Contextual Reminder
          FA->>Client: Send Gentle Reminder (Email/SMS) via Ambassador Agent Integration
          Client-->>FA: Reply (e.g., "Can I pay half now?")
          FA->>FA: Classify Intent (Negotiation/Exception)
          FA->>Owner: Push Action Card to Agent Feed ("Client X requests split payment. Approve?")
      end
  ```

  ### Mobile UX Flow (375px)
  1. The Finance Agent operates invisibly in the background.
  2. When an anomaly occurs (e.g., a high-value invoice is 30 days late, or a client replies to a reminder), the agent pushes an Action Card to the owner's Agent Feed on their mobile device.
  3. **Action Card UI:**
      - Title: "Invoice #104 Overdue (Action Required)"
      - Context: "Client ACME Corp is 14 days late on $500. They replied: 'Waiting on our own clients, can pay next week.'"
      - Agent Proposal: "Drafted reply: 'No problem, we will pause the late fee. Please pay by next Friday. Link: [Payment Link]'"
      - Action Buttons: `[ Approve & Send ]`, `[ Edit ]`, `[ Escalate to Call ]`.
  4. The owner taps `[ Approve & Send ]` with zero typing.

  ### AI Agent Integration
  - **Department:** Finance / Operations.
  - **Capabilities:** Requires access to the Ledger service (to check invoice status) and the Communications service (to send/receive emails/SMS). Needs a specific system prompt emphasizing a polite but firm tone, escalating edge cases to the owner.

  ## 4. Implementation Prompt
  **Role:** Backend / Full-Stack Engineer

  **Task:** Implement the core infrastructure for the Autonomous Billing & Invoice Recovery Agent.

  **Critical User Journey (CUJ):**
  1. Nora (Agency Principal) completes a project and issues a $1,000 invoice via OHC.
  2. The invoice becomes 3 days overdue.
  3. A scheduled background job triggers the Finance Agent.
  4. The Finance Agent reviews the ledger, sees the overdue status, and autonomously drafts and sends a polite, contextual email to the client using a secure payment link.
  5. The client replies to the email saying, "I lost my credit card, can I pay via ACH?"
  6. The Finance Agent interprets this, updates the invoice to allow ACH, generates the ACH instructions, drafts a reply, and surfaces it to Nora's mobile feed as an Action Card for 1-tap approval.

  **Requirements:**
  - Create a robust scheduled job (e.g., using a Postgres-backed queue) to run the daily AR analysis.
  - Define the `Invoice` and `ClientInteraction` schema extensions if necessary.
  - Implement the LLM workflow to generate the reminder and parse client responses.
  - Build the API endpoint that surfaces these anomalies as Action Cards in the Agent Feed.
  - Write Playwright E2E tests simulating an overdue invoice, the automated reminder, and a client response triggering an Action Card for the owner.
  - Ensure all database queries strictly enforce `tenant_id` isolation.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []