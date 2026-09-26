issue_title: "Blocked: Lack of measured representative serving costs or owner outcomes (F14)"
issue_description: |
  ## Mission Queue Protocol

  **Title**: Blocked: Lack of measured representative serving costs or owner outcomes (F14)

  **Problem Statement**: The current source does not supply a measured deployment cost, representative workload distribution, or reconciled provider invoice. Before setting a price card or implementing billing limits, we need measured owner setup/review/correction time, observed success/failure rates, provider requests, active/reserved resources, and actual invoice reconciliation from a non-technical owner/operator's perspective.

  **Research Report**:
  - Reviewed `docs/research/business_capability_and_usage_economics_audit.md` (F14)
  - Reviewed `RESEARCH.md` (OHC-10)
  - Reviewed codebase (`src/server/services/billing`, `src/server/pricing`)
  - Finding: We lack a small, permissioned set of recent owner workflows across candidate segments to establish baseline economics and willingness to pay. No public interviews, quotes, or pricing experiments have been provided for the selected Nora workflow (or others like Maya, Carlos, Priya, Leo, Fatima).
  - Conclusion: Blocked. Without real customer data and actual provider invoices, we cannot safely implement a cost model or charge owners. We must not invent interviews or contact people without authorization.

  **Design Doc**:
  High-level architecture:
  - Entity types: `Customer Workflow`, `OHC Metric Collection`, `Cost Auditor`, `Actual Invoice Reconciliation`.
  - Integration points: Connecting owner metrics directly to billing usage without arbitrary overhead.

  UI wireframes / Mobile UX flow (375px first):
  - A mobile dashboard showing: "Estimated task cost and maximum authorized spend", an itemized OHC bill, and separate customer-direct provider usage.

  AI agent integration points:
  - The Cost Auditor tracks AI usage securely.

  ```mermaid
  graph TD;
      A[Customer Workflow] --> B[OHC Metric Collection];
      B --> C[Cost Auditor];
      C --> D[Actual Invoice Reconciliation];
      D -.-> E[Blocked: Missing Customer Data];
  ```

  **Implementation Prompt**: None. Blocked on business prerequisites. Do not implement any billing features until external evidence is provided.

  **Priority**: P0

  **Estimated Scope**: Large

  **Strategy Admission**:
  - OHC target ID: OHC-10
  - Launch/Run stage: Measurement
  - Observed/inferred gap: Missing data
  - Evidence level: Blocked
  - Baseline and measurable result with denominator: N/A
  - Dependencies/reuse: Needs external interviews
  - Non-goals: Inventing customer data
  - Authority class: Owner consent required
  - Cost/measurement plan: To be determined after data collection
  - Happy-path and failure acceptance checks: N/A

  ## Superpowers Workflow Provenance
  - Loaded Skills: `using-superpowers`
  - Repository URL: https://github.com/obra/superpowers/
  - Revision Hash: 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  - Checks Performed: Reviewed memory, codebase, and docs for evidence of actual interviews or pricing data. Checked current metrics in `billing.rs` and `auditor.rs`.
  - Outcomes: Identified that we lack required external data (interviews/customer workflows/provider invoices). Returning a no-work/blocked outcome.
issue_priority: P0
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
