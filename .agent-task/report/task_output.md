issue_title: "Agentic B2B Project Intake & Milestone Billing Workflow"
issue_description: |
  # Research Report: Agentic B2B Project Intake & Milestone Billing Workflow

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  The current OHC platform excels at simple transactions, appointments, and e-commerce (e.g., Maya's cakes, Carlos's handyman quotes, Priya's boutique). However, service-based B2B operators and agencies struggle with disjointed workflows. Platforms like HubSpot or HoneyBook offer project management and billing, but they require heavy manual configuration and lack invisible AI automation. Nora (Agency Principal) currently has to jump between intake forms, a CRM, a proposal drafting tool (like Google Docs/Notion), and an invoicing platform (Stripe/QuickBooks).

  ## 2. OHC Gap & Pain Point Identification (Track 1)
  - **Persona Focus:** Nora (Agency Principal). She runs a small design studio with contractors and clients.
  - **The Gap:** OHC lacks a robust "Project" primitive capable of managing multi-phase work. Nora needs an automated flow from client intake to proposal generation, milestone tracking, and automated milestone invoicing. Currently, the OHC quote system is too simple for multi-stage SOWs (Statements of Work) and lacks milestone-triggered billing.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Sync Protocol
  - **Central Ledger (PostgreSQL):** Introduce new core entities: `Project`, `Milestone`, and `MilestoneInvoice`.
  - **Strict Multi-Tenant Isolation:** All queries strictly enforce `tenant_id` for row-level security.
  - **Stripe Integration:** `MilestoneInvoice` links to Stripe Payment Intents or Invoices, supporting partial payments (deposits) and milestone-based collections.

  ### AI Agent Coordination
  - **Sales Agent ("The Negotiator"):** Ingests client intake forms (via email or web widget). It cross-references Nora's past projects (Knowledge Assistant RAG) to draft a comprehensive Statement of Work (SOW) and proposed milestones.
  - **Operations Agent ("The Manager"):** Tracks project progress. When a contractor marks a task complete, it flags the associated milestone as ready for review.
  - **Finance Agent ("The Accountant"):** Upon milestone approval by Nora, the Finance Agent automatically drafts and queues a Stripe invoice for the client, sending an "Action Card" to Nora's feed for 1-tap approval.

  ### System Architecture & Journey Flow
  ```mermaid
  erDiagram
      Project ||--o{ Milestone : "contains"
      Milestone ||--o{ MilestoneInvoice : "generates"
      Project {
          uuid id
          uuid tenant_id
          string name
          string status
      }
      Milestone {
          uuid id
          uuid project_id
          string title
          string status
      }
      MilestoneInvoice {
          uuid id
          uuid milestone_id
          string stripe_invoice_id
          int amount_cents
      }
  ```

  ```mermaid
  sequenceDiagram
      actor Client
      actor SalesAgent
      actor Owner
      actor OperationsAgent
      actor FinanceAgent

      Client->>SalesAgent: Submits Intake Form
      SalesAgent->>Owner: Drafts Proposal & Pushes Action Card
      Owner->>SalesAgent: Approves Proposal
      SalesAgent->>Client: Sends SOW + Deposit Link
      Client->>SalesAgent: Pays Deposit
      OperationsAgent->>Owner: Marks Project Active
      Note over OperationsAgent,Owner: Time passes, work completes
      OperationsAgent->>Owner: Flags Milestone Ready
      Owner->>FinanceAgent: Approves Milestone
      FinanceAgent->>Client: Generates & Sends Stripe Invoice
  ```

  ### Mobile-First Implementation
  - **Feed-Centric UX (375px):** Nora manages projects via her Agent Feed. She receives action cards like: "Client XYZ requested a rebrand. Proposal drafted based on previous projects. [Review & Send]".
  - **Milestone Cards:** "Milestone 1 (Wireframes) complete. Send $2,000 invoice?" with 44x44px touch targets for [Send Invoice] or [Edit].

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Feature Name:** Agentic B2B Project Intake & Milestone Billing

  **Target Persona:** Nora the Agency Principal

  **Outcome:** Nora can capture client demand, approve an AI-drafted multi-milestone proposal, and collect payments at each stage with zero manual data entry across different tools.

  **Critical User Journey (CUJ):**
  1. A prospective client fills out an intake form on Nora's OHC-powered site requesting a "website redesign".
  2. The Sales Agent drafts a Proposal containing 3 Milestones (Deposit, Design, Launch) and pushes a review card to Nora's mobile feed.
  3. Nora taps "Approve". The proposal is sent to the client with a Stripe deposit link.
  4. The client pays the deposit. The Operations Agent marks Milestone 1 active and creates backend tasks.
  5. Upon design completion, Nora taps a button to complete Milestone 2; the Finance Agent instantly sends the next invoice.

  **Next Actions for Engineering (Implementer Prompt):**
  - **Step 1:** Create PostgreSQL schemas for `projects`, `milestones`, and `milestone_invoices`, strictly isolated by `tenant_id`.
  - **Step 2:** Develop the backend service layer (Go + Bazel) to handle project state transitions and integrate Stripe billing for milestone payments.
  - **Step 3:** Extend the Sales Agent capability to ingest unstructured intake data and output structured Proposal JSON (milestones, pricing) using the LLM.
  - **Step 4:** Build the mobile-first (375px) "Action Cards" in the UI to allow Nora to review drafted proposals and approve milestone invoices with 1-tap.

  **Design Considerations:**
  - Maintain the Translucent Glass UI mandate for all proposal review screens.
  - No explicit technical terms exposed to the user.

  **Priority:** P1
  **Estimated Scope:** Large
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []