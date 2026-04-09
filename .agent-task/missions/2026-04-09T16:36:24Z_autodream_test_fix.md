---
status: PENDING
Title: Fix AutoDream Orchestration Tests Database Schema
Priority: P1
Estimated Scope: Small
---

# Problem Statement
The OHC `autodream_test.go` integration tests (`TestAutoDreamPruneSessions`, `TestAutoDreamTruthInjectionAndConflict`, and `TestAutoDreamConsolidateEpoch`) are currently failing in the `//srcs/server/orchestration:orchestration_test` suite due to missing database tables or columns (`shared_tasks`, `agent_session_data`, etc.). The test environment initializes an in-memory SQLite database but fails to apply the necessary migrations or schema creations to support KAIROS AutoDream components.

# Research Report
- Based on recent KAIROS Orchestration implementations, tests need a correct hybrid db state.
- The errors state: `SQL logic error: no such table: shared_tasks (1)` and `SQL logic error: no such table: agent_session_data (1)`.
- The `srcs/server/orchestration/autodream_test.go` uses `db.NewTestProvider` which creates a clean memory DB, but it appears the KAIROS schemas (like `013_shared_tasks.sql` or `024_autodream_memories.sql`) are either not applied or lack columns.
- The tests need proper setup blocks executing schema definitions matching the real environment.

# Design Doc
Update the test setup in `srcs/server/orchestration/autodream_test.go` to explicitly create the `shared_tasks`, `agent_session_data`, and `autodream_memories` tables before running the tests, or ensure the test `initDB` runs all necessary migrations.
Example snippet for table creation if not using migrations:
```sql
CREATE TABLE IF NOT EXISTS shared_tasks (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    parent_plan_id TEXT,
    title TEXT NOT NULL,
    description TEXT,
    status TEXT NOT NULL DEFAULT 'PENDING',
    agent_id TEXT,
    dependencies JSONB,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS agent_session_data (
    session_id TEXT PRIMARY KEY,
    agent_id TEXT NOT NULL,
    context_data TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
    last_accessed TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
CREATE TABLE IF NOT EXISTS autodream_memories (
    id TEXT PRIMARY KEY,
    organization_id TEXT NOT NULL,
    agent_id TEXT,
    content TEXT NOT NULL,
    embedding vector(1536),
    source_type TEXT NOT NULL,
    created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
);
```

# Implementation Prompt
You are an Implementer agent. Your task is to fix the failing KAIROS AutoDream orchestration tests.
1. Open `srcs/server/orchestration/autodream_test.go`.
2. Locate the setup functions for the failing tests (`TestAutoDreamPruneSessions`, `TestAutoDreamTruthInjectionAndConflict`, `TestAutoDreamConsolidateEpoch`).
3. Ensure the test database provider has the required `shared_tasks`, `agent_session_data`, and `autodream_memories` schemas properly applied using `provider.Exec` before test execution.
4. Verify tests pass by running `bazelisk test //srcs/server/orchestration/...`.
