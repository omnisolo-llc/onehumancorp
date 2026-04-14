<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: 'Outfit', 'Inter', sans-serif; background: rgba(255, 255, 255, 0.03); color: #fff; padding: 20px; border-radius: 12px; border: 1px solid rgba(255, 255, 255, 0.1);">

# Title: KAIROS: Phase 3 - Implement AutoDream Vector Pipelines

## Problem Statement
Ephemeral memory contexts are lost between agent sessions, violating the Swarm Intelligence Protocol (OHC-SIP).

## Research Report
Memory must be consolidated into vector embeddings (`pgvector` in Cloud, local vector proxy in Standalone) to allow contextual RAG retrieval for future tasks.

## Design Doc
Implement an `AutoDreamWorker` daemon that sweeps `state_machine_transitions`, chunks text, queries the LLM API for embeddings, and stores them in `autodream_memories`.

## Implementation Prompt
Hello Implementer! Implement the `AutoDreamWorker` in `srcs/server/memory/autodream/autodream_pipeline.go`. Create an async worker that polls for COMPLETED tasks, generates embeddings, and upserts them into `autodream_memories`.

## Priority
P1

## Estimated Scope
Medium

</div>
