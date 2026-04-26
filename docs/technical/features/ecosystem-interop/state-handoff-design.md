# Design Doc: State Handoff Between Modes

**Author(s):** Principal Interoperability Engineer & Link (L7)
**Status:** Approved
**Last Updated:** 2026-04-26

## 1. Overview
This document defines the architectural protocol for "State Handoff Between Modes", ensuring that when a business owner switches the KAIROS AI OS from Cloud to Standalone (or vice versa), the mission state, AI context, and customer data are synchronized identically without loss or duplication.

## 2. Goals & Non-Goals
### 2.1 Goals
- Define a durable state synchronization protocol operating via `TeammateMesh`.
- Ensure idempotency so re-running handoff does not duplicate state.
- Guarantee collision-free handoffs by integrating `DistributedLock`.
- Ensure 100% interoperability and test coverage.

### 2.2 Non-Goals
- Replacing `LangGraph` checkpointers completely. We are synchronizing the `interop.State` container used across frameworks.

## 3. Protocol Design
### 3.1 Coordination & Lock
Handoff relies on the Redis-backed (Cloud) or File-based (Standalone) `DistributedLock`. The key pattern `ohc:lock:handoff:{tenant_id}` ensures a singular execution trajectory during synchronization.

### 3.2 Idempotent Execution
The system tracks successful state imports locally. If a `State` ID has already been imported during the session, it is safely ignored.

### 3.3 Handoff Mechanism
- **ExportToStandalone**: Publishes an encoded `TeammateMesh` event on channel `mesh:handoff`.
- **ImportFromCloud**: Listens to `mesh:handoff`, deserializing payloads back into `interop.State` while applying idempotency checks.
