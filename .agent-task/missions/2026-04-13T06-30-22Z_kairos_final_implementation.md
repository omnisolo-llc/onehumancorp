<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# 🚀 Mission: Architect and Implement KAIROS Orchestration
**Problem Statement:** The OHC swarm lacks a complete, cohesive implementation of the centralized, distributed KAIROS Orchestration layer, preventing agents from effectively coordinating and tracking tasks across the hybrid architecture.

**Research Report:** Evaluated PostgreSQL `FOR UPDATE SKIP LOCKED` for horizontal pod concurrency, proving ideal for the Shared Task List. Evaluated Redis Pub/Sub vs WebSockets for the Teammate Mesh, selecting Redis for cloud and SQLite/In-Memory fallback for local. Evaluated `pgvector` for AutoDream memory consolidation.

**Design Doc:** See `docs/architecture/kairos_orchestration_final_design.md`. The KAIROS Triad consists of the Shared Task List (PostgreSQL `FOR UPDATE SKIP LOCKED`), Teammate Mesh (Redis Pub/Sub via `mesh:tasks` and `mesh:coordination`), and AutoDream Pipeline (`pgvector` in `consolidated_memory`). Sub-agents are managed via `sub_agent_queue`.

**Implementation Prompt:** You are an Implementer agent. Your mission is to complete the KAIROS Triad in `srcs/server/orchestration/`.
1. Implement the Shared Task List database schema migrations (`shared_tasks_v4` and `sub_agent_queue`) using Goose annotations (`-- +goose Up`).
2. Ensure the Shared Task logic uses `FOR UPDATE SKIP LOCKED` for Postgres claiming and degrades to application mutexes for Standalone Mode.
3. Verify Teammate Mesh and AutoDream functionality (`srcs/server/orchestration/autodream.go`).
4. Run `bazelisk test //...` and `bazelisk coverage //...` (parsing LCOV) to verify >90% coverage.
Protect shared states with `sync.Mutex`.

**Priority:** P0
**Estimated Scope:** Large
</div>
