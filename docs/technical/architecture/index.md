# Architecture Documentation

The architecture section is the entry point for system-level design, runtime topology, and shared concepts.

## Primary References

- [System Design](system-design.md)
- [System Overview](system-overview.md)
- [Hybrid OS Design](kairos/hybrid-os-design.md)

## Scope

Use this section for the durable description of the platform: runtime modes, orchestration, storage, trust boundaries, and cross-service contracts.

## Contents

### Core Architecture
- [System Overview](system-overview.md) - High-level system architecture
- [System Design](system-design.md) - Detailed system design
- [Architecture V2](architecture-v2.md) - Architecture version 2

### Orchestration
- [Features Design](orchestration/features-design.md)
- [Hybrid Architecture](orchestration/hybrid-arch-decomposition.md)
- [Core Design](orchestration/hybrid-core-design.md)
- [Master Plan](orchestration/master-plan.md)
- [Implementation Blueprint](orchestration/implementation-blueprint.md)

### KAIROS (Orchestration Engine)
- [Core Features](kairos/core-features.md)
- [Master Orchestration](kairos/master-orchestration.md)
- [Hybrid OS Design](kairos/hybrid-os-design.md)
- [Implementation Guide](kairos/implementation-guide.md)
- [Shared Task Mesh](kairos/shared-task-mesh.md)
- [Sub-Agent Queue](kairos/sub-agent-queue.md)

### RAG & Search
- [RAG Synthesis](rag/synthesis.md)
- [Oracle Hybrid RAG Master Plan](rag/oracle-hybrid-rag-master-plan.md)

### Reliability
- [Chaos Engineering](reliability/chaos-engineering.md)

### Security
- [SIP Compliance Design](security/sip-compliance-design.md)

### Sync
- [CRDT Sync Blueprint](sync/crdt-sync-blueprint.md)

### Research
- [Agent Harness Class](research/agent-harness-class.md)
- [Harness Research](research/harness-research.md)

### Implementation Guides
- [Hybrid Implementation Plan](implementation-guides/hybrid-implementation-plan.md)

### Competitive Analysis
- [Competitive Analysis](competitive-analysis.md)

## Legacy

Legacy one-off design explorations are stored in `archive/`.
