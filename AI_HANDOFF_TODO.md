# AI Handoff Todo

Branch: `copilot/runtime-cleanup-agent-launcher`

## Completed In This Session

- Rebasing work had already been completed earlier in the session history; current branch content is now on top of latest `main` locally.
- Managed builtin worker control-path messages were migrated to protobuf-backed envelopes in the active runtime path:
  - worker config
  - task assignment
  - kill request
  - task result envelope
- Added worker heartbeat/readiness tracking in the orchestration hub.
- Added `ReportWorkerState` gRPC handling and a hub wait helper for worker readiness.
- Updated the managed worker controller to wait for an explicit `READY` state before treating provisioning as successful.
- Added focused tests for:
  - managed task assignment protobuf encoding
  - worker config protobuf round-trip
  - builtin runner protobuf task/result flow
  - worker-state gRPC reporting
- Validation completed successfully with:
  - `bazel test //srcs/server/agents:agents_test //srcs/server/agents/builtin:builtin_test //srcs/server/orchestration:orchestration_test --test_output=errors`
  - `bazel build //srcs/server/agents/taskrunner:ohc-agent-task`

## Files Touched For The Managed-Worker Protobuf Slice

- `srcs/server/agents/agent_task_worker.go`
- `srcs/server/agents/agent_task_worker_test.go`
- `srcs/server/agents/worker_controller.go`
- `srcs/server/agents/worker_controller_test.go`
- `srcs/server/agents/taskrunner/main.go`
- `srcs/server/agents/builtin/control_messages.go`
- `srcs/server/agents/builtin/grpc_hub_adapter.go`
- `srcs/server/agents/builtin/hub_adapter.go`
- `srcs/server/agents/builtin/runner.go`
- `srcs/server/agents/builtin/runner_test.go`
- `srcs/server/orchestration/service.go`
- `srcs/server/orchestration/service_grpc_test.go`
- `srcs/server/agents/BUILD.bazel`
- `srcs/server/agents/builtin/BUILD.bazel`

## Remaining High-Priority Work

### 1. Valkey Migration

Goal: replace Redis-branded deploy defaults with Valkey-compatible defaults while preserving runtime compatibility where needed.

Primary files to update:

- `deploy/helm/ohc/Chart.yaml`
- `deploy/helm/ohc/values.yaml`
- `deploy/helm/ohc/templates/backend-deployment.yaml`
- `deploy/helm/ohc/templates/chatwoot.yaml`
- `deploy/tests/kind_e2e_test.sh`
- `deploy/tests/deploy_artifacts_test.sh`
- `deploy/docker-compose.yml`
- `README.md`
- any dashboard/wizard strings that explicitly say Redis instead of Valkey

Constraints:

- Keep app env compatibility if code still expects `REDIS_URL` or `REDIS_ADDR`.
- Prefer switching infra/service naming and docs first, then narrow runtime aliases only where necessary.
- Re-run deploy-oriented tests after the migration.

### 2. Root-Level Cleanup

Goal: remove or relocate obvious stale root artifacts without breaking hidden dependencies.

Investigate and decide:

- `Cargo.toml`
- `Cargo.lock`
- `deploy/docker/ohc-core/Dockerfile`
- `test_ohc_hybrid_cli.sh`

Current understanding:

- Rust/core artifacts look stale or inconsistent, but should not be deleted blindly until the `deploy/docker/ohc-core` references are reconciled.
- `test_ohc_hybrid_cli.sh` should likely move under `deploy/tests/` or another dedicated test location.

### 3. Deprecated `local` Runtime Quarantine

Goal: continue retiring the deprecated `srcs/server/agents/local` path now that managed builtin workers are the main durable path.

Likely tasks:

- audit `srcs/server/agents/local/BUILD.bazel`
- remove remaining non-legacy dependencies from `srcs/server/agents/BUILD.bazel`
- decide whether local tests should be kept as legacy coverage or moved behind a narrower/manual target

Do not do this opportunistically during unrelated changes; make it a deliberate follow-up so regressions stay attributable.

### 4. Broader Protobuf Consolidation

The user originally asked for all plain objects/configs to move toward protobuf. This session only completed the worker control-plane slice.

Recommended next scope, in order:

- identify still-live ad hoc JSON/control structs in orchestration and agent runtime paths
- convert only active inter-process or inter-service payloads first
- avoid repo-wide blanket conversion in one pass

### 5. 1000-Worker Load Test

Goal: add a dedicated load or soak test for hiring and tearing down 1000 managed builtin workers.

Notes from earlier research:

- a Bazel-aware process-backed test is possible but more complex than a mocked unit test
- use a Bazel `data` dependency and runfiles if launching the real worker binary from a Go test
- existing `builtinclient` gRPC tests provide a useful server startup pattern

Preferred outcome:

- at least one explicit benchmark or manual load test target for worker steady-state overhead
- measure startup success rate, readiness latency, and memory/connection overhead

## Validation Shortlist For Next Session

After the next change batch, prefer running focused targets instead of whole-repo tests first:

- `bazel test //srcs/server/agents:agents_test --test_output=errors`
- `bazel test //srcs/server/agents/builtin:builtin_test --test_output=errors`
- `bazel test //srcs/server/orchestration:orchestration_test --test_output=errors`
- `bazel build //srcs/server/agents/taskrunner:ohc-agent-task`

For Valkey work, add the relevant deploy targets/tests once that patch is ready.

## Notes For The Next AI

- The main managed-worker protobuf migration is already working and tested.
- `srcs/server/agents/builtin/control_messages.go` was briefly user-edited during the session and had to be reconciled; current version is the resolved version.
- The generated Go proto API in this repo uses builder-style construction. When adding new protobuf-backed messages, prefer `*_builder{...}.Build()` and the generated getters.
- The branch had been rebased earlier and will likely require `git push --force-with-lease` rather than a normal push.