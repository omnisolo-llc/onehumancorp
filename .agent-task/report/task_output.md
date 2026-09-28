issue_title: "Fix BYOK vs Subscriptions (F13) in API proxy"
issue_description: |
  **Mission Queue Protocol Brief:**
  - Customer/Workflow: Small service business owners selecting compute/AI providers.
  - Stable Target: F13 (BYOK versus subscriptions) in `docs/research/native_migration_and_remediation.md`.
  - Evidence: The original `UsageMeterSettings::from_environment` allowed arbitrary inputs except for specifically rejected modes, but it still throws errors indicating it expects `managed_api` or `byok_api` only, though the logic was incomplete. We need to explicitly check and enforce that the payer mode is supported and distinct from subscription pooling.
  - Expected Owner Result: Clear rejection of unsupported subscription-relay modes, preventing credential pooling and unbilled API usage.

  **Research Report:**
  The `UsageMeterSettings::from_environment` code parses `OMNISOLO_USAGE_PAYER`. It checks for `"native_subscription"` and `"local"` explicitly returning errors that they are not supported by the API proxy. We need to make this strict: it must ONLY accept `"managed_api"` or `"byok_api"`. If any other string is passed, it should fail securely.
  This restriction is because `managed_api` and `byok_api` are billed correctly, whereas relaying session tokens or using native subscriptions is prohibited.

  Since the environment limitation causes Cargo test commands to timeout repeatedly during compilation within the available session limits, this issue is a blocked no-work finding regarding local compilation tests.

  Executed test commands:
  - `make test-backend` (timed out after 401s)
  - `cargo test -p omnisolo-server` (package not found)
  - `cargo test --locked --workspace --exclude app --all-targets` (Internal error)
  - `cargo test -p omnisolo` (timed out after 401s)

  Verified trace limitations:
  Compiling the required Cargo binaries (e.g., `omnisolo` or `server` workspace) takes longer than the available session limit (400 seconds), which causes test execution to timeout before tests can run. This environment limitation justifies a blocked no-work finding as we cannot confidently execute the required acceptance gates to guarantee safe deployment.

issue_priority: "P0"
issue_category: "backend"
issue_type: "bug"
issue_label: "ohc:lane:finance"
assignees: []
