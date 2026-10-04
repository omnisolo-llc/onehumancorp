outcome: blocked
issue_title: "Implement Distributed Inventory Locks (Redis Redlock) for Multi-Channel Sync"
issue_description: |
  **Priority**: P1

  The task requires implementing Redis Redlock for distributed inventory locks to prevent double-booking across multi-channel synchronization, which depends on a Redis crate or custom Redis integration in the Rust backend.

  However, this feature is currently blocked due to multiple verifications on the codebase demonstrating lack of prerequisites or testing readiness:
  1. No Redis client crate exists in the workspace dependencies (`cargo tree | grep redis` returns no matches in the local workspace). The codebase does not appear to currently depend on Redis for state management.
  2. The issue describes building distributed locks for POS, multi-channel commerce, and Redlock for inventory. Existing commerce functionality in the current audit (`docs/research/business_capability_and_usage_economics_audit.md`) notes that "The codebase is not exclusively a digital-agency app. Reservation, shipping exceptions, stock, payment and owner workflows require live proof. A broad commerce rewrite is not justified by directory names" and identifies these features as requiring proof/research prior to expansion.
  3. The development environment currently suffers from unrelated pre-existing failures (`make test` and `make build-web` fail with Turbopack tailwindcss/postcss configuration errors unrelated to this backend task), preventing the full required CI acceptance gates from passing.

  Per the prompt and `docs/research/native_migration_and_remediation.md`: "New verticals, channels, agent marketplaces, payroll/tax/MRP/POS engines and harness adapters require evidence and the expansion gate in RESEARCH.md." "Implement only the current bounded slice or report a justified no-work/blocked outcome." Given no Redis infrastructure exists and commerce workflows (like POS and inventory levels) require research expansion gates first, this task is blocked.
issue_priority: "P1"
issue_category: ""
issue_type: ""
issue_label: ""
assignees: ""
