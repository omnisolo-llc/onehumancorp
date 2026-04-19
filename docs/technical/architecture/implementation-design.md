<div markdown="1" style="backdrop-filter: blur(20px) saturate(200%); background: rgba(255, 255, 255, 0.03); font-family: 'Outfit', 'Inter', sans-serif;">

# Master Architecture Design: KAIROS AI OS Implementations

## 1. Shared Task List
- **Cloud-Native**: PostgreSQL `shared_tasks` table.
- **Standalone**: SQLite fallback.
- **Migrations**: Required in `srcs/server/db/migrations/`.

## 2. Teammate Mesh
- **Cloud-Native**: Redis Pub/Sub channels `mesh:tasks`, `mesh:coordination`.
- **Standalone**: Memory Pub/Sub fallback.

## 3. autoDream Pipelines
- **Cloud-Native**: pgvector/Pinecone for vector storage in `consolidated_memory`.
- **Worker**: `AutoDreamWorker` with proper auth extraction handling.

</div>