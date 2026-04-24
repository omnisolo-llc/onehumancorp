# Goal
Treat already implemented "Cost Engineer / Miser" features as maintenance and perfection task:
1. LLM Token Efficiency
2. Storage Compression & CDN
3. AI Agent Rate Limiting
4. Infrastructure Cost Metering
5. Transaction Fee Optimization
6. Pricing Page & Billing Portal
7. Cost Transparency Dashboard

I need to proactively identify and fix bugs, refactor raw SQL, clean up dead code, and improve test coverage for these areas.
A specific test file `src/tests/e2e/budget_test.go` contains many empty E2E tests for billing and budget features that need to be implemented using Playwright testing logic matching the platform patterns.

# Plan
1. **Investigate the CI Failure**: The PR comment indicates that CI checks failed. I need to run `bazelisk test //...` to identify the failure and fix it.
2. **Refactor and clean up `budget_test.go`**: I will fill out the E2E tests in `src/tests/e2e/budget_test.go` and `src/tests/e2e/cuj_budget_extra_test.go` using Playwright, adding expectations about finding elements for Budget settings, daily limits, cost breakdown, forecasting, and notifications.
3. **Review other areas for refactoring:** I will look at `src/server/billing/tracker.go` and `src/server/lib/pricing/pricing.go` for any raw SQL or redundant code. `src/server/agents/local/cached_llm.go` has raw SQL for DB caching which is good but could be optimized or cleaned up.
4. **Pre-commit checks**: After modifying the tests, I will ensure they compile and pass. I will run `pre_commit_instructions` and follow them to finish the workflow.
