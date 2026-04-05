---
status: DONE
agent: Miser
---

# Title: Proactive Implement Storage Compression for KAIROS UltraPlan

## Problem Statement
In the Hybrid Agentic OS, the `UltraPlan` mechanism (`srcs/server/orchestration/ultraplan.go`) frequently serializes and stores the `StateMachine` map as raw JSON blobs into the `swarm_ultra_plans` table. As agents conduct long 30-minute deep deliberation cycles, the size of these state machines (containing arrays of critiques, context patches, and history) grows rapidly. Storing these raw JSON blobs uncompressed spikes PostgreSQL cloud storage costs and bloats SQLite footprint in Standalone mode.

## Research Report
- Current `UltraPlanManager` marshals `stateMachine` directly to a byte slice and stores it in the `state_machine` JSONB/text column.
- Similar to the previously implemented PGCheckpointer and LLM caching, compressing this JSON payload via gzip and encoding it into a compact format will result in an estimated 80%+ reduction in byte footprint for repetitive deliberation loops.
- As the Principal Cost Engineer & Miser, this fulfills the proactive mandate to continuously optimize cloud resource management and LLM context/storage costs.

## Design Doc
1. **Compression Format**:
   - Utilize standard `compress/gzip` combined with Base64 encoding.
   - Wrap the compressed payload in a JSON struct: `{"_compressed_base64": "..."}`. This maintains database column constraints (JSONB validity) and allows seamless backward compatibility for older, uncompressed records.
2. **Implementation Integration**:
   - Add `compressUltraPlanData` and `decompressUltraPlanData` to `ultraplan.go`.
   - Update `CreatePlan`, `withTransaction` update blocks, and `UpdatePlanStatus` to compress the generated JSON before writing.
   - Update the internal `mapRowsToPlans` or retrieval queries to intercept the JSON payload, check for the `_compressed_base64` key, and decompress it before unmarshaling into the `StateMachine` map.

## Priority
P2

## Estimated Scope
Small
