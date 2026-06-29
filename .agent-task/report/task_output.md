issue_title: "Automated Milestone Billing & Autonomous Invoice Chasing"
issue_description: |
  # Research Report: Automated Milestone Billing & Autonomous Invoice Chasing Architecture

  ## Problem Statement
  Service-based businesses and agencies, like Nora (Agency Principal), rely heavily on milestone-based project billing. They currently waste hours tracking project progress across fragmented tools, drafting invoices, and manually chasing clients for overdue payments. This administrative burden causes delayed revenue and damages client relationships when "chasing" becomes awkward. Nora needs an assistant that inherently links project milestones to invoicing and autonomously handles follow-ups without requiring her intervention or sounding like a robotic demand.

  ## Research Findings
  Our user personas (particularly Nora and Leo) suffer from cash flow delays because of manual invoicing friction.

  ### Competitive Analysis
  - **FreshBooks & QuickBooks:** Good at recurring invoices and automated reminders, but they are generic and detached from actual project work (milestone completion). They require the owner to manually mark a phase "done" in one system and then issue an invoice in another.
  - **HoneyBook / Dubsado:** Excellent at workflow automation for freelancers, but setup is notoriously complex. Users must build "workflows" manually with rigid rules.
  - **Shopify / E-commerce Builders:** Completely unsuited for milestone-based service billing.
  - **Stripe Billing:** Powerful API, but no built-in project management for the end user.
  - **OHC Differentiation:** OHC will merge project task completion with billing. The "Finance Assistant" and "Operations Assistant" agents will coordinate. When Operations marks a milestone complete, Finance will draft the invoice, request Nora's approval (1-tap on mobile), and then autonomously handle the polite, context-aware chasing of the client.

  ### Data & References
  - Studies indicate small businesses spend up to 15 hours a week chasing payments.
  - Automating invoice reminders can decrease late payments by over 30%.
  - Our personas demand a "zero-setup" workflow where the AI deduces the milestone rules from the initial project proposal.

  ## Architectural Design

  ### System Overview

  ```mermaid
  graph TD
      subgraph Frontend "Flutter App / PWA (Mobile-First 375px)"
          Dashboard[Owner Feed / Dashboard]
          ProjectView[Project & Milestone View]
      end

      subgraph Backend "Go + Bazel Backend"
          API[API Gateway gRPC/REST]
          ProjectEngine[Project State Machine]
          BillingEngine[Billing & Invoicing Engine]
      end

      subgraph AI "AI Agent Departments"
          OpsAgent[Operations Agent]
          FinanceAgent[Finance Agent]
          CommsAgent[Customer Assistant]
      end

      subgraph External
          Stripe[Stripe Checkout / Billing]
          EmailSMS[Email / SMS Gateway]
      end

      Dashboard <--> API
      ProjectView <--> API

      API --> ProjectEngine
      API --> BillingEngine

      ProjectEngine -- Milestone Completed --> OpsAgent
      OpsAgent -- Triggers --> FinanceAgent
      FinanceAgent --> BillingEngine
      BillingEngine --> Stripe

      FinanceAgent -- Monitors Overdue --> CommsAgent
      CommsAgent -- Drafts Chaser --> EmailSMS
  ```

  ### Core Data Model (PostgreSQL)
  - `Project`: Represents the overall client engagement.
  - `Milestone`: Represents a phase of the project, linked to `Project`. Contains `status` (pending, active, completed) and `billing_amount`.
  - `Invoice`: Linked to `Milestone`. Contains Stripe invoice ID, `status` (draft, sent, paid, overdue), and `due_date`.
  - `CommunicationThread`: Tracks the history of the "chasing" conversation to ensure the AI has full context.

  ### AI Agent Coordination
  - **Operations Assistant**: Tracks task completion. When the final task in a milestone is checked off (e.g., "Deliver final logo files"), it updates the milestone status and alerts the Finance Assistant.
  - **Finance Assistant**: Detects the completed milestone. Drafts the invoice via Stripe API. Sends a push notification to the owner (Nora): "Milestone 1 complete. Send $500 invoice to Client X?"
  - **Customer Assistant (The Chaser)**: If an invoice is 3 days overdue, this agent drafts a polite, context-aware email. It checks the CommunicationThread to ensure it doesn't sound repetitive. It can even answer client replies like "Can I pay next week?" by negotiating based on owner-defined parameters.

  ### Mobile-First UX Flow (375px)
  - **The Work Feed**: The primary interface is the owner's feed.
  - **Card: "Invoice Ready"**: A clean card appears: "Project Alpha: Design Phase complete. Invoice for $1,200 is drafted."
  - **Interaction**: One large "Approve & Send" button, and a secondary "View Draft" button.
  - **Card: "Payment Overdue"**: "Client Y is 5 days late on $400. I've drafted a polite check-in."
  - **Interaction**: "Send" or "Edit". The mobile keyboard opens natively if editing.
  - **Touch Targets**: Minimum 44x44px. Use OHC Premium Token library (Glassmorphism tokens for elevation and clarity).

  ### Security & Multi-Tenancy
  - All database queries for Projects and Invoices must explicitly filter by `tenant_id` enforcing row-level security.
  - Stripe Webhooks must be verified using the endpoint secret and idempotency keys to prevent duplicate payments.

  ## Implementation Prompt

  **Feature Name:** Autonomous Milestone Billing & Invoice Chasing
  **Target Persona:** Nora the Agency Principal

  **Outcome:** Nora creates a project with 3 milestones. When she marks milestone 1 as complete on her phone, OHC immediately drafts the invoice for approval. If the client doesn't pay, OHC follows up politely on Nora's behalf.

  **Critical User Journey (CUJ):**
  1. Nora is logged into the OHC mobile app. She views a project and taps the checkbox next to the "Finalize wireframes" task, completing the milestone.
  2. The Operations agent flags the milestone as complete and triggers the Finance agent.
  3. A new card appears at the top of Nora's feed: "Wireframes complete. Send $1,500 invoice to Acme Corp?"
  4. Nora taps "Approve".
  5. Fast forward 7 days: The invoice is past due. The Customer Assistant agent drafts a polite follow-up email.
  6. A new card appears in Nora's feed: "Acme Corp is 2 days late. Send drafted follow-up?" Nora taps "Approve".

  **Acceptance Criteria for Implementer:**
  - Build the PostgreSQL schema for Milestones and link them to Invoices.
  - Implement the Stripe Invoice creation integration in the Go backend.
  - Create the AI trigger flow: Task Completion -> Milestone Completion -> Invoice Draft.
  - Build the 375px mobile UI card for the Flutter app allowing 1-tap approval.
  - Do NOT prescribe the exact LLM prompt; allow the Customer Assistant to use its standard context-aware drafting.
  - E2E Playwright test must cover checking off the task and approving the resulting invoice.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
