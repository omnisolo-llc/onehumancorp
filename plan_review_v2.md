1. **Claim the mission**: Update the mission file with specific bash commands:
   `sed -i 's/agent: Researcher/agent: Link/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   `sed -i 's/status: PENDING/status: IN_PROGRESS/' .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md`
   Verify with `cat .agent-task/missions/2026-04-07T08-02-24Z_hybrid_mcp_rag_sync.md | head -n 5`.

2. **Database Migration**: Based on `cat srcs/server/db/migrations/005_sip.sql`, the target table is indeed `swarm_memory_embeddings`.
   Create `srcs/server/db/migrations/032_hybrid_rag_sync.sql` using:
   ```bash
   cat << 'EOF' > srcs/server/db/migrations/032_hybrid_rag_sync.sql
-- 032_hybrid_rag_sync.sql
ALTER TABLE swarm_memory_embeddings ADD COLUMN sync_status VARCHAR(50) DEFAULT 'pending';
ALTER TABLE swarm_memory_embeddings ADD COLUMN last_sync_at TIMESTAMP NULL;
