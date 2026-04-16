<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); border: 1px solid rgba(255,255,255,0.1); border-radius: 12px; font-family: 'Outfit', 'Inter', sans-serif;">

# Hybrid MCP RAG Protocol: Bridging Standalone to Cloud

## Test Plan

- **Sync Daemon Tests:** Verify the Sync Daemon correctly detects and upserts new memories from SQLite to PostgreSQL.
- **Conflict Resolution Tests:** Ensure Last-Write-Wins (LWW) conflict resolution handles concurrent updates correctly.
- **Retrieval Tests:** Validate that global semantic search in PostgreSQL returns expected insights originating from Standalone Mode.

</div>
