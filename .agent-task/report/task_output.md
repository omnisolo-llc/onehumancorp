# [database] Integrate sqlite-vec for Local Vector Search Parity

## Title
Integrate `sqlite-vec` for Local Vector Search Parity

## Problem Statement
The OHC Hybrid Architecture currently handles vector embeddings for the AutoDream pipeline using `pgvector` in Cloud-Native PostgreSQL deployments. However, in Standalone Desktop Mode, local vector embeddings are managed using a suboptimal local blob or approximate index in SQLite. This creates a feature disparity and prevents true offline AI memory capabilities, limiting the Swarm's autonomous functioning on local desktops without heavy workarounds.

## Research Report
- **Goal**: Implement true local vector search for the Standalone Desktop Mode by integrating `sqlite-vec`, an open-source vector search extension for SQLite.
- **Why sqlite-vec**:
  - Extremely fast and lightweight, specifically designed for SQLite.
  - Written in C, easily loadable as a runtime extension or statically linked in Go via CGO.
  - Supports standard vector operations (L2 distance, cosine similarity, inner product) mirroring `pgvector`.
  - Enables true local RAG (Retrieval-Augmented Generation) and semantic memory for agents.
- **Architecture Validation**:
  - Cloud Mode: `pgvector` is already operational.
  - Standalone Mode: Replaces manual blob/approximate index with `sqlite-vec` virtual tables (e.g., `CREATE VIRTUAL TABLE consolidated_memory_vec USING vec0(embedding float[1536])`).
  - Integration: Update `srcs/server/db/provider.go` to load the `sqlite-vec` extension when connecting to SQLite.

## Design Doc
1. **Dependency Integration**:
   - Add `github.com/asg017/sqlite-vec-go-bindings` or compile the extension dynamically in the Bazel build rules.
2. **Database Schema Update**:
   - Update `docs/features/kairos/master_design_doc.md` and related database migrations.
   - For SQLite, create a `vec0` virtual table alongside the main `consolidated_memory` table and set up triggers to keep them synchronized.
3. **API Contracts**:
   - Standardize the `db.Provider` interface for vector searches so `SearchEmbeddings(ctx, vector)` seamlessly translates to `<->` in `pgvector` and `vec_distance_L2` in `sqlite-vec`.

## Implementation Prompt
"Integrate `sqlite-vec` into the OHC Standalone Desktop Mode. Modify the database provider in `srcs/server/db/` to ensure the `sqlite-vec` extension is loaded when connecting to SQLite. Update the AutoDream pipeline logic to utilize the `vec0` virtual table for semantic searches in Standalone Mode, matching the behavior of `pgvector` in the Cloud-Native mode. Ensure all E2E tests for memory retrieval pass in both PostgreSQL and SQLite configurations."

## Priority
P1

## Estimated Scope
Medium
