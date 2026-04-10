<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05); color: #fff;">

# Teammate Mesh

The Teammate Mesh is the "Nerves" of the OHC Hybrid AI OS, providing a highly available, low-latency communication layer for agents to broadcast state changes, advertise capabilities, and stream real-time events.

## 1. Architecture

- **Cloud-Native Mode:** Relies on Redis Pub/Sub (`rueidis`) combined with a `CentrifugeNode` hub to horizontally scale real-time broadcasting.
- **Standalone Mode:** Uses an in-memory (`MemoryMeshTransport`) event bus or local SQLite-backed mechanism.

## 2. Protocol Definitions

Real-time events are defined via Protobuf RPCs.

```protobuf
message MeshEvent {
  string event_id = 1;
  string topic = 2;
  bytes payload = 3;
  int64 timestamp = 4;
}
```

## 3. Telemetry Integration

Every endpoint within the Teammate Mesh must be instrumented with OpenTelemetry metrics for deep observability of mesh latency, throughput, and connection drops. Metrics are directly piped to Prometheus in Cloud deployments or buffered locally via SQLite in Standalone scenarios.

</div>
