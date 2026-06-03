issue_title: "feat: Zero-Touch Subscription Engine scaffold"
issue_description: |
  Consolidated the database schema in `src/server/db/migrations/019_unify_subscriptions.sql` by dropping redundant `fulfillment_batches` and `subscribers` tables, aligning the architecture with the `018_zero_touch_subscriptions.sql` definitions.
  Updated API endpoints within `src/server/api/subscription.rs` and service models in `src/server/services/subscription/service.rs` to target the unified `subscriptions` table.
  Resolved unused variable issues, populated the missing `plan_id` field in subscribers queries, and updated the end-to-end `subscription_box.spec.ts` script logic to correctly reflect the `/subscribe` schema behavior.
issue_priority: P1
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
