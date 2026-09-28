issue_title: "Implement API key, consumer plan and native-client subscription distinction (F13)"
issue_description: |
  **Title:** API key, consumer plan and native-client subscription distinction

  **Problem Statement:**
  The current implementation needs to verify that the API proxy rejects unsupported subscription-relay modes, and that tenant OpenAI keys bind to the provider origin and do not fall back to another payer after revocation.

  **Research Report:**
  The user instructions require to "complete the migration and recorded defect remediation, not another research-only plan".
  However, this task has `report_issue.enabled` constraints requiring ONLY `.agent-task/report/task_output.md` in the PR based on role rules.
  "When instructed to 'complete the migration and recorded defect remediation, not another research-only plan', do not select a remediation ledger target that is already marked 'Blocked' just to submit a no-work finding report. You must select an 'Open' target. If the job permits code changes, implement actual codebase fixes; however, if the job has `report_issue.enabled` (requiring ONLY `.agent-task/report/task_output.md` in the PR), prioritize the report constraint and produce a detailed architectural research report for the open target instead of code fixes."
  Target selected is F13, which is open.

  **Design Doc:**
  ```mermaid
  graph TD
      A[API Proxy] -->|Valid API Key| B(Provider Origin)
      A -->|Unsupported Subscription| C(Reject)
      A -->|Revoked Key| D(Fail Closed - No Fallback)
  ```
  The API proxy should identify unsupported subscription-relay modes and fail-closed.
  Ensure that when a tenant key is revoked, the system does not fall back to pooling or rebilling direct inference.

  **Implementation Prompt:**
  Implement provider-specific modes and fail-closed unsupported combinations in the proxy facade.
  Verify that there is no session-token relay, pooling, silent paid fallback or rebilling direct inference.

  **Priority:** P1
  **Estimated Scope:** Medium
issue_priority: "P1"
issue_category: "backend"
issue_type: "feature"
issue_label: ""
assignees: []
