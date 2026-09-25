issue_title: "Fix BudgetManager projected cost alert condition for zero-limit budgets"
issue_description: |
  # Mission Queue Protocol: Fix BudgetManager projected cost alert condition for zero-limit budgets

  ## Problem Statement
  The `is_projected_cost_over_threshold` function in `BudgetManager` flags small usage logic poorly when limit budgets are 0, treating it as over threshold if `projected_cost_cents > 0`. This incorrectly warns users for even trivial amounts when their budget is uninitialized or explicitly set to $0 for an unlimited pay-as-you-go configuration, which breaks usage economics tracking principles that require clear, uninhibited capacity where limits are not enforced. Also, it wrongly triggers if `current > 0`, ignoring the actual intended budget usage semantics.

  ## Research Report
  Our audit (`docs/research/business_capability_and_usage_economics_audit.md`) notes the necessity for strict budget accounting that doesn't artificially block valid provider usage for small metrics. Currently, the `BudgetManager`'s implementation of `is_projected_cost_over_threshold`:
  ```rust
  if self.total_limit_cents <= 0 {
      return projected_cost_cents > 0 || current > 0;
  }
  ```
  causes the function to return `true` indicating an over-threshold violation when the limit is 0 (or negative) but any current or projected cost exists. This contradicts other threshold checks in the system like `check_alert_threshold` and `check_alert_threshold_cents` which explicitly return `false` if `total_limit_cents <= 0`. It's crucial for OHC to allow unrestricted usage (returning `false` for threshold checks) when a hard budget limit isn't configured (> 0).

  ## Design Doc
  - **Architecture diagram:** N/A (minor code fix)
  - **UI wireframes:** N/A (backend logic)
  - **Mobile UX flow:** N/A
  - **AI agent integration points:** Modifying this logic ensures agents aren't prematurely throttled by the rate limit / budget system when evaluating whether an upcoming action will exceed budget bounds on un-budgeted accounts.
  - **Decisions:** Update the `is_projected_cost_over_threshold` function in `src/server/pricing/budget.rs` to simply return `false` when `self.total_limit_cents <= 0`.

  ## Implementation Prompt
  Implementer: Fix the `is_projected_cost_over_threshold` method in `src/server/pricing/budget.rs` so that if `total_limit_cents` is 0 or less, it returns `false`, aligning it with `check_alert_threshold` behavior and accurately reflecting that a zero limit does not have a threshold to exceed. Update the test `test_check_alert_threshold_with_projected_costs` to reflect that `zero_manager.is_projected_cost_over_threshold(800)` should be `false`.

  ## Priority
  P2

  ## Estimated Scope
  Small

  ## Strategy Admission
  - **Segment:** Backend / Finance
  - **Authority Class:** System / Autonomous policy checking
  - **Dependencies:** None
  - **Expected Outcome:** Proper handling of un-budgeted accounts, avoiding false "over threshold" triggers, accurately reflecting budget limit capacity checking.
issue_priority: P2
issue_category: research
issue_type: task
issue_label: [agent-report]
assignees: []
