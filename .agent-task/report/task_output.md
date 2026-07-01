issue_title: "[Research] Autonomous Agentic Invoice Follow-up & Debt Collection"
issue_description: |
  # OHC Autonomous Invoice Follow-up: Agentic Workflow Deep Dive

  ## Target Persona: Nora (Agency Principal) & Carlos (Field Service Owner)

  ## Problem Statement
  Following up on unpaid invoices is awkward, time-consuming, and often neglected by SMB owners, leading to severe cash flow problems. Existing tools like QuickBooks or FreshBooks offer rigid, static automated emails that feel robotic and can damage delicate client relationships. Owners need an assistant that handles collections gracefully and autonomously.

  ## Research Report & Gap Analysis
  - **Traditional Accounting Software**: Rely on static email templates triggered by fixed time intervals (e.g., 3 days overdue). These lack context and cannot negotiate or answer client questions about the invoice.
  - **The OHC Agentic Approach**: Instead of static emails, an AI Finance Agent monitors the ledger. When an invoice becomes overdue, it generates a highly personalized, context-aware reminder (e.g., referencing a recent successful project milestone or apologizing for the wait). The agent can handle replies, negotiate payment plans if authorized, and deliver messages via the client's preferred channel (Email/SMS/WhatsApp).

  ## Architecture & Design Flow

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant Ledger as PostgreSQL Ledger
      participant Worker as AI Job Queue Worker
      participant Agent as Finance AI Agent (Gemini)
      participant UI as Owner Mobile Feed (375px)
      participant Client as Client Channel (SMS/Email)

      Ledger->>Worker: Identify overdue invoice (Tenant Scope)
      Worker->>Agent: Fetch client context & invoice details
      Agent->>Agent: Generate contextual reminder draft
      Agent->>UI: Push 375px card to Work Triage Feed
      UI-->>Agent: Owner Taps "Approve & Send"
      Agent->>Client: Dispatch personalized reminder
      Client-->>Agent: Client replies (e.g., "Need 3 more days")
      Agent->>UI: Push negotiation card to Owner Feed
  ```

  ### Mobile UX
  - **Owner Review (Mobile UX)**: The drafted reminder is pushed to the owner's mobile "Work Triage" feed.
    - The UI is a clean 375px card stating: "Client X is 3 days late on $500 invoice. Send this drafted reminder?"
    - Actions: "Approve & Send", "Edit Draft", "Snooze 3 Days".
    - The card clearly highlights the impact on cash flow.

  ## Implementation Prompt
  - Implement a backend job using the AI Job Queue to identify overdue invoices.
  - Integrate with the LLM (Gemini Pro) to generate personalized, context-aware reminder copy based on client history.
  - Build the mobile-first (375px) feed card component allowing the owner to review, edit, or approve the generated reminder with a single tap.
  - Ensure all database queries respect tenant isolation rules.

  ## Codebase Audit Findings (Top 5 Confusing/Inconsistent Areas)
  During repository discovery, the following 5 areas lacked clarity or seemed confusing and should be addressed in future optimizations:
  1. The `visual_workflow.rs` implementation mentions "no-code assembly" but the schema heavily relies on raw Rust macros without a clear UI definition path.
  2. The separation between the Go API (`src/server/api`) and the Rust backend logic (`src/server/ohc`) is undocumented regarding service boundaries (gRPC vs FFI).
  3. `docs/business/market_research` contains many unstructured files with different naming conventions (e.g. `[research]_...` vs. `ux_analysis_...`).
  4. The testing harness mandates `bazelisk test //...` but times out frequently on simple runs without localized caching instructions.
  5. The mobile layout breakpoints in the `playwright.config.ts` do not strictly enforce the 375px minimum required by the design spec in all suites.

  ## Priority & Scope
  - **Priority**: P1
  - **Estimated Scope**: Medium
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
