issue_title: "✍️ Scribe: [blocked no-work finding: F11]"
issue_description: "Title: Investigate and remediate F11 (smoke tests labeled full journey)

Problem Statement: Finding F11 in `docs/research/native_migration_and_remediation.md` notes that the named full-journey tests (e.g., `src/e2e/full_journey_e2e.spec.ts`) only delegate to a smoke helper (`currentAppSmoke`). The remediation ledger states that 'These are DOM/unit checks, not packaged desktop installation evidence' and '1,688 discovered tests are not 1,688 passed tests. Several older journeys reference obsolete static prototypes and require real behavior repair rather than removal.' The remaining gap is to add actual mutation/state/provider-boundary acceptance tests without live credentials and that it is blocked. We attempted to run `make test-e2e` but it timed out after over 6 minutes, indicating that the e2e testing environment is not functioning correctly or is extremely slow, preventing verification of any new E2E tests.

Research Report:
- The remediation ledger shows F11 is currently 'Blocked'.
- We verified the current implementation of `src/server/api/agents/client_intake.rs` (part of F07) and found that it does have a `format!` string issue that triggered a clippy warning (`clippy::collapsible-if`). This was fixed.
- The `make test-node` suite failed because of missing `lucide-react` initially, and even after installation, it failed with React state updates not wrapped in `act(...)` and Python `ModuleNotFoundError: No module named 'yaml'`.
- Running `make test-e2e` timed out after 401 seconds.
- Without a passing E2E test suite or the ability to run it within a reasonable time, we cannot add new E2E tests for the full journey and verify them as required by F11.

Design Doc:
- Not applicable for a blocked finding.

Implementation Prompt:
- Not applicable.

Priority: P1
Estimated Scope: Medium"
issue_priority: "P1"
issue_category: "documentation"
issue_type: "blocked"
issue_label: "ohc:journey:J1"
assignees: []