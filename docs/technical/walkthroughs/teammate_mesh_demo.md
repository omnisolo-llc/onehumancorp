<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Teammate Mesh Demo Walkthrough

This walkthrough demonstrates the real-time collaboration capabilities of the OHC Swarm using the Teammate Mesh.

## 1. Setup
Ensure the `ohc-mesh-hub` service is running. In a local environment, this will fallback to an in-memory SQLite-backed event bus.

## 2. Scenario: Multi-Agent Collaboration
We will simulate two agents, "Agent Alpha" and "Agent Beta", collaborating on a shared task pool.

- **Step 1:** Dispatch a multi-part mission to the Orchestration Hub.
- **Step 2:** Watch as Agent Alpha claims "Task 1" via `FOR UPDATE SKIP LOCKED`.
- **Step 3:** Agent Alpha streams state updates to the `ohc.mesh.agent.status` channel.
- **Step 4:** Agent Beta observes Agent Alpha's progress and dynamically adjusts its own execution plan, picking up "Task 2".

## 3. Observability
Open the Hybrid Telemetry Grafana dashboard. You will see live metrics reflecting the locking mechanisms and inter-agent gRPC latencies.

</div>
