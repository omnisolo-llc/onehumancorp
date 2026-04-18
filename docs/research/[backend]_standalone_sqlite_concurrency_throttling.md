# Title: Implement Standalone SQLite Concurrency Throttling

## Problem Statement
A comparative analysis of Prometheus metrics, Kubernetes logs, and local SQLite status files reveals divergent bottleneck profiles based on the deployment mode. For Standalone Desktop Mode (single-user, local Go backend with SQLite `swarm.db`), the primary bottleneck is **Database Lock Contention** (`database is locked` errors). As the local agent swarm attempts highly parallel operations against the shared SQLite file, host I/O limits are rapidly reached, leading to higher failure rates in task delegation during burst workloads due to exponential backoff exhaustion on SQLite connections.

## Research Report
The Standalone Desktop mode operates with a local SQLite database (`swarm.db`). While Cloud-Native mode handles high concurrency via PostgreSQL, SQLite is constrained by local host I/O. When multiple agents perform parallel task delegations, it causes database lock errors. To ensure zero-error stability, we must trade raw throughput for reliability by introducing a dynamic concurrency limiter specifically for Standalone mode.

## Design Doc
1. Modify `DelegateMission` (or the equivalent delegation mechanism in `srcs/server/orchestration/delegation.go` or similar) to parse the `OHC_STANDALONE` status or environment variable `OHC_MULTITENANT` (if `OHC_MULTITENANT != true`, it's Standalone mode).
2. Introduce a dynamic concurrency limiter (e.g., using a buffered channel or a semaphore like `golang.org/x/sync/semaphore`) to strictly throttle parallel agent writes to SQLite when in Standalone mode.
3. Ensure this throttling mechanism correctly handles retries with backoff but never exhausts them under normal load because the semaphore limits the parallel pressure on the SQLite file.
4. Ensure 100% unit test coverage for the new throttling logic.

## Implementation Prompt
You are an Implementer. Implement the design above:
1. Identify `DelegateMission` or the equivalent task delegation function in `srcs/server/orchestration/`.
2. Check the mode using `os.Getenv("OHC_MULTITENANT") == "true"`.
3. In Standalone mode (`OHC_MULTITENANT` != "true" or `OHC_STANDALONE` check), implement a dynamic concurrency limiter (e.g., a semaphore with a low limit like 1 or 2) to wrap the database write operations for task delegation.
4. Ensure the throttling logic has robust error handling and 100% test coverage.
5. Verify tests pass using `bazelisk test //...`.

## Priority
P1

## Estimated Scope
Medium
