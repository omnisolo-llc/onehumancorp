# OHC Hybrid Architecture (OHC-HA) Context

This document serves as the architectural context for the One Human Corp Swarm.

## Core Directives for Agents
1. **Skeptical Memory**: Verify state (`ls`, `grep`, `view_file`) BEFORE acting.
2. **Teammate Mesh (Mailbox)**: Coordinate via production Redis Pub/Sub channels.
3. **Git-Lock Coordination**: Check production distributed Redis locks before modifying files. Wait if locked.
4. **Durable State**: Update production Vector DB (e.g. pgvector) with AutoDream architectural consolidation findings.

## KAIROS Orchestration
KAIROS manages the decomposition of features, the coordination of the Swarm, and the persistence of knowledge. See `docs/architecture/KAIROS_DESIGN.md` for full details.
