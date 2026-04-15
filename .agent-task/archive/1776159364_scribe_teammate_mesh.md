---
title: "Autonomous Implement: Distributed Lock Systems for Teammate Mesh"
priority: P0
estimated_scope: Medium
status: DONE
agent: jules
---

# Title: Autonomous Implement: Distributed Lock Systems for Teammate Mesh

## Problem Statement
The OHC swarm requires a distributed locking mechanism to prevent agents from simultaneously modifying shared resources, ensuring the Swarm Intelligence Protocol (OHC-SIP) remains conflict-free. The agent needs to define `Lock` and `Unlock` interfaces in `srcs/server/interop/lock.go` that handle both Cloud mode (Redis distributed locks) and Standalone mode (local Mutex).

## Design Specification
- Path: `srcs/server/interop/lock.go`
- Provide a `DistributedLock` interface:
  - `Lock(ctx context.Context, key string, ttl time.Duration) (bool, error)`
  - `Unlock(ctx context.Context, key string) error`
- Implement a `NewDistributedLock()` function:
  - Check `REDIS_URL` and `OHC_STANDALONE` (similar to `NewTeammateMesh`).
  - Fallback to local memory implementation if `rueidis.ParseURL` fails.
  - In cloud mode, use `SET key value NX PX ttl`.
- Update `srcs/server/interop/BUILD.bazel` to include `lock.go` in the `interop` library.
- Write tests in `srcs/server/interop/lock_test.go` and include it in `BUILD.bazel`.

