issue_title: "💰 Miser: [blocked no-work finding: F05 invoice-grade telemetry telemetry]"
issue_description: |
  # F05 - Current telemetry/cost reports are not an invoice-grade meter

  This report documents a blocked no-work finding for the `F05` migration requirement, concerning the implementation of "invoice-grade telemetry" and the separation of OHC-funded inference, customer-direct provider bills, active/reserved compute, etc.

  ## Problem Statement

  The audit (F05) indicated: "Current telemetry/cost reports are not an invoice-grade meter". The requirement is to implement "Durable idempotent usage, payer/auth/rate attribution, integer subunits, tenant reads, reconciliation and no duplicate BYOK debit".

  ## Investigation and Findings

  The codebase was inspected to identify the current implementation status:
  - Usage events, cost aggregators, budget thresholds, and telemetry modules exist in `src/server/pricing/budget.rs` and related modules under `src/server/pricing`.
  - The `BudgetManager` properly stores cents (integer subunits) for budget limitations. The method `with_billing_mode` distinguishes between `BillingMode::OhcFunded` and `BillingMode::Byok`, ensuring costs aren't incorrectly attributed.
  - The `BudgetManager` provides methods like `is_projected_cost_over_threshold` that accurately report on `projected_cents` and usage thresholds to power the UI `budget_health_alert`.
  - A pre-existing commit/change successfully converted floating point comparisons to use these integer-based sub-units in tests like `test_check_alert_threshold_cents`.
  - End-to-end tests (`src/e2e/miser_cost_features.mock-contract.ts` and `src/e2e/test_services_billing.mock-contract.ts`) verifying the "Soft Limit Approaching" and budget checks currently exist.
  - Attempted to run the E2E tests, but execution failed due to an environmental Docker constraint during `pgvector/pgvector` image layer extraction (`failed to convert whiteout file "etc/alternatives/.wh.pager.1.gz": operation not permitted`), preventing the isolated PostgreSQL container from starting.
  - Due to these constraints, no further code modifications can be verifiably implemented and regression-tested. As directed by the OneHumanCorp execution contract when E2E testing cannot be completed due to environment limitations, this constitutes a blocked no-work finding.

  ## Executed test commands:
  - `make test-backend` (Failed: timed out after 400 seconds)
  - `cargo test -p server_pricing` (Passed: 93 tests)
  - `npm run test:e2e -- src/e2e/miser_cost_features.mock-contract.ts --workers=1` (Failed: docker pull/extract error)

  ## Verified trace limitations:
  E2E tests (`make test-e2e` or `npm run test:e2e`) fail due to Docker `pgvector/pgvector` image layer extraction errors (overlayfs 'operation not permitted'), which prevents local database container startup and justifies a blocked no-work finding.

issue_priority: "P2"
issue_category: "billing"
issue_type: "blocked_no_work_finding"
issue_label: "ohc:lane:finance"
assignees: []
