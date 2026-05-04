# Teammate Mesh Communication Layer Protocol

## Title
Protocol for Cross-Mode Synchronization and Job Dispatch

## Problem Statement
The OHC Swarm communicates across both Cloud and Standalone mode. The protocol must allow the main server and the builtin agent microservice to dispatch jobs, report status, and sync contexts regardless of whether the business owner is online (Cloud with Redis) or offline (Standalone with SQLite/IPC). The main server and agent microservice must stay perfectly synchronized.

## Architecture
The protocol is entirely protobuf-based over a pub/sub event bus defined by `TeammateMesh`.
1. **Cloud Mode**: The transport uses a Redis Pub/Sub backend via `RedisTransport`. Messages are published using the `TeammateMeshEvent` envelope.
2. **Standalone Mode**: The transport uses an IPC mechanism backed by an embedded SQLite table (`mesh_messages`) via `IpcTransport`. Messages follow the exact same `TeammateMeshEvent` protobuf format.

## Cross-Mode State Handoff
State handoff between Cloud and Standalone environments is handled by the `HandoffManager` using the `SyncStateHandoff` message format.
- State updates are broadcasted to the `mesh:coordination:handoff` topic.
- A LWW (Last-Writer-Wins) mechanism using a timestamp prevents stale data from overriding newer data in the database.
- A reflection prevention mechanism (`mode_source`) ensures an instance doesn't apply state updates it originated.
- An explicit acknowledgment mechanism is built-in (`mesh:ack:<msg_id>`).

## Job Dispatch
Jobs are dispatched via the `TaskDecompositionService`.
- Tasks are encoded as `SharedTask` JSON blobs and sent over the `task.assigned` topic.
- In both Standalone and Cloud mode, the respective transport transparently delivers this task.
- Lock coordination prevents multiple agents from acting on the same task simultaneously. Lock keys use the pattern `ohc:lock:{tenant_id}:{resource_type}:{resource_id}`.

## Implementation Prompt
This research report validates the TeammateMesh protobuf architecture, confirming that the Rust implementation provides equal capability offline via `IpcTransport` and online via `RedisTransport`, utilizing the standard `TeammateMeshEvent` envelope.
