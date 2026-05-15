# OHC Interoperability Architecture

## Protocol Design

The `InteropProtocol` guarantees that Cloud and Standalone modes communicate reliably using `protobuf` over an agnostic `Bus`.

### State Handoff

State handoff ensures mission idempotency across deployment modes.

```mermaid
sequenceDiagram
    participant SourceMode (Cloud)
    participant LockProvider (Redis/SQLite)
    participant MessageBus (PubSub/IPC)
    participant TargetMode (Standalone)

    SourceMode->>LockProvider: acquire_lock("handoff:mission_id")
    LockProvider-->>SourceMode: OK
    SourceMode->>LockProvider: acquire_lock("handoff:processed:mission_id")
    LockProvider-->>SourceMode: OK (Idempotency check)
    SourceMode->>MessageBus: publish(proto::StateHandoff)
    MessageBus-->>TargetMode: subscribe(system:state_handoff)
    TargetMode->>TargetMode: Process StateHandoff
    SourceMode->>LockProvider: release_lock("handoff:mission_id")
```

### Job Dispatch

Job dispatches include exponential backoff and timeout logic.

```mermaid
sequenceDiagram
    participant MainServer
    participant MessageBus
    participant BuiltinAgent

    MainServer->>MessageBus: subscribe("system:job_ack:job_123")
    MainServer->>MessageBus: publish("system:job_dispatch:tenant_id", proto::JobDispatch)
    MessageBus-->>BuiltinAgent: receive(JobDispatch)
    BuiltinAgent->>MessageBus: publish("system:job_ack:job_123", proto::JobAck)
    MessageBus-->>MainServer: receive(JobAck)
    MainServer->>MainServer: Proceed with Execution
```

### Health Monitoring

Cross-mode health monitoring uses a ping-ack protocol.

```mermaid
sequenceDiagram
    participant Monitor
    participant Swarm

    Monitor->>Swarm: publish("system:health_ping")
    Swarm-->>Monitor: publish("system:health_ack:node_id")
```

## Distributed Locking

Locks behave consistently across modes:
- **Cloud Mode**: `RedisBus` uses `Redlock` Lua scripts to prevent TOCTOU.
- **Standalone Mode**: `IpcBus` uses an advisory table `bus_locks` in SQLite with `ON CONFLICT` and `expires_at` checks to prevent stale locks.
- **In-Memory**: `MemoryBus` uses standard Tokio Mutex and Instant checks.
