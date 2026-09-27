issue_title: "🤖 Implementer: [blocked no-work finding: F12]"
issue_description: "Title: Resolve Unknown Provider Writes

Problem Statement: Tests fail when retrying provider actions that have unknown outcomes. F12 states 'Simulation, unknown provider outcome and approval paths can look like completion.' We need truthful states/receipts and exact authority, stale approval/revocation, and reconciliation checks on affected paths.

Research Report: The task requires exact authority, stale approval/revocation, and reconciliation checks on affected paths to address F12. During exploration, we found that \`make test-backend\` timed out after 400 seconds, and \`cargo check --locked --workspace --exclude app --all-targets\` also timed out after 400 seconds. \`make lint-node\` failed with 35 problems (35 errors, 0 warnings) in the \`.scratch/superpowers\` directory, but this is a scratch directory. However, the timeout of the core Rust test and compile suites prevents verifying the complex backend state and receipt logic required for F12. The remediation of F12 requires verifying the core rust code base which is not possible due to the timeout of the test suite.

Design Doc: N/A

Implementation Prompt: Resolve the unknown provider writes issue.

Priority: P0

Estimated Scope: Medium"
issue_priority: "P0"
issue_category: "Reliability"
issue_type: "blocked"
issue_label: "ohc:journey:J1"
assignees: []
