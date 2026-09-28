issue_title: "Universal Provider & Local Usage Accounting (F04)"
issue_description: |
  ## Title
  Universal Provider & Local Usage Accounting (F04)

  ## Problem Statement
  Currently, the system experiences inconsistent and unmetered model usage tracking across the platform. While basic metered routes properly preserve native quantities and local LLM bounds, the proposal adapter returns hardcoded dummy metrics (e.g. 0 or 10 tokens), and certain proxy boundaries stream responses without intercepting, counting, or settling token usage. From an owner-operator perspective, this means we cannot trust our telemetry to bill tenants accurately, we might leak inference capacity without accounting for it, and BYOK (Bring Your Own Key) vs platform-provided subscription distinctions could fall out of sync with actual costs. If we cannot measure exactly who spent what—including tool and embedding paths—we cannot enforce budgets, which risks unbounded provider exposure.

  ## Research Report
  - **Context:** The `native_migration_and_remediation.md` ledger identifies F04 as an "Open" target, requiring complete inventory and reconciliation of actual provider counts.
  - **Baseline:** Found instances in `api/proposals.rs` where mock or hardcoded token usages are returned. Local proxy streaming and other model inference boundaries still lack a unified interception point for final settling of usage, allowing unmeasured streams.
  - **Competitive Analysis:** Competitors like OpenAI, Anthropic, and SaaS solutions (Shopify, Wix) employ strict idempotency and usage gateways before fulfilling AI generations to ensure accurate billing margins. Without similar boundaries, our platform could absorb arbitrary model costs.

  ## Design Doc
  ### Architecture Diagram
  ```mermaid
  sequenceDiagram
      participant App as Mobile/Web App
      participant Route as API Gateway & Proxy
      participant Meter as Usage Meter
      participant Ledger as Cost Auditor & Ledger
      participant Provider as LLM Provider

      App->>Route: Request Generation (Tenant ID, Context)
      Route->>Meter: Check Tenant Budget & Reserve Capacity
      Meter-->>Route: Authorized
      Route->>Provider: Send Prompt (Include callback/interceptor)
      Provider-->>Route: Stream Response + Usage Stats
      Route->>Ledger: Commit Actual Usage (Idempotent ID, Model, Tokens)
      Ledger-->>Meter: Release/Adjust Reservation
      Route-->>App: Completed Response
  ```

  ### UI Wireframes / Screen Flow Description (375px)
  - **Owner Usage Dashboard:** A clean, translucent macOS-style card UI displaying "Current Month AI Usage" vs "Available Limit".
  - **Over-limit State:** If a request is blocked, show a friendly inline notification to the operator: "Your business AI budget needs a bump to process this request. [Adjust Settings]".

  ### Mobile UX Flow
  - Maya (the baker) generating 10 proposals in an hour shouldn't hit surprise bills. She taps a "Generate Proposal" button, which transparently checks her usage budget. If successful, she views the result. If near limit, she gets a push notification allowing a one-tap upgrade via standard dashboard settings.

  ### AI Agent Integration Points
  - **Tool & Coordinator Agents:** Any autonomous background agent request must bundle its `task_id`, `agent_id`, and `tenant_id` into the model invocation, ensuring we can attribute every sub-step and retrieval (RAG) query to a distinct workflow.
  - **Proposal Adapter:** Must intercept actual usage from the AI generation instead of defaulting to mocked values.

  ## Implementation Prompt
  **To the Engineering Swarm (Implementer):**
  Implement strict model usage tracking and reconciliation for all external AI generation paths (F04).
  1. Remove hardcoded dummy token values (e.g., in the proposal generator `api/proposals.rs`) and ensure actual usage returned from the Local/Native LLM client is correctly captured.
  2. Implement an interception layer in proxy streaming boundaries to count incoming tokens if the provider doesn't emit explicit usage headers, or strictly parse usage headers if they do.
  3. Plumb `tenant_id`, `task_id`, and `model_name` through to the usage ledger for every generation, tool use, and embedding.
  4. The outcome must guarantee that we never invent "free usage" and always capture an accurate reflection of the provider counts. Provide regression tests to prove idempotency and exact counting. Do not prescribe specific database schemas—extend the existing event pipeline.

  ## Priority
  P0

  ## Estimated Scope
  Medium

  ## Strategy Admission
  - **Target ID:** OHC-F04
  - **Customer/Stage:** All personas / Launch stage (critical for billing).
  - **Evidence Level:** Codebase static analysis and remediation ledger (F04 is listed as Open).
  - **Baseline/Metric:** Currently 0% confidence in tool/embedding/proxy token accuracy. Target: 100% requests accounted through `UsageMeter` before forwarding to `CostAuditor`.
  - **Dependencies/Reuse:** Reuse existing `usage_ledger` and `usage_meter` middleware.
  - **Non-Goals:** Building a new customer invoice pipeline (that is F05). We are only fixing usage tracking precision.
  - **Authority Class:** System-level telemetry.
  - **Cost Plan:** Minimal computational overhead to capture metrics.
  - **Acceptance Checks:** (Happy) A request passes through the proxy, returns text, and exact usage is recorded. (Failure) A provider omits usage, we fail closed or fallback to a deterministic tokenizer count without inventing numbers.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
