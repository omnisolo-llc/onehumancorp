# OHC Universal Core Design Protocols

1. **Skeptical Memory**: Verify state (`ls`, `grep`, `view_file`) BEFORE acting.
2. **Teammate Mesh (Mailbox)**: Coordinate via production Redis Pub/Sub channels. Check mailbox at start; post coordination sessions to teammates.
3. **Git-Lock Coordination**: Check production distributed Redis locks before modifying files. Wait if locked.
4. **Durable State**: Update production Vector DB (e.g. pgvector/Pinecone) with "AutoDream" architectural consolidation findings.
5. **KAIROS Orchestrator**: KAIROS coordinates the Hybrid Agentic OS via the Shared Task List and Realtime Teammate Mesh.
