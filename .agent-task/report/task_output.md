issue_title: "🎥 Lens Audit: Verify and Repair Customer Connection UI/API (F06)"
issue_description: |
  # Research Report

  ## OHC Mission & Capability Goals
  Based on the active capability map in `RESEARCH.md`:
  - **Goal:** Establish the operation (Integrations).
  - **Required Features:** Connected accounts, standing authority.
  - **Evidence:** Persistent business context and verified readiness; provider/identity prerequisites shown accurately.

  ## Problem Statement
  Finding F06 notes that the `tool_integrations.rs` connection path correctly returns `501 Not Implemented` and `usable: false` because verified connection with encrypted storage is incomplete. The product directive states this failure mode is "honest" and explicitly forbids replacing it with a fake connected badge. We must preserve this 501 boundary behavior.

  ## Discovered Status
  - F06 was previously marked as "Closed" in the remediation ledger `docs/research/native_migration_and_remediation.md`. I have verified the code in `src/server/api/tool_integrations.rs` properly returns a 501. The task is to "select one existing issue or research uncertainty". I have audited F06.

  ## Blocked Prerequisites
  The UI implementation for integrations does correctly handle the connection workflow by hitting `/api/v1/integrations/[id]/connect` and correctly expects the 501 error gracefully when unsupported. The audit of this workflow reveals that it functions as intended (it "fails honestly") per the owner's directive. No further code repair is possible without violating the directive to not "invent new UI" or "mock data", and a fully functioning vault is not in scope for the Lens phase.

  ## Proposed UX Flow & Architecture Diagram
  ```mermaid
  graph TD
      A[Integration UI] -->|Click Connect| B[Connect API Route]
      B --> C[Rust tool_integrations.rs]
      C --> D{Is Provider Supported?}
      D -- No/Missing Vault --> E[Return 501 Not Implemented]
      E --> F[UI Renders Unavailable Error]
  ```

  ## Superpowers Workflow Provenance
  - **Repository**: https://github.com/obra/superpowers.git
  - **Revision**: latest
  - **Skills Loaded**: `using-superpowers`, `brainstorming`
  - **Checks Performed**: Checked capability goals in `RESEARCH.md`, the open audit findings in `docs/research/native_migration_and_remediation.md`, and inspected the code paths in `src/server/api/tool_integrations.rs` and `src/ui/next/src/app/integrations/`. Verified `make test-node` outcomes to check E2E tests related to integrations.
  - **Outcomes**: Formulated a no-work/blocked report for addressing F06 to document the tested limitation and conform to the Lens directive and no-work report formatting rules.
issue_priority: "P0"
issue_category: "ui"
issue_type: "audit"
issue_label: ["ohc:lane:ui", "agent-report"]
assignees: []
