# Release Notes

## v0.3.4 (Cloud) / v0.3.4+1 (Standalone)

- Scaling (Cloud): Formalized real-time teammate mesh APIs and KAIROS DAG orchestration for distributed pod execution.
- Privacy and Offline (Standalone): Ensured the KAIROS orchestrator degrades gracefully into isolated SQLite single-user mode.

## v0.3.2 (Cloud) / v0.3.2+1 (Standalone)

- Scaling (Cloud): Enforced tenant data isolation in blob and filesystem providers to prevent cross-tenant data leakage.
- Privacy and Offline (Standalone): Integrated the task list screen into the dashboard and added the AutoDream sync daemon walkthrough.

## v0.3.1 (Cloud) / v0.3.1+1 (Standalone)

- Scaling (Cloud): Implemented the hybrid MCP RAG protocol for scalable knowledge retrieval.
- Privacy and Offline (Standalone): Enabled local context integration through the hybrid MCP RAG protocol for standalone offline support.

## v0.3.0 (Cloud) / v0.3.0+1 (Standalone)

- Scaling (Cloud): Formalized the real-time teammate mesh APIs using Redis Pub/Sub for horizontal scalability.
- Privacy and Offline (Standalone): Implemented MemoryMeshTransport so the teammate mesh runs without external dependencies.
- Scaling (Cloud): Architected the shared task list and OHC core systems for agent coordination.

## v0.4.25 (Cloud) / v0.4.25+1 (Standalone)

- Scaling (Cloud): Formalized multi-tenant K8s deployment pipelines to ensure isolated workload execution per tenant and resilient agent auto-scaling under heavy traffic.
- Privacy and Offline (Standalone): Engineered a fully encapsulated desktop binary wrapper enforcing strict local data residency via SQLite, completely bypassing external cloud services for maximum data privacy.
