issue_title: Blocked: Local storage hardening for standalone mode
issue_description: |
  Blocked no-work finding: Local Hardening.
  The Sentinel task asks to "Ensure Standalone Mode uses secure, encrypted SQLite storage and proper file permissions for the local wrapper."
  However, this task has timed out during exploration running `make test-rust` multiple times, and even specific `cargo test` variants hit time limits.
  The execution environment is hitting memory/process limits preventing completion of testing required before making any changes. The environment limitations are verified.

  Final evidence:
  Loaded skills: brainstorming, using-superpowers, systematic-debugging, writing-plans, executing-plans, subagent-driven-development, test-driven-development, requesting-code-review, receiving-code-review, verification-before-completion, using-git-worktrees
  Skill provenance: superpowers revision 8ca22dba9a94f28898bbce59f2537ff4d87c747d
  Checks and outcomes as final evidence:
  `make test-node` passes.
  `make lint-node` passes.
  `cargo build` completes in 5m 21s.
  `make test-rust` times out (400s+).
  `cargo test --all-targets --workspace --exclude app` times out (400s+).
  `cargo check --locked --workspace --exclude app --all-targets` times out (400s+).

issue_priority: ""
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
