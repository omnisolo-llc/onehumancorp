status: "PENDING"
agent: "KAIROS Orchestrator"
---
Title: "KAIROS Master Design Doc: Hybrid AI OS Orchestration Implementation"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
The One Human Corp (OHC) platform demands a structural and aesthetic vision for the "Hybrid Agentic OS" that unifies distributed agent orchestration. The Swarm requires a centralized KAIROS orchestration layer to decompose complex feature requests, coordinate safely across Cloud-Native and Standalone modes, and maintain long-term memory, while enforcing the OHC premium aesthetic.

# Research Report
- Based on the Hybrid Architecture (`OHC-HA`), OHC must degrade gracefully from Kubernetes/PostgreSQL (Cloud) to SQLite/local mutexes (Standalone Desktop).
- KAIROS relies on three foundational pillars for agent orchestration:
  1. **Shared Task List (Phase 1):** Durable distributed state machine for sub-task management.
  2. **Teammate Mesh (Phase 2):** Realtime pub/sub communication (CentrifugeNode + Redis/Local Memory).
  3. **AutoDream Pipeline (Phase 3):** Background memory consolidation using `pgvector` or local fallback.
- OHC mandates Visual Excellence. Any UI downstream from these architectures must reflect the premium "Glassmorphism" identity.

# Design Doc
**Architecture Overview & Decomposed Features:**
The KAIROS Orchestration engine will decompose tasks and coordinate agents via a distributed State Machine tracker, managing the Swarm through a robust Sub-Agent Queue.

**Database Schema Overview:**
- `shared_tasks`: Tracks global decomposed features (`FOR UPDATE SKIP LOCKED` vs Mutex).
- `state_machine_transitions`: Observability audit log for task state changes.
- `autodream_memories`: Vector DB index for long-term embedded truth.

**Visual Excellence Mandate (CRITICAL):**
All UI components rendering this orchestration layer MUST implement:
```css
backdrop-filter: blur(20px) saturate(200%);
background: rgba(255, 255, 255, 0.03);
font-family: 'Outfit', 'Inter', sans-serif;
```

# Implementation Prompt
You are an Implementer agent. Your mission is to actualize the KAIROS Master Design Doc into the OHC platform.
1. Implement the `Shared Task List` data structures and `TaskQueue` background processors in `srcs/server/orchestration/queue/queue.go` and `srcs/server/db/migrations/`.
2. Construct the `Teammate Mesh` APIs in `srcs/server/orchestration/hub.go` using `CentrifugeNode`. Ensure it falls back to memory if `rueidis` is not present in Standalone Mode.
3. Build the `AutoDream` background embedding worker in `srcs/server/orchestration/autodream_pipeline.go`, updating `autodream_memories` table via Minimax LLM vectors.
4. Provide >95% unit test coverage for all backend logic. Use `db.NewTestProvider(t)` and `auth.ClaimsContextKeyForTest` for context injection.
5. Create a dashboard UI using Flutter for KAIROS observability, applying the exact CSS styling variables provided in the Design Doc.
