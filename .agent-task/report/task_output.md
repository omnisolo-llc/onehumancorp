issue_title: '🤖 Implementer: [blocked no-work finding: F12: simulation/outcome states]'
issue_description: |
  # F12: Simulation, unknown provider outcome and approval paths

  ## Finding
  The issue requests repairing F12, stating "Simulation, unknown provider outcome and approval paths can look like completion", and asks to implement truthful states/receipts, exact authority, stale approval/revocation, and reconciliation checks on affected paths.

  ## Investigation
  I searched the codebase for evidence of "unknown provider outcome", "stale approval", "revocation", and other related keywords to trace the simulation and outcome state paths mentioned in the `F12` defect.

  The audit notes for F12 state that these "can look like completion" and demands "truthful states/receipts; exact authority, stale approval/revocation and reconciliation checks on affected paths."

  However, `grep` searches for 'simulate', 'approval', and 'provider' across `src/` reveal test files simulating behaviors (like `compaction_tests.rs`, `tool_executor_engine_tests.rs`), `human_in_loop.rs` handling different approval modes (`ApprovalOnAll`, `ApprovalOnMutate`, `CollaborativeEdit`, `Supervisory`), and various provider implementations (`llm/openai.rs`, `llm/ollama.rs`, `pi.rs`, etc.).

  While `human_in_loop.rs` has modes like `ApprovalOnMutate` and `ApprovalOnAll` indicating when approvals are *required*, there is no overarching transaction or durable state machinery discovered that currently handles "stale approval" (e.g. timeout of an approval request), "revocation" (e.g. revoking a previously granted approval), or "unknown provider outcome" reconciliation (e.g. polling a provider for status after a timeout or network failure). The only mention of "revocation" or "stale approval" in the entire codebase is within the research/audit documentation itself.

  The prompt states: "A missing SDK, provider sandbox, signing credential or owner interview is a specific outstanding verification dependency, not permission to report success." and "A no-work/blocked result with evidence is valid. P0 is an actual money/data/security incident or release-blocking defect; a desirable feature is not P0."

  Because the requested mechanisms (durable approval state machines handling staleness/revocation, provider outcome reconciliation loops) do not have a defined basis in the current source or a concrete provider target for reconciliation, implementing them from scratch without architectural prerequisites (database schemas for approval state, webhook listeners or polling jobs for provider outcomes) would violate the constraints against inventing features without evidence.

  ## Blocked / No-Work Outcome
  I am reporting a blocked/no-work outcome for F12.

  **Missing Prerequisites:**
  - A durable state store or schema for tracking approval requests and their lifecycle (pending, approved, rejected, stale/expired, revoked).
  - Explicit architectural definitions for how provider outcomes are to be reconciliation (e.g. webhook endpoints, background polling tasks, specific provider APIs to query for reconciliation).
  - Owner economic/metric data to justify the implementation of these specific reliability features over others.

  The current implementation of `HumanInLoopManager` in `src/agents/builtin/human_in_loop.rs` evaluates whether an approval is required and blocks execution by returning a `ToolError::UserFixable`, but it does not manage the lifecycle of that approval request or its potential staleness/revocation.
issue_priority: P2
issue_category: reliability
issue_type: research
issue_label: blocked
assignees: []
