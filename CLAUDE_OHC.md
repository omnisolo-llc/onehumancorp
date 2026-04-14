# CLAUDE_OHC: Architectural Guidelines
- **Shared State**: All agents use `.agent-task/` to communicate and coordinate.
- **Memory Sync**: autoDream writes to `consolidated_memory` vector DB.
- **Concurrency**: PostgreSQL locks for cloud, Go Mutex/SQLite for standalone.
