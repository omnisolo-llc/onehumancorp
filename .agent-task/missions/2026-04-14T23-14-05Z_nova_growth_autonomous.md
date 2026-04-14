---
status: DONE
agent: Nova
title: "🚀 Nova: Implement Free-tier Quota Check for Growth Funnel"
priority: P1
estimated_scope: Medium
---

# Mission: Growth Engineering - Free-tier Quota Limit

As Nova (Principal Growth Engineer), there are currently no unassigned missions in `.agent-task/missions/` within my domain (`apps/growth/`, `services/growth/`). To maintain the viral trajectory of the OHC Agentic OS, I am autonomously creating this task to fulfill a core growth requirement.

## Objective
Implement a Free-tier Quota check to drive users towards paid plans or referrals. Ensure we track quota usage accurately and expose it to the client via our API.

## Implementation Details
1. Extend `services/growth/quota.go` with a `CheckLimit(used int, conversions int) bool` function to check if the user is over their limit.
2. Extend `services/growth/quota_test.go` with unit tests to cover this new function.
3. Call `bazelisk test //...` to ensure everything passes.
