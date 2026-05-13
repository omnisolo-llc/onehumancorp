# OHC Hybrid Agentic OS: Chaos Engineering and Reliability Deep Dive

## 1. Introduction

The OHC Hybrid Agentic OS is designed to operate seamlessly across two fundamentally different environments: a multi-tenant Cloud environment powered by PostgreSQL and a single-tenant Standalone environment powered by SQLite. Achieving "Absolute Mode Parity" while maintaining high reliability requires a sophisticated approach to testing and failure management.

This document details the reliability architecture, the chaos engineering suite, and the ML-Resilience rules implemented to ensure the system's robustness.

## 2. Reliability Architecture

### 2.1. Native Fault Injection

Unlike traditional chaos engineering which often relies on network-level proxies (e.g., Toxiproxy), OHC implements a **Native Fault Injection Framework** within the database abstraction layer (`src/server/db.rs`).

This allows us to:
- Simulate latency for specific SQL operations.
- Inject random errors with configurable probability.
- Test mode-specific behaviors (e.g., SQLite lock contention vs. Postgres transaction isolation).

### 2.2. ML-Resilience Core

AI agents introduce unique failure modes:
- **LLM API Unavailability**: Managed via a 3-attempt retry loop with exponential backoff and a persistent "PAUSED" state.
- **Malformed Responses**: Handled through internal validation and re-prompting.
- **Long-Running Tasks**: Enforced via a strict 60-second timeout at the harness level.
- **Token Budgeting**: Enforced server-side to prevent runaway costs and resource exhaustion.

### 2.3. Idempotent Operations

To safely retry agent jobs, all mutating tools (e.g., `Write`, `Edit`) are implemented with **Idempotency Checks**. Before performing a write, the tool verifies if the desired state already exists on disk, skipping the operation if it does. This prevents "compounding edits" where multiple retries of the same task lead to corrupted files.

## 3. Chaos Engineering Suite

The chaos suite in `src/server/chaos.rs` covers several critical scenarios:

### 3.1. SQL Sync Lag Simulation

In Standalone mode, missions are synced to the Cloud asynchronously. We simulate lag in this process to ensure the UI remains responsive and eventually consistent. The "Optimistic UI" is verified through E2E tests.

### 3.2. Resource Exhaustion

We mock CPU and Memory exhaustion by injecting high latency into core paths, verifying that the system degrades gracefully by timing out and returning cached data instead of crashing or hanging.

### 3.3. Team Mesh Resilience

The communication layer between agents is tested for:
- **Message Duplication**: Ensuring the `processed_count` only increments once.
- **Mailbox Corruption**: Verifying that invalid or unreadable messages do not crash the agent worker.
- **Lock Race Conditions**: Testing the robustness of the `.agent-lock` mechanism under high concurrency.

## 4. Mode Parity Audit

A major goal of this mission was ensuring functional parity between Postgres and SQLite.

### 4.1. Schema Alignment

We identified and resolved several discrepancies in the `agent_missions` and `onboarding_state` tables. Postgres now correctly supports the `mission_log` and `sync_error` columns used by the Standalone sync daemon.

### 4.2. Behavioral Parity

Tests in `src/server/db.rs` verify that:
- **Tenant Isolation** is enforced correctly via RLS in Postgres and explicit scoping in SQLite.
- **Transaction Handling** fails gracefully in both modes when the database is unavailable.
- **Error Types** are mapped to consistent internal categories (`Transient`, `LlmRecoverable`, `UserFixable`, `Fatal`).

## 5. Metrics and Observability

Reliability is measured through:
- **P99 API Latency**: Tracked under simulated load (100 Cloud users, 10 Standalone users).
- **Error Rates**: Monitored during injected LLM outages.
- **Retry Success Rates**: Verifying the effectiveness of the 3-attempt loop.

## 6. Conclusion

The OHC "Hybrid Agentic OS" now stands as a model of reliability. By combining native fault injection, idempotent AI tools, and a comprehensive chaos engineering suite, we have ensured that the system remains stable and predictable regardless of the deployment mode or environmental conditions.
