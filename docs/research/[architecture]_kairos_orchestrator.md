# OHC KAIROS Orchestrator Architecture Research Report

## 1. Executive Summary

This report documents the architectural design and integration plan for the core KAIROS Orchestrator within the OneHumanCorp (OHC) platform. It evaluates the system from the perspective of our core personas (e.g., Maya, Carlos, Priya) to ensure the AI swarm operates seamlessly without race conditions, and that any orchestration complexity is abstracted behind a simple "1-Tap" mobile interface.

## 2. Findings

-   **Current State:** The platform has a hierarchical agent architecture but inter-agent communication and state management require formalization to support complex business workflows without deadlocks.
-   **User Need:** Business owners need the swarm of agents to work together like a cohesive team. They shouldn't have to resolve technical disputes between the Sales Agent and the Operations Agent.
-   **Technical Gap:** We need to implement a robust Orchestrator utilizing a Shared Task List and Teammate Mesh, backed by distributed locking, to handle complex multi-agent handoffs safely.

## 3. Recommended Architecture

The architecture will introduce the KAIROS Orchestrator as the central nervous system:
1.  **Shared Task List:** A distributed, tenant-scoped queue of pending actions.
2.  **Teammate Mesh:** An event bus for inter-agent communication.
3.  **State Transition Manager:** A system to track the lifecycle of multi-step, multi-agent workflows.

### Key Mechanisms:
-   **Distributed Locking:** Ensuring only one agent can modify a specific resource (e.g., an Order) within a specific `tenant_id` at any time.
-   **Idempotency:** Guaranteeing that retried tasks do not result in duplicate actions.
-   **Mobile-First Visibility:** Surfacing orchestration state via a simple "Swarm Activity" feed on the mobile dashboard.

## 4. Next Steps

1.  **Phase 1:** Implement the Shared Task List and Teammate Mesh interfaces in the Rust backend.
2.  **Phase 2:** Implement the distributed locking mechanism and update the agent execution loop.
3.  **Phase 3:** Develop the mobile UI for the Swarm Activity feed.

(Note: See `docs/research/[architecture]_kairos_orchestrator.md` for the full design document and Implementer prompt).
