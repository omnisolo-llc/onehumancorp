---
status: "PENDING"
Title: "Master Design Doc: KAIROS AI OS Orchestration (Phase 4)"
Priority: "P0"
Estimated Scope: "Large"
---

# Problem Statement
The OHC Swarm demands a highly scalable, fault-tolerant backbone to coordinate long-running distributed agentic workloads. We lack a Master Design doc that unifies Shared Task List, Teammate Mesh, and AutoDream.

# Research Report
Based on the KAIROS Orchestration Design Doc, we need a unified system.
1. Shared Task List (The Brain): A durable, distributed state machine living in PostgreSQL.
2. Teammate Mesh (The Nerves): A highly available, low-latency communication layer.
3. AutoDream (The Memory): The long-term persistence layer embedding into pgvector.

# Design Doc
The KAIROS Triad unifies these.
- Cloud Architecture uses PostgreSQL `FOR UPDATE SKIP LOCKED`, Redis Pub/Sub, and `pgvector`.
- Standalone Architecture uses SQLite local locks and recency extraction.
- Aesthetic Guidelines apply strictly:
`<style>
body {
  backdrop-filter: blur(20px) saturate(200%);
  background: rgba(255, 255, 255, 0.03);
  font-family: 'Outfit', 'Inter', sans-serif;
}
</style>`

# Implementation Prompt
You are an Implementer agent. Your mission is to implement the KAIROS Triad per the Design Doc.
