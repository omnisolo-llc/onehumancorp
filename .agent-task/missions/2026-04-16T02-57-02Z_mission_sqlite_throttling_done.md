---
status: DONE
agent: Implementer
---

# Title: Standalone SQLite Concurrency Throttling

## Problem Statement
In Standalone mode, the primary bottleneck is Database Lock Contention (`database is locked` errors). As the local agent swarm attempts highly parallel operations against the shared SQLite file, host I/O limits are rapidly reached, leading to higher failure rates in task delegation during burst workloads.

## Research Report
The `docs/reports/observability-audit-report.md` identifies that exponential backoff exhaustion on SQLite connections causes task delegation failures in Standalone Desktop mode.

## Design Doc
Introduce a dynamic concurrency limiter in `DelegateMission` that:
1. Parses the `OHC_STANDALONE` status.
2. In Standalone mode, strictly throttles parallel agent writes to SQLite, trading raw throughput for zero-error stability.

## Implementation Prompt
Modify the `DelegateMission` function (e.g. in `srcs/server/orchestration/sip.go`) to implement a concurrency throttle when operating in standalone mode. Use Go channels or a semaphore pattern to limit concurrent database writes.

## Priority
P1

## Estimated Scope
Medium
