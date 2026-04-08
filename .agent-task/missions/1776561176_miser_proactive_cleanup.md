---
status: DONE
agent: Miser
---
# Title: 💰 Miser: [new cost feature] Proactive Auto-Eviction of Embedding Cache

## Problem Statement
The local SQLite `embedding_cache` and `llm_reason_cache` tables are growing unbounded because there is no eviction policy in place. In Standalone Mode, this unbounded growth leads to excessive disk usage and slower SQLite queries, contrary to our "cost engineering" mandate. While Redis handles TTL automatically via `EX`, SQLite does not.

## Research Report
The `CachedMinimaxClient` saves to SQLite when `db.Provider` is set, but never deletes old entries. We should create a background eviction mechanism or an active eviction check within the pipeline to remove rows older than a certain duration (e.g., 30 days) to prevent disk bloat. Since I am the Miser agent and all my missions are completed, I will proactively implement this.

## Design Doc
1.  **Add Timestamp Column**:
    - Update the DB schema or ensure there's a `created_at` timestamp in `embedding_cache` and `llm_reason_cache`. (I will check if they exist).
2.  **Eviction Logic**:
    - Add an `EvictOldCaches(ctx context.Context, olderThan time.Duration)` method to the `CachedMinimaxClient` or as a standalone function in `orchestration/cached_minimax_client.go` that runs `DELETE FROM ... WHERE created_at < ...`.
    - If `created_at` doesn't exist, I will create a migration to add it and set default `CURRENT_TIMESTAMP`.

## Implementation Prompt
1. Check `srcs/server/db/migrations/` for `embedding_cache` schema.
2. If `created_at` doesn't exist, create a new migration.
3. Add `EvictOldCaches` to `CachedMinimaxClient`.

## Priority
P2
## Estimated Scope
Small
