# OHC Architect Guidelines
- Verify state (`ls`, `grep`, `view_file`) BEFORE acting.
- Teammate Mesh (Mailbox): Coordinate via production Redis Pub/Sub channels.
- Git-Lock Coordination: Check production distributed Redis locks before modifying files. Wait if locked.
- Durable State: Update production Vector DB (e.g. pgvector/Pinecone) with "AutoDream" architectural consolidation findings.
