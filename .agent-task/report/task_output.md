issue_title: "Architecture: Unified AI-Driven Estimate & Invoicing Pipeline"
issue_description: |
  # Research Report: Unified AI-Driven Estimate & Invoicing Architecture

  ## Executive Summary
  This report details an architectural design for a new, AI-driven quoting and invoicing system for OneHumanCorp (OHC), addressing critical workflow friction for service-based owners. Our objective is to design a platform feature that seamlessly translates inbound chat inquiries into sent estimates, collected deposits, and final invoices—entirely supervised by AI agents.

  ## 1. Market Mapping & Competitor Discovery (Track 1)
  Competitors like Square, QuickBooks, and HoneyBook provide quoting tools but require manual form-filling, often disrupting the service provider's mobile-centric daily flow. For users like Carlos (field service owner) or Nora (agency principal), breaking flow to build a quote manually in a traditional SaaS dashboard means delayed responses and lost leads.

  ## 2. OHC Gap & Pain Point Identification (Track 3)
  - **Persona Focus:**
    - Carlos (Handyman) needs to read an SMS/DM request, generate an immediate quote with a deposit link, and book the job—from a 375px Android device while driving between jobs.
    - Nora (Agency Principal) needs to intake project requests, draft multi-phased proposals, and collect milestone payments.
  - **The Gap:** OHC currently lacks an integrated, multi-tenant estimate-to-invoice data pipeline tightly coupled with AI conversation agents. A service owner must manually string together chat contexts, pricing tables, and payment processors.

  ## 3. Deep Dive Architecture Design (Track 2 & Track 3)

  ### Data Model & Sync Protocol (PostgreSQL)
  - **Core Entities:** `Document` (Estimate, Proposal, Invoice), `LineItem`, `PaymentSchedule`, and `PaymentLink`.
  - **Central Ledger:** All documents live in a strongly isolated multi-tenant schema (`tenant_id` on all tables, RLS enabled).
  - **State Machine Transitions:** `DRAFT` -> `PENDING_APPROVAL` -> `SENT` -> `PARTIALLY_PAID` -> `PAID`.

  **ER Diagram:**
  ```mermaid
  erDiagram
      Tenant ||--o{ Document : creates
      Document ||--o{ LineItem : contains
      Document ||--o{ PaymentSchedule : requires
      PaymentSchedule ||--o| PaymentLink : generates

      Tenant {
          uuid id
          string name
      }
      Document {
          uuid id
          uuid tenant_id
          string status
          decimal total_amount
          string type
      }
      LineItem {
          uuid id
          uuid document_id
          string description
          decimal price
          int quantity
      }
      PaymentSchedule {
          uuid id
          uuid document_id
          decimal amount
          string due_date
          string status
      }
      PaymentLink {
          uuid id
          uuid payment_schedule_id
          string stripe_url
          string status
      }
  ```

  ### AI Agent Coordination
  - **Sales & Revenue Assistant ("The Closer"):** Ingests conversational context (e.g., "Need 3 doors painted") and historical tenant pricing, then automatically drafts an Estimate object.
  - **Customer Success Assistant:** Translates the Estimate into a plain-language proposal and presents it in-thread (via SMS/WhatsApp/WebChat) with a one-tap Stripe Checkout Payment Link for the deposit.
  - **Finance Assistant:** Observes the Stripe webhook. On deposit success, it transitions the Estimate to an active Job/Invoice and schedules a reminder for the final balance.

  **Sequence Diagram:**
  ```mermaid
  sequenceDiagram
      actor Customer
      participant Chat UI
      participant Sales Agent
      participant Ledger (DB)
      participant CS Agent
      participant Stripe
      participant Finance Agent

      Customer->>Chat UI: "Need a quote for 3 doors"
      Chat UI->>Sales Agent: Parse request
      Sales Agent->>Ledger (DB): Create Draft Estimate & Line Items
      Ledger (DB)-->>Sales Agent: Estimate ID
      Sales Agent->>CS Agent: Trigger approval workflow
      CS Agent->>Stripe: Generate Payment Link
      Stripe-->>CS Agent: Payment URL
      CS Agent->>Chat UI: Present Proposal + URL
      Customer->>Stripe: Pays Deposit
      Stripe->>Finance Agent: Webhook (payment_intent.succeeded)
      Finance Agent->>Ledger (DB): Update Status to PARTIALLY_PAID
  ```

  ### Mobile-First Implementation
  - **UI/UX Flow:**
    1. Owner receives notification: "Draft quote ready for Carlos: $450 Door Painting."
    2. Owner taps notification, views the glass-morphic translucent summary card on a 375px screen.
    3. Large (44x44px min) buttons: "Send as-is" or "Edit Items".
  - **Zero Trust:** Ensure SPIFFE/SPIRE validates the Sales Agent's right to draft an invoice for that specific tenant.

  ## 4. Proposed Implementation Steps & Issue Prompt

  **Issue Prompt for Implementer Agent:**
  Implement the AI-Driven Estimate & Invoicing Pipeline:
  1. Create the `Estimate`, `LineItem`, and `PaymentSchedule` proto definitions and corresponding PostgreSQL schema migrations with RLS (`tenant_id`).
  2. Implement the gRPC API service for the Estimate lifecycle (`CreateEstimate`, `ApproveEstimate`, `SendEstimate`).
  3. Integrate the Sales Agent to generate a draft `Estimate` from natural language input.
  4. Build a Flutter mobile-first view (375px base) using the OHC Premium Token library to display a translucent glass summary card of the draft estimate, with large touch targets for approval.
  5. Add full E2E Playwright test coverage simulating the owner receiving a draft, approving it, and the system generating a Stripe-compatible payment link stub.

  Ensure all UI uses the macOS Translucent Glass aesthetic and UniFi-style data cards. No hardcoded mock data.

  **Estimated Scope**: Large
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []