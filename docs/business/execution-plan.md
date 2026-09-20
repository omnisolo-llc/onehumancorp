# Execution Model

This page now describes how execution is tracked instead of serving as an in-repo task board.

## Policy

- GitHub issues are the source of truth for active work.
- `docs/business/execution-plan.md` is a process document, not a live backlog.
- Root `RESEARCH.md` is the canonical product-priority and validation document.
- Historical local task files have been removed from the repository.

## Documentation Gate

Before opening implementation work, attach or reference these documents when they exist:

- Design doc
- CUJ
- Test plan

## Current Epic Buckets

- Selected client-to-cash journey inventory and paid-pilot validation.
- Owner-approved business profile, offer, tool connections and delegated authority.
- Inquiry, proposal, booking/deposit, digital delivery/review, final payment and follow-up.
- Durable execution, tenant/client isolation, evidence, exception handling and hard budget caps.
- Activation, retained use, net time saved, serving cost and honest release evidence.

## Issue admission gate

Each issue must name the initial service-business customer, workflow stage, stable OHC target ID where applicable, evidence/reproduced defect, measurable result, dependencies/reuse, non-goals, authority class, cost plan, and happy/failure-path acceptance checks. Search existing issues before creating another. Re-triage older broad-suite work against `RESEARCH.md` rather than silently implementing it.

P0 is reserved for actual security/data/money incidents or release-blocking correctness defects. Initial-workflow features are normally P1. New verticals, payroll/tax engines, manufacturing, POS and random harness expansion remain gated. A no-change or blocked finding is valid, not permission to manufacture a feature.

Report the evidence level: documented, implemented, test-verified, provider-sandbox-verified or pilot-verified. Passing unit tests are not provider or customer evidence.

## Active Backlog

GitHub issues are the canonical source for the active backlog. See the roadmap in `docs/business/roadmap.md` for strategic direction.
