issue_title: "🔎 Oracle: [F04 Model usage tracking and attribution]"
issue_description: |
  # Title
  F04: Model paths disagree on usage

  # Problem Statement
  Different model paths handle usage tracking inconsistently. The proposal adapter and some local-model paths return `Usage::default()`, effectively inventing "free" usage, while the proxy forwards streams without settlement. From a non-technical owner/operator's perspective, this means they cannot accurately see how much AI usage they are paying for, and the business could leak API costs without proper attribution or billing limits.

  # Research Report
  - We found `Usage::default()` being returned in `src/server/api/agents/client_intake.rs`, `src/server/api/perplexity.rs`, and multiple places in `src/server/harness/middleware/inference.rs`.
  - The `proposals.rs` LLM adapter previously returned default usage; although moved or restructured, the intake flow still bypasses accurate metering.
  - Proxy pathways forward streams without meter settlement, preventing accurate capacity reservation and billing reconciliation.
  - The lack of consistent usage tracking prevents the enforcement of hard spend limits and makes it impossible to reconcile OHC-funded model usage against provider invoices.
  - Sources: `docs/research/business_capability_and_usage_economics_audit.md`, codebase search (`grep -rn "Usage::default()" src/server/`).

  # Design Doc
  - **Architecture diagram:**
    ```mermaid
    flowchart LR
        Client[Client Request] --> Proxy[API Gateway / Proxy]
        Proxy --> Inference[Inference Middleware]
        Inference --> LLM[Provider LLMs / APIs]
        Inference --> Metering[Usage Metering & Settlement]
        Metering --> Billing[Tenant Billing Ledger]
        LLM -.->|Usage Metadata| Metering
    ```
  - **Entity Types & Key Relationships:**
    - `InferenceRequest`: tied to a `Tenant` and `Model`.
    - `UsageRecord`: contains token counts, compute time, and request ID.
  - **Integration Points:**
    - Model response payloads must be parsed for token usage (e.g., OpenAI/Anthropic usage blocks).
    - Stream responses must aggregate chunk usage or rely on trailing metadata for accurate totals.
  - **Mobile UX Flow (375px first):**
    - The owner views their usage on a "Cost & Capacity" mobile screen.
    - They see itemized usage for proposals, chats, and automated tasks.

  # Implementation Prompt
  - Describe the user-facing outcome: The owner can view an accurate, unified ledger of AI usage that precisely matches provider activity.
  - Critical User Journey: Owner checks their dashboard, sees detailed API spend for the month, and sets a hard cap on usage. The system stops processing requests when the cap is hit.
  - Acceptance criteria:
    - No `Usage::default()` returns in inference paths unless explicitly zero-cost by design.
    - Stream proxy endpoints accurately aggregate and report token usage upon completion.
    - All token usage is attributed to a specific tenant/request ID.

  # Priority
  P1

  # Estimated Scope
  Medium

  # Strategy Admission
  - Target ID: F04
  - Launch/Run stage: Run
  - Observed/Inferred gap: Observed missing metering implementation.
  - Evidence level: Codebase inspection (verified `Usage::default()` usage).
  - Baseline and measurable result with denominator: 100% of LLM calls have associated non-default usage metadata.
  - Dependencies/reuse: `harness/middleware/inference.rs` and billing services.
  - Non-goals: Creating a new ledger system from scratch; we will reuse the existing billing primitives.
  - Authority class: Internal system infrastructure.
  - Cost/measurement plan: Cost tracking accuracy will be verified against provider invoices in testing.
  - Happy-path and failure acceptance checks:
    - Happy: Tokens are accurately parsed and billed to tenant.
    - Failure: If usage data is missing, the request should either fallback to an estimate or log a critical unbilled-usage error instead of silently returning 0 tokens.

  # Final evidence
  - Loaded Superpowers skills/revision: using-superpowers (8ca22dba9a94f28898bbce59f2537ff4d87c747d)
  - Checks executed: git status, grep -rn "Usage::default()" src/server/
  - Outcomes: Identified 5 places where Usage::default() is used. git status showed a clean working tree.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []