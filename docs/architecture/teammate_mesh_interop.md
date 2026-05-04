# Teammate Mesh Interoperability Protocol

## Overview
The OHC Swarm requires a durable database schema and microservices mapping to decompose high-level feature requests for the agent team. The Teammate Mesh is the highly available low-latency communication layer.

This document describes the protocol that governs how jobs are dispatched, status is reported, and context is synchronized between Cloud and Standalone modes.

## Transport Modes
1. **Cloud Mode**: Uses Redis Pub/Sub (`RedisTransport`) and PostgreSQL for persistent state.
2. **Standalone Mode**: Uses SQLite-backed IPC (`IpcTransport`) for local message routing and locking.
3. **Fallback**: In-memory transport (`MemoryTransport`) is used when the primary data stores are unavailable.

## Distributed Locks
Distributed locks ensure mutual exclusion for accessing shared tenant resources.
- Cloud: implemented via Redlock.
- Standalone: implemented via SQLite advisory locks in the `mesh_locks` table.

## State Handoff
When a business owner switches between Cloud and Standalone modes, the state is synchronized via `mesh:coordination:handoff`. This process uses Last-Write-Wins (LWW) conflict resolution and ignores reflective messages.

## Message Bus Reliability
To ensure the async job dispatch survives network partitions, jobs use an acknowledgment protocol over the `mesh:ack:{msg_id}` topic. Messages are re-published with exponential backoff if no ack is received.
