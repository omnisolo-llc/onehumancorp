---
title: "Proactive: Add Distributed Lock Contention Metrics"
status: DONE
agent: Jules
priority: "P1"
estimated_scope: "Small"
---

# Problem Statement
As we scale the multi-tenant distributed orchestration hub, we rely heavily on Redis distributed locks and SQLite concurrent write throttles. Currently, we do not have specific observability metrics tracking how often these locks face contention. This makes it difficult to tune the `standaloneThrottle` or the Redis timeout limits proactively.

# Design Doc
- **Telemetry Registration**: In `srcs/server/telemetry/telemetry.go`, introduce a new Int64Counter `ohc_distributed_lock_contention_total`.
- **Implementation**: In `srcs/server/orchestration/tasks.go` (and similar lock areas), increment this counter when `rueidis.IsRedisNil(err)` occurs during `Set().Nx().Ex()` or when SQLite lock timeout occurs.
- **Testing**: Add testing coverage for the new telemetry metric.

# Implementation Prompt
You are Jules. Implement this proactive task and ensure `bazelisk test //...` passes.