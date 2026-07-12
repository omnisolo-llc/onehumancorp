# Agent Stream Cancellation Remediation Implementation Plan

> **For agentic workers:** REQUIRED SUB-SKILL: Use superpowers:subagent-driven-development (recommended) or superpowers:executing-plans to implement this plan task-by-task. Steps use checkbox (`- [ ]`) syntax for tracking.

**Goal:** Stop LLM, tool, retry, and memory work promptly when an agent-event consumer disconnects, while bounding every event buffer.

**Architecture:** Keep the existing synchronous event callback and place cancellation at the spawned-producer boundaries. Both `Agent::query` and gRPC `run_task` race their in-flight run future against `Sender::closed()` so dropping the receiver drops the run future; `query` changes from an unbounded channel to a fixed-capacity channel, and terminal errors use awaited sends when the consumer remains present.

**Tech Stack:** Rust 2024, Tokio MPSC/select, Tonic streams, Cargo, Bazel.

---

### Task 1: Cancel and bound `Agent::query`

**Files:**
- Modify: `src/agents/builtin/agent.rs`

- [x] **Step 1: Write the failing receiver-drop regression**

Add a blocking `LlmClient` whose `chat` future sets an atomic flag when dropped. Start `Agent::query`, wait until `chat` begins, drop the returned receiver, and require the flag to become true within one second.

- [x] **Step 2: Verify the current unbounded producer does not cancel**

Run: `cargo test -p ohc_builtin_agent query_stops_when_receiver_is_dropped --lib`

Expected: FAIL by timeout because the unbounded sender never observes receiver closure.

- [x] **Step 3: Bound the channel and race execution against closure**

Change the return type to `tokio::sync::mpsc::Receiver<AgentEvent>`, construct `mpsc::channel(64)`, use `try_send` in the synchronous callback, and wrap `self.run(...)` in `tokio::select!` with `tx.closed()`. Await the terminal `TaskError` send only when the run itself fails.

- [x] **Step 4: Run focused and stream regressions**

Run: `cargo test -p ohc_builtin_agent query_stops_when_receiver_is_dropped --lib && cargo test -p ohc_builtin_agent stream_tests --lib`

Expected: all focused tests PASS.

- [x] **Step 5: Commit query cancellation**

```bash
git add src/agents/builtin/agent.rs
git commit -m "perf: cancel abandoned agent query streams"
```

### Task 2: Propagate gRPC receiver cancellation through retries and memory

**Files:**
- Modify: `src/agents/builtin/service.rs`

- [x] **Step 1: Write the failing gRPC receiver-drop regression**

Inject the same blocking/drop-observable LLM into `AgentServiceImpl`, call `run_task`, wait for the LLM to begin, drop the response stream, and require the in-flight chat future to be dropped within one second.

- [x] **Step 2: Verify the current service producer keeps running**

Run: `cargo test -p ohc_builtin_agent run_task_stops_when_receiver_is_dropped --lib`

Expected: FAIL by timeout because `try_send` errors are discarded and retry execution never checks channel closure.

- [x] **Step 3: Race every paid wait against receiver closure**

In the producer task, select `tx.closed()` against each timed `agent.run` attempt and each retry backoff. Return immediately on closure, and check `tx.is_closed()` before constructing or writing completion memory. Keep the existing capacity of 64.

- [x] **Step 4: Run service and full agent regressions**

Run: `cargo test -p ohc_builtin_agent run_task_stops_when_receiver_is_dropped --lib && cargo test -p ohc_builtin_agent --lib`

Expected: all tests PASS.

- [x] **Step 5: Commit service cancellation**

```bash
git add src/agents/builtin/service.rs
git commit -m "perf: stop agent tasks when clients disconnect"
```

### Task 3: Verify and record F-04 remediation

**Files:**
- Modify: `docs/reports/production_agent_optimization_report.md`

- [ ] **Step 1: Run targeted formatting and static checks**

Format `agent.rs` and `service.rs` with child-module traversal disabled, then run `cargo check -p ohc_builtin_agent`.

- [ ] **Step 2: Run Bazel verification**

Run: `bazel test //src/agents/builtin:ohc_builtin_agent_lib_unit_test`

Expected: the target PASSes.

- [ ] **Step 3: Record remediation evidence**

Mark F-04 remediated without removing its original audit text. Record the bounded capacity, cancellation races, focused regressions, full Cargo count, and Bazel result.

- [ ] **Step 4: Commit report evidence**

```bash
git add docs/reports/production_agent_optimization_report.md docs/superpowers/plans/2026-07-12-agent-stream-cancellation-remediation.md
git commit -m "docs: record agent stream cancellation remediation"
```
