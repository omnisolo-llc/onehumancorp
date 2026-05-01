# Architecture Document: Teammate Mesh Interop Layer

## Objective

Design a robust protocol for ensuring reliable communication and state handoff between the Cloud and Standalone modes of the OHC application, specifically over the Teammate Mesh layer, to ensure the main server and the builtin agent microservice stay in sync.

## Overview

The OHC platform operates in two deployment modes:
- **Cloud:** Distributed microservices leveraging Redis (Pub/Sub & Redlock) for communication and locking.
- **Standalone:** Local execution using local IPC (Memory) for communication and locking.

The protocol between the main server and builtin agent must be the same in both Cloud and Standalone modes — only the underlying transport mechanism changes.

## 1. Teammate Mesh Communication Layer

### Architecture
- **Abstraction:** The communication layer is abstracted via the `TeammateMesh` trait and `MeshTransport` interface. `create_transport` dynamically injects the appropriate transport implementation depending on the operational mode (`RedisTransport` for Cloud, `MemoryTransport` for Standalone).
- **Wire Format:** All communication payload structures on the wire are defined using **Protobuf**. No ad-hoc JSON is permitted. The schema strictly conforms to `TeammateMeshEvent` defined in `src/proto/hub.proto`.
- **Channel Routing:** Jobs are dispatched to agents over well-defined mesh topics (`mesh:tasks`, `mesh:coordination`). Context and statuses are synchronized over standard channels.

## 2. Message Bus Reliability

### Resiliency and Retry Semantics
- **Acknowledgment:** A robust acknowledgment protocol is built into the transport layer. A sender publishes a task via `publish_task` and sets a timer.
- **Retries:** If the main server does not receive an acknowledgment (or status update) from the builtin agent within a designated timeout, the dispatch is retried using exponential backoff to handle transient network partitions or process restarts.
- **Idempotency:** Re-dispatching the same task requires the agent to handle duplicate jobs smoothly. The `Message` protobuf will contain unique request identifiers.

## 3. Distributed Locking

### Consistent Semantics Across Modes
- **Purpose:** Prevent conflict when multiple parts of the swarm (or main server and agent) try to access or mutate the same tenant resource simultaneously.
- **Cloud Mode (Redis):** Implemented using Redis `SET ... NX EX` to guarantee distributed mutual exclusion across multiple replicas and pods (Redlock pattern).
- **Standalone Mode (Memory):** Implemented using an in-memory concurrent map within `MemoryTransport` that replicates the exact timeout and expiration behaviors of Redis locks.
- **Semantics:** Both lock implementations expose an identical `acquire_lock(resource, owner, ttl_seconds)` and `release_lock` interface.

## 4. State Handoff Between Modes

### Seamless Mode Transition Protocol
- **Synchronization Logic:** When transitioning from Cloud to Standalone (or vice versa), the agent mission state, AI context, and customer data must be synchronized.
- **Durable Storage:** The system serializes the context out of Redis/Memory into the persistent Vector DB (`pgvector` or `sqlite-vec`), enabling the new mode to resume from the persisted state.
- **Idempotent Transitions:** Handoff operations are strictly idempotent. Each mission step and synchronization task uses unique markers. If a handoff is interrupted and re-run, it will not create duplicate tasks or customer data.

## 5. Cross-Mode Health Monitoring

### Health Probes and Presence
- **Registration:** Agents periodically broadcast their active status via `register_presence`, supplying their `agent_id`, `status`, and a `ttl_seconds`.
- **Cloud Mode:** `RedisTransport` creates expiring `presence:{agent_id}` keys.
- **Standalone Mode:** `MemoryTransport` tracks presence in a timed in-memory map.
- **Monitoring:** The main server queries `get_active_agents` to monitor builtin agent responsiveness. Unresponsive agents trigger alert mechanisms or initiate task reassignment.

## Conclusion

This interop protocol unifies the message format via Protobuf, implements identical locking paradigms across Cloud and Standalone modes, fortifies the system against transient failures using rigorous acknowledgment semantics, and uses durable data storage for seamless and idempotent state handoffs.
