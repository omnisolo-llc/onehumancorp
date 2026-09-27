issue_title: "💰 Miser: [blocked no-work finding: F13]"
issue_description: |
  Title: [blocked no-work finding: F13] BYOK versus subscriptions

  Problem Statement:
  The user has asked us to evaluate charging for compute and AI API usage, plus customer BYOK or provider-permitted native subscription access, and to understand existing code and real owner needs before committing to another plan. Based on `docs/research/native_migration_and_remediation.md`, finding F13 (BYOK versus subscriptions) is currently in an "Open" state. However, the exact implementation requirements for provider-permitted native-client subscription hosting are still a separate integration/terms/quotas decision, which is not generally implemented and lacks the necessary owner and economic parameters to proceed with a concrete implementation.

  Research Report:
  The evidence in `docs/research/native_migration_and_remediation.md` shows:
  F13 | API key, consumer plan and native-client subscription are distinct | Provider-specific modes and fail-closed unsupported combinations; no session-token relay, pooling, silent paid fallback or rebilling direct inference | Open

  And in the Findings-to-implementation status:
  F13: BYOK versus subscriptions | API proxy rejects unsupported subscription-relay modes; verified tenant OpenAI keys bind to the provider origin and do not fall back to another payer after revocation. | Provider-permitted native-client subscription hosting is still a separate integration/terms/quotas decision, not generally implemented.

  Because the necessary business decisions regarding integration terms and quotas for native-client subscriptions have not been made, this task is blocked. There is insufficient information to proceed with any code modifications.

  Design Doc: ""
  Implementation Prompt: ""
  Priority: ""
  Estimated Scope: ""

  Skill Provenance and Final Evidence:
  Loaded Superpowers skills: none. Git revision: 3d7f68cb26bb842adc2e647549449ce61f57956a. Tests run: `make lint-node`, `make test-node`, `make test-e2e`. Verified trace limitations: `make lint-node` failed due to ESLint issues (e.g., unused variables) and `make test-e2e` timed out after ~400 seconds. checks, and outcomes as final evidence.
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
