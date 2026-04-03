---
status: DONE
agent: jules
---
# 🔬 Mission: Implement Teammate Mesh Architecture

**Priority**: P0
**Estimated Scope**: Large

## 1. Problem Statement
The OHC Swarm communicates through various mechanisms, but lacks a unified "Teammate Mesh" communication layer. The project needs a robust realtime communication layer inside OHC using WebSockets, gRPC, and Redis Pub/Sub to ensure zero-latency interop across Cloud and Standalone modes.

## 2. Research Report
Currently, the system uses Centrifuge for some WebSocket communications, but a dedicated Teammate Mesh adapter `TeammateMesh` component needs to be explicitly created in `srcs/server/interop/mesh.go` that handles mode switching:
- **Cloud-Native**: Redis Pub/Sub via Centrifuge.
- **Standalone**: Local IPC / In-memory Pub/Sub.

## 3. Design Doc
### Architecture
- **Mesh Component**: Create `srcs/server/interop/mesh.go` defining the `TeammateMesh` struct.
- **Interfaces**:
  - `Publish(ctx context.Context, channel string, data []byte) error`
  - `Subscribe(ctx context.Context, channel string) (<-chan []byte, error)`
- **Configuration**: Uses `config.GetMode()` to switch between Redis and Memory backend.

## 4. Implementation Prompt
You are an Implementer agent. Your task is to build the `TeammateMesh` Architecture.
1. **File to create**: `srcs/server/interop/mesh.go` and `srcs/server/interop/mesh_test.go`.
2. **Logic**: Implement `TeammateMesh` that uses Redis Pub/Sub in Cloud mode, and an in-memory channel-based pub/sub in Standalone mode.
3. **Tests**: Ensure tests cover both Cloud mode (mocked/test Redis) and Standalone mode, avoiding race conditions and ensuring messages are delivered.
4. **Metrics**: Add Prometheus metrics for `mesh_messages_published` and `mesh_messages_received`.
