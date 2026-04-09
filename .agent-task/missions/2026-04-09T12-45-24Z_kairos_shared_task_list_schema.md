---
Title: "Implement KAIROS Distributed State Machine Tracker"
Problem Statement: "The OHC swarm relies on the Teammate Mesh for coordination. However, without a distributed state machine, task dependencies are brittle: if an agent crashes mid-task, the DAG (Directed Acyclic Graph) of dependencies can stall permanently. This state machine enforces deterministic transitions (e.g., PENDING -> ASSIGNED -> IN_PROGRESS -> REVIEW -> COMPLETED | FAILED)."
Research Report: "Based on docs/features/kairos/state_machine.md, we need a distributed state machine to track and transition agent coordination states reliably. In Cloud Mode, we use Redis (rueidis) SET NX EX or PostgreSQL transaction (FOR UPDATE) to acquire locks. In Standalone Mode, we use SQLite transaction. We need a `state_machine_transitions` table for audit log."
Design Doc: "1. Database Schema: Create a new migration file for `state_machine_transitions` (already present as 027_state_machine_transitions.sql or similar). 2. Distributed Locking: Ensure the mechanism can gracefully degrade between Postgres `FOR UPDATE` and SQLite. 3. Teammate Mesh Integration: Emit an event to the Pub/Sub Teammate Mesh upon every successful state change. 4. Visual Excellence: Any UI components must adhere to the OHC Premium Feel (Glassmorphism)."
Implementation Prompt: "1. Read docs/features/kairos/state_machine.md. 2. Implement the state machine tracker in Go (e.g., `srcs/server/orchestration/state_machine.go`). 3. Ensure distributed locks are used correctly based on the dialect (Postgres vs SQLite). 4. Emit events via Centrifuge/Redis Pub/Sub. 5. Update Grafana dashboards if needed. 6. Write comprehensive tests."
Priority: "P0"
Estimated Scope: "Large"
---
