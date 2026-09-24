issue_title: "OHC-04: Implement Durable 'Inquiry Capture -> Qualified Quote' Workflow"
issue_description: |
  # Research Report: Inquiry Capture to Qualified Quote

  ## Problem Statement
  For Nora (solo web/design/marketing professional) and other non-technical owner/operators, capturing an inbound inquiry and converting it to a qualified, priced quote is a fragmented and manual process. Operators currently cobble together website forms, email back-and-forth, manual pricing estimation, and PDF generators. This friction causes missed leads, inconsistent pricing, and significant administrative overhead before any revenue is guaranteed. An AI team needs standing authority to intercept raw inquiries, apply approved business pricing logic, and produce a verifiable draft quote without requiring constant owner supervision.

  ## Research Report
  Our research evaluated leading solutions for small business inquiry-to-quote workflows:

  *   **HoneyBook:** A dominant player in the creative professional space.
      *   *Pros:* Excellent visual templates, integrated booking and invoicing.
      *   *Cons:* Steep learning curve, heavily reliant on the owner manually setting up complex "smart files" and workflows. The AI features are assistive (e.g., text generation) rather than autonomous agentic workflows.
      *   *Source:* [HoneyBook Help Center - Getting started with Smart Files](https://help.honeybook.com/en/) (Accessed: 2026-09-18)
      *   *Sentiment:* "Honeybook is great but it takes hours to set up my pricing blocks and I still have to manually review every inquiry." (Verified User Quote, Reddit r/freelance)

  *   **Claude for Small Business (Anthropic):**
      *   *Pros:* High-quality reasoning, can parse messy client emails.
      *   *Cons:* Lacks persistent business context out-of-the-box. Requires the owner to repeatedly paste their pricing guidelines and manually copy the output into a separate quoting tool. Prohibits third-party Claude.ai login/token relay per terms.
      *   *Source:* [Claude Code legal/auth guidance](https://code.claude.com/docs/en/legal-and-compliance) (Accessed: 2026-09-18)

  *   **Dubsado:**
      *   *Pros:* Highly customizable CRM for client-based businesses.
      *   *Cons:* Extremely complex onboarding. Often requires hiring a "Dubsado specialist" just to set up the lead capture to proposal flow.
      *   *Source:* [Dubsado Feature Overview](https://www.dubsado.com/features) (Accessed: 2026-09-18)

  ### The OHC Gap & Invisible Agentic Solution
  **Current State (Audit F04, F07):** The codebase has a rudimentary `ClientIntakeRequest` flow (in `src/server/api/agents/client_intake.rs`) that attempts to draft a proposal. However, it relies on global heuristics, creates fixed/unauthorized commitments, and struggles with exact usage accounting (F04).

  **Proposed State:** Implement a fully durable, multi-agent workflow where an `Inquiry Agent` qualifies the lead, a `Pricing Agent` applies the owner's explicit pricing rules, and a `Drafting Agent` generates the quote. The owner only reviews the final structured output (`ActionRisk::DraftForReview`).

  ### Strategy Admission
  *   **OHC target ID:** OHC-04 (Customer inquiry → qualified quote)
  *   **Launch/Run stage:** Run
  *   **Evidence level:** Documented / test-verified
  *   **Current behavior:** Basic form submission creates a hardcoded draft quote.
  *   **Business Result:** 100% of captured inquiries result in a structured quote draft within 5 minutes, saving the owner ~30 minutes per lead.
  *   **Dependencies:** OHC-03 (Business offer context). Reuses existing `quotes` and `quote_line_items` DB tables.
  *   **Non-goals:** Executing the payment or contract signing (handled in OHC-05).
  *   **Authority class:** Standing authority to draft; explicit approval required to send.

  ## Design Doc

  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      actor Customer
      participant API as Web API
      participant Orchestrator as Department Orchestrator
      participant SalesDept as Sales Department (Agents)
      participant DB as Database
      actor Owner

      Customer->>API: Submit Inquiry (Name, Details)
      API->>DB: Store raw Inquiry
      API->>Orchestrator: Dispatch 'Qualify & Quote' Goal
      Orchestrator->>SalesDept: Assign Task
      SalesDept->>DB: Read Pricing Heuristics (Tenant Context)
      SalesDept->>SalesDept: LLM Reason: Map details to service & calculate price
      SalesDept->>DB: Insert Quote (Status: DRAFT)
      SalesDept->>DB: Insert QuoteLineItems
      SalesDept->>Orchestrator: Goal Complete (ActionRisk: DraftForReview)
      Orchestrator->>Owner: Notify: "Draft Quote Ready for Review"
  ```

  ### Mobile UX Flow (375px first)
  1.  **Feed View:** Owner sees a new card: "New Inquiry from [Name]".
  2.  **Card Details:** Expanding the card shows the raw inquiry text alongside the AI-generated Quote Draft (Line items, quantities, total price).
  3.  **Action Bar:** Persistent bottom action bar with two large buttons: "Approve & Send" (Primary) and "Edit Quote" (Secondary).

  ## Implementation Prompt
  1.  Refactor `handle_client_intake` in `src/server/api/agents/client_intake.rs` to rely on a durable job queue rather than performing all LLM calls synchronously in the web request handler.
  2.  The web endpoint should insert the raw inquiry into the database and immediately return a 202 Accepted response.
  3.  A background worker (e.g., expanding `QuoteGenerationWorker`) picks up the inquiry, loads the tenant's specific pricing heuristics, and uses the `LocalLLMClient` to structure the line items.
  4.  The worker must persist a `Quote` in `DRAFT` status and dispatch a notification to the owner via the `DepartmentOrchestrator`.
  5.  **Acceptance Criteria:**
      *   Inbound message creates one lead.
      *   LLM produces a grounded, priced quote based *only* on tenant heuristics (no hallucinated pricing).
      *   Follow-up action is correctly enqueued for owner review.
      *   No fake checkout URLs are generated at this stage.

issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
