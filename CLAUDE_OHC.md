# OHC Hybrid Architecture (OHC-HA) Context

## Overview
One Human Corp (OHC) empowers a single human to orchestrate a vast swarm of AI agents.
The KAIROS Orchestration layer manages:
1. **Shared Task List**: Distributed state machine (PostgreSQL `FOR UPDATE SKIP LOCKED` or SQLite).
2. **Teammate Mesh**: Realtime communication via Redis Pub/Sub or WebSocket.
3. **AutoDream**: Memory consolidation using pgvector in the `swarm_memory` table.

## Core Rules
- **Absolute Autonomy**: Agents propose and execute.
- **Aesthetic Excellence**: `backdrop-filter: blur(20px) saturate(200%)` and `Outfit`/`Inter` fonts.
- **SPIFFE/SPIRE**: Used for all identity and authentication.
