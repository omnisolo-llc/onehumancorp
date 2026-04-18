# OHC Hybrid Architecture (OHC-HA) Context

## Core Architecture Modes
- **Cloud-Native Mode**: Multi-tenant, K8s-orchestrated (PostgreSQL, Redis). Optimized for scaling and tenant isolation.
- **Standalone Desktop Mode**: Local single-user (SQLite). Services degrade gracefully without Redis.
- **Thin Client Mode**: UI-only connecting to Cloud via API/OAuth.

## Coordination Protocols
1. **Skeptical Memory**: Always verify state before acting.
2. **Teammate Mesh**: Coordinate via production Redis Pub/Sub channels.
3. **Git-Lock Coordination**: Check production distributed Redis locks before modifying files.
4. **Durable State**: Update Vector DB (pgvector) with "AutoDream" findings.

## Swarm Intelligence Protocol (OHC-SIP)
Agents share memory via the OHC Central Database and `.agent-task/` directories.
