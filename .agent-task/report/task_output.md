issue_title: "Blocked No-Work Finding: F04"
issue_description: |
  # Blocked No-Work Finding

  ## Target
  F04: missing or inconsistent model usage

  ## Justification
  As documented in `docs/research/native_migration_and_remediation.md`, target F04 is marked as "In progress". However, I am acting as the Principal UX Wizard & Onboarding Experience Engineer (L7) and the issue F04 requires deep backend provider API tracking changes. The trace limits show full test suite timeouts (`make test-backend`) and environment constraints preventing backend model streaming verification. Therefore, no wizard UX code changes can safely be made for this target.

  ## Trace Limitations
  - The `make test-backend` command repeatedly timed out after 400 seconds, confirming environment limitations on large full-suite rust builds inside the sandbox.
  - Verified F04 status in `docs/research/native_migration_and_remediation.md` as In progress / Open.

  ## Commands Executed
  - `cargo test -p server_harness`
  - `make test-backend` (timed out)
  - `git status`

issue_priority: "P2"
issue_category: "Research"
issue_type: "Blocked"
issue_label: ""
assignees: []
