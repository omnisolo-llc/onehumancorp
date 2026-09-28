issue_title: "⚡ Bolt: [blocked no-work finding: F13]"
issue_description: |
  **Target ID**: F13: API key, consumer plan and native-client subscription are distinct.

  **Issue**: Provider-permitted native-client subscription hosting is still a separate integration/terms/quotas decision, not generally implemented.

  **Verified trace limitations:**
  - This is a research and policy-level task regarding provider subscription brokering and consumer credential use (OpenAI/Anthropic/Google).
  - The `RESEARCH.md` states: "Evaluate managed API, customer API-key/cloud-account billing, provider-permitted native-client subscription access and local inference separately. A ChatGPT, Claude or Gemini subscription is not a general-purpose API key. Verify current official authentication, hosting and quota terms for the exact integration; distinguish permitted unmodified native-client hosting from forbidden session-token relay. Never collect consumer session tokens or pool subscriptions."
  - Implementing support for these different models requires an explicit decision on integration/terms/quotas, not a backend code optimization.
  - Compiling the workspace (`make test-backend`) failed repeatedly due to timing out after 400 seconds, an environment limitation preventing full test verification.
  - No new code was written since the finding is a blocked research/policy decision outside of codebase changes.

  **Executed test commands:**
  - `make test-backend` (Timed out)
  - `cargo check --locked --workspace --exclude app --all-targets` (Timed out)

  **Decision evidence:** Blocked.
issue_priority: high
issue_category: research
issue_type: bug
issue_label: agent-report
assignees: []
