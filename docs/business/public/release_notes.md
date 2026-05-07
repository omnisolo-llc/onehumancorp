# Release Notes

## v0.4.30 (Cloud) / v0.4.30+1 (Standalone)

### Cloud Scaling Improvements
- 🗺️ Guide: Animated Confetti on Go-Live (#11420)

### Privacy/Offline Improvements
- 🗺️ Guide: Animated Confetti on Go-Live (#11420)

## v0.4.29 (Cloud) / v0.4.29+1 (Standalone)

### Cloud Scaling Improvements
- 🔗 Link: Implemented Teammate Mesh Communication Layer and Distributed Locks (#11313)

### Privacy/Offline Improvements
- 🔗 Link: Ensured mesh communication layer degrades gracefully into isolated standalone instances (#11313)


## v0.4.28 (Cloud) / v0.4.28+1 (Standalone)
### Cloud Scaling Improvements
- ✍️ Scribe: Scaled the Help Center & Tooltip Documentation System for multi-tenant cloud environments (#11267)

### Privacy/Offline Improvements
- ✍️ Scribe: Enabled offline-first support for the Help Center & Tooltip Documentation System in standalone mode (#11267)

## v0.4.27 (Cloud) / v0.4.27+1 (Standalone)

- Scaling (Cloud): 🎨 Canvas: Refactored the MCP LocalProxyClient to use an abstract BlobProvider with S3 support for cloud multitenant scaling.
- Privacy and Offline (Standalone): 🎨 Canvas: Added LocalBlobProvider implementation to ensure privacy and offline capabilities for the MCP proxy.

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
