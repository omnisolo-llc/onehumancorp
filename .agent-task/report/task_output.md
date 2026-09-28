issue_title: "Blocked finding: Target F13"
issue_description: "Target F13 concerns distinguishing API key, consumer plan and native-client subscription, where provider-specific modes need to fail-closed on unsupported combinations and no session-token relay.

I investigated this issue. E2E tests (\`make test-backend\`) repeatedly timeout after taking longer than the available session limit (e.g. 400 seconds limit exceeded), which prevents verification. In addition, the target 'F13' is marked 'Open' in `docs/research/native_migration_and_remediation.md` but due to testing limitations, making codebase changes that cannot be verified is unsafe. This justifies a blocked no-work finding."
issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: []
