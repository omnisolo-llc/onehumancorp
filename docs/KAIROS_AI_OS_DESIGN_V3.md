# OHC KAIROS - Architecture & Design Doc

## Overview
This document defines the structural and aesthetic vision for the OHC "Hybrid Agentic OS", specifically focused on the Shared Task List feature and Sub-Agent Orchestration.

## Core Mandates
1. **Absolute Autonomy**: Agents propose and execute based on this vision.
2. **Aesthetic Excellence**: "Premium Feel" required for all OHC interfaces.
   - Glassmorphism: `backdrop-filter: blur(20px) saturate(200%)`, `background: rgba(255, 255, 255, 0.03)`
   - Typography: `font-family: 'Outfit', 'Inter', sans-serif`
3. **Hybrid Consistency**: Must support both Cloud-Native Mode (PostgreSQL, Redis) and Standalone Desktop Mode (SQLite, local memory).

## Architecture & Data Flow
- **Task Decomposition**: KAIROS orchestrates the database schema to decompose complex feature requests.
- **Teammate Mesh**: Realtime communication layer using production Redis Pub/Sub channels (Cloud) or local event bus (Standalone).
- **Sub-Agent Queue**: Background queuing logic (BullMQ/Celery style) for agent orchestration.
- **Durable State**: Production Vector DB (e.g. pgvector/Pinecone) integration for AutoDream architectural consolidation.

## Shared Task List Implementation
- **Schema**: Tables for `shared_tasks`, `task_dependencies`, `sub_agent_queues`. Ensure `convertBindVars` is implemented for robust degraded Standalone Mode fallback (stripping clauses like `FOR UPDATE SKIP LOCKED`).
- **Backend API**: Endpoints located in `srcs/server/api/tasks/queue.go` returning structured queueing logic and ensuring robust >95% test coverage.
- **UI Task List**: Built in Flutter (`apps/desktop/lib/ui/shared_task_list.dart`) explicitly using `ConsumerWidget` with Riverpod for state management. Adheres strictly to the Visual Excellence Mandate.

## Observability
All tasks and orchestrations must export metrics via OpenTelemetry and Prometheus (`prometheus.NewCounterVec`).
