<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.05); color: #fff;">
# Chaos Engineering Resilience Architecture

## Overview
The OHC Hybrid Agentic OS employs rigorous chaos engineering practices to ensure absolute autonomy and resilience across Cloud, Standalone, and Thin Client modes. This architecture is designed to continuously inject controlled failures into the Agent Harness and cluster orchestrator, allowing the system to self-heal and adapt to market reality.

## Injection Framework
Failure injection is orchestrated via a dedicated sub-swarm. This swarm safely injects faults across nodes, tests our graceful degradation in Standalone Mode, and verifies the "Zero Secrets" SPIFFE/SPIRE identity architecture under stress.

## Specific Failure Modes: CorruptAgentLock
A critical new failure mode is `CorruptAgentLock`. This mode specifically targets the distributed Git-Lock coordination mechanism (backed by Redis in Cloud mode and SQLite in Standalone).
- **Mechanism:** The chaos agent intentionally corrupts the lock state or simulates a split-brain scenario.
- **Resilience Goal:** The orchestrator must detect the corrupted lock, safely fence off the affected agent, and transparently failover the operation without losing any durable state or violating the Swarm Intelligence principles.

## Interpreting the Dashboard
The `chaos_resilience.json` Grafana dashboard is your primary lens into the swarm's health during chaos events.
- **Recovery Time Objective (RTO):** Visualizes the time taken for the system to recover from an injected failure. A spike indicates a resilience gap.
- **Lock Contention Metrics:** Pay special attention to the `CorruptAgentLock` panels. They display the rate of lock corruption events and the percentage of successful automated recoveries.
- **Token Burn Rate:** Chaos events should not cause uncontrolled token burn. Monitor the extrapolated 24h burn rate forecast to ensure budget compliance during stress tests.
</div>
