# OHC Hybrid Agentic OS: Fault Injection Framework

## Overview

As part of the OHC Reliability Mandate, we have implemented a native Fault Injection Framework within the core database layer. This framework allows for programmatic simulation of real-world failures (latency spikes, connection errors) across both Cloud and Standalone environments without requiring external proxy tools.

## Architecture

The framework is integrated into the `DB` struct in `src/server/db.rs`. It utilizes an asynchronous, thread-safe configuration map that dictates when and how individual database operations should fail.

### Core Components

1.  **`FaultInjectionConfig`**: Defines the failure parameters for a specific operation.
    *   `delay`: An optional `Duration` to sleep before executing the operation (simulates network/disk latency).
    *   `error_rate`: A float (0.0 to 1.0) representing the probability of the operation failing with an artificial error.

2.  **`DB::set_fault`**: An async method to configure a fault for a specific operation name (e.g., "upsert_mission").

3.  **`DB::execute_with_retry`**: The primary execution gate. It intercepts all calls to check if a fault should be injected *before* attempting the actual database interaction.

## Usage in Chaos Engineering

Chaos experiments can now be written as standard Rust tests. By configuring faults on a shared `Arc<DB>` instance, we can verify how higher-level services (like the `SipDB` or `AgentMissionWorker`) handle underlying resource degradation.

### Example: Simulating Database Latency

```rust
// In a chaos test
db.set_fault("upsert_mission", Some(Duration::from_secs(2)), 0.0).await;

// This call will now take at least 2 seconds, triggering timeout logic in the caller
let result = sip_db.upsert_mission(id, status, payload, true).await;
assert!(result.is_err()); // Assuming 1s timeout in caller
```

## ML-Resilience Parity

This framework is identical in Cloud (Postgres) and Standalone (SQLite) modes. This ensures that a circuit breaker verified in Standalone mode will behave identically when deployed to the Cloud, maintaining the "Absolute Mode Parity" requirement.

## Future Expansion

*   **Network Partition Simulation**: Expanding the framework to the gRPC and Mesh transport layers.
*   **Memory Exhaustion**: Adding a hook to artificially inflate memory usage during specific tasks to test graceful degradation.
*   **Dynamic Faults via API**: Exposing a restricted `/api/v1/chaos/faults` endpoint for E2E tests to inject faults during Playwright runs.
