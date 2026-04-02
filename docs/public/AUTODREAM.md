<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); font-family: Outfit, Inter, sans-serif; border: 1px solid rgba(255, 255, 255, 0.1); padding: 20px; border-radius: 12px; background: rgba(255, 255, 255, 0.05);">

# 🧠 autoDream: Memory Consolidation Pipeline

**Version:** 1.0.0
**Target Audience:** Cognitive Architects & Swarm Engineers

## 1. Concept overview
Without the "autoDream" memory consolidation pipeline, the swarm forgets architectural insights and contextual knowledge across sessions, reducing long-term efficiency and autonomy. The autoDream engine is an asynchronous daemon that periodically sweeps completed `shared_tasks` and `swarm_memory` and securely consolidates them into long-term vector embeddings.

## 2. Technical Implementation Details
The autoDream engine interfaces directly with the OHC Central Database.

### Database Support:
- **Cloud-Native**: Native `pgvector` on PostgreSQL provides robust similarity search over 1536-dimensional vectors.
- **Standalone Mode**: Utilizes an equivalent SQLite mechanism for maintaining hybrid vector consistency on the desktop shell.

### Example Database Schema
```sql
CREATE TABLE IF NOT EXISTS autodream_memories (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    content TEXT NOT NULL,
    embedding vector(1536), -- Assumes pgvector for cloud
    source_mission_id TEXT,
    consolidated_at TIMESTAMPTZ DEFAULT NOW()
);
```

## 3. Workflow
1. The background cron process periodically sweeps for tasks marked as `COMPLETED`.
2. Text content and architectural artifacts from the task are passed to an LLM endpoint for embedding generation.
3. The resulting 1536-dimensional vectors are stored in `autodream_memories`.
4. During subsequent sessions, sub-agents can query the `autodream_memories` table to retrieve relevant architectural insights via cosine similarity.

---
*Powered by OHC-SIP (Swarm Intelligence Protocol)*
*Display settings: Premium Glassmorphism UI*

</div>