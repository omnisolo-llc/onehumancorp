<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# OHC-SIP Capabilities: Architectural Overview

**Version:** 1.0.0
**Target Audience:** Core Developers, AI Orchestration Engineers

## 1. Introduction to Swarm Intelligence Protocol (OHC-SIP)

The One Human Corp (OHC) Swarm Intelligence Protocol (OHC-SIP) dictates how the autonomous agent swarm operates, coordinates, and persists memory across the **Hybrid Architecture (OHC-HA)**. This document outlines the fundamental capabilities exposed by the OHC-SIP to orchestrate swarm nodes efficiently and autonomously.

## 2. Core Capabilities

### 2.1 Teammate Mesh Cross-Mode Handoff
Agents often need to hand off tasks depending on the computational context and required specialty.
- **Cloud Mode:** Relies on Redis Pub/Sub (`mesh:tasks` channel) using the high-performance `redis` library. This allows distributed, stateless API pods to communicate seamlessly.
- **Standalone Desktop Mode:** Fallbacks gracefully to the local SQLite `shared_tasks` table to synchronize handoffs.

### 2.2 Skeptical Memory & AutoDream
The **AutoDream** capability consolidates architectural findings and prevents the "hallucination creep" commonly seen in autonomous agents.
- **Durable Vector Embeddings:** Leveraging PostgreSQL (`pgvector`) in Cloud mode and falling back to a custom schema in SQLite (with `BLOB` datatypes in place of `VECTOR`) for Standalone mode.
- **Transparent LLM Caching:** Before hitting the Minimax API to compute an embedding, the system aggressively checks the L1 (`redis`) and L2 (SQLite/Postgres `embedding_cache` table) caches.

### 2.3 Visual Excellence Mandate
A key capability of OHC is aesthetic superiority. Every agent, when generating a UI artifact or documentation, enforces the **Glassmorphism** standard.
- *Rule of Thumb:* `backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif;`

### 2.4 Hermetic Sandboxing
Agents execute within Bazel's hermetic sandboxes. This prevents dirty states and non-deterministic builds.
- When generating code, agents must never modify `BUILD.bazel` manually; instead, they trigger `bazelisk run //:gazelle` to synchronize targets automatically.

## 3. Designing a New Hybrid Capability

To build a new capability that satisfies the OHC-SIP, adhere to the **Hybrid Architecture Degradation Principle**:

1. **Start with the Interface:** Define the behavior as an abstract Go interface.
2. **Implement Cloud-Native (`OHC_MULTITENANT=true`):** Build the PostgreSQL and Redis implementations. Ensure explicit tenant isolation (`WHERE organization_id = $1`).
3. **Implement Standalone (`DATABASE_URL=sqlite://...`):** Build the SQLite fallback. Ensure it requires zero network calls and minimal memory overhead.
4. **Wire the Injection:** Use the provider injection pattern in `src/server/` to inject the correct implementation at runtime based on the environment variables.

<div style="margin-top: 20px; padding: 15px; border-left: 4px solid #4CAF50; background: rgba(76, 175, 80, 0.1);">
  <strong>Note:</strong> Always verify your new capabilities across both the Cloud and Standalone environments using <code>bazelisk test //...</code>.
</div>

</div>
