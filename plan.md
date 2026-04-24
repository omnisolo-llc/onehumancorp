1. **Design Document**:
   - Write a short architecture doc in `docs/technical/architecture/memory_consolidation.md` describing the design for the Persistent Memory Layer, Conflict Resolution, Stale Context Pruning, and Cross-Department Context Sharing. I will use `run_in_bash_session` to run a `cat` command to write this markdown document. I will use `read_file` to verify the creation of the document.
2. **Schema Modifications**:
   - Add new columns to `consolidated_memory` table (used for cross-department sharing and cross-session persistence). I will create two new Goose schema migrations (one for SQLite and one for Postgres) in `srcs/server/db/migrations/` using `run_in_bash_session` with a `cat` command:
     - `ALTER TABLE consolidated_memory ADD COLUMN last_accessed_at DATETIME/TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP;`
     - `ALTER TABLE consolidated_memory ADD COLUMN confidence_score FLOAT DEFAULT 1.0;`
   - Use `run_in_bash_session` to list the migrations directory to verify they are created.
3. **Memory Consolidator Implementation**:
   - I will update `EmbeddingRecord` and `VectorRepository` in `srcs/server/memory/vector_repository.go` to use `consolidated_memory` instead of `autodream_memories_master`. I will use `run_in_bash_session` with `sed` or standard python script to string-replace the file.
   - I will implement `SemanticSearch` in `srcs/server/memory/vector_repository.go` based on dialect (checking `!r.db.IsSQLite()`). If PG, use PG vector syntax (`ORDER BY embedding <-> $3`). If SQLite, fallback to basic retrieval or exact match.
   - I will implement `ResolveConflicts` in `srcs/server/memory/autodream/service.go`. It will retrieve similar facts, use LLM to summarize/resolve, and delete the obsolete ones while keeping the newest/highest-confidence fact. I will use `run_in_bash_session` with a python script to patch the file.
   - I will implement `PruneStaleContext` in `srcs/server/memory/vector_repository.go` to delete `consolidated_memory` records older than X days and with a `last_accessed_at` timestamp older than Y days.
   - I will use `read_file` on `srcs/server/memory/vector_repository.go` and `srcs/server/memory/autodream/service.go` to verify the code edits were applied accurately.
4. **Testing**:
   - Create `srcs/server/memory/vector_repository_test.go` and update `srcs/server/memory/autodream/service_test.go` using a Python script via `run_in_bash_session` to test `ResolveConflicts`, `PruneStaleContext`, and `SemanticSearch`.
   - Use `read_file` to verify changes in the test files.
   - Run `bazelisk test //...` using `run_in_bash_session` to ensure all tests pass (unit test coverage MUST be 100%).
5. **Pre-commit**:
   - Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
6. **Submit**:
   - Submit the change with branch name "feature/ai-memory-consolidation".
