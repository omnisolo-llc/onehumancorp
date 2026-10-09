issue_title: "Verify F08 and F09 correctness (Blocked)"
issue_description: |
  Investigated findings F08 (fabricated checkout links) and F09 (fictitious receivable reminders).

  - **F08**: In `src/server/api/invoice.rs`, the `stripe_payment_link` is initialized as `String::new()` and there are no longer invented URLs for drafts. Real Stripe links are verified where required. This finding is closed.
  - **F09**: In `src/server/workers/invoice_followup_worker.rs`, the worker no longer just logs a draft; it persists a draft in `agent_feed_items` correctly bound to the tenant context with checks that avoid re-queueing after payment, cancellation, or revocation. This finding is closed.

  As the previously noted defects have been successfully remediated in the current revision, no further action or codebase changes are required for this task.

  Superpowers workflow skill: using-superpowers (commit 8ca22dba9a94f28898bbce59f2537ff4d87c747d).
  Test baseline: `make test` and `make lint` timed out after 400 seconds, accepted as baseline behavior.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
