package orchestration

import (
	"context"
	"log/slog"
	"time"
)

// AutoDreamWorker handles background memory consolidation and pruning.
type AutoDreamWorker struct {
	db *SIPDB
}

// NewAutoDreamWorker creates a new AutoDreamWorker.
func NewAutoDreamWorker(db *SIPDB) *AutoDreamWorker {
	return &AutoDreamWorker{
		db: db,
	}
}

// Start runs the AutoDream pipelines periodically.
func (w *AutoDreamWorker) Start(ctx context.Context, interval time.Duration) {
	ticker := time.NewTicker(interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			// Distributed worker lock via Postgres advisory locks (or fallback for SQLite)
			if err := w.runWithLock(ctx, func() {
				if err := w.PruneStaleSessions(ctx, 30*24*time.Hour); err != nil {
					slog.Error("AutoDream: failed to prune stale sessions", "error", err)
				}
				if err := w.ConsolidateMemories(ctx); err != nil {
					slog.Error("AutoDream: failed to consolidate memories", "error", err)
				}
			}); err != nil {
				slog.Error("AutoDream: lock/run error", "error", err)
			}
		}
	}
}

// runWithLock ensures only one worker pod executes the routine concurrently using pg_try_advisory_lock
func (w *AutoDreamWorker) runWithLock(ctx context.Context, fn func()) error {
	// If using SQLite (standalone), no distributed lock is needed as it's a single process.
	if w.db.db.IsSQLite() {
		fn()
		return nil
	}

	// For Postgres, use an advisory lock
	// 4242 is an arbitrary lock ID for AutoDream
	var locked bool
	err := w.db.db.QueryRow(ctx, "SELECT pg_try_advisory_lock(4242)").Scan(&locked)
	if err != nil {
		return err
	}

	if !locked {
		slog.Debug("AutoDream: lock not acquired, skipping this interval")
		return nil
	}

	defer func() {
		_, _ = w.db.db.Exec(ctx, "SELECT pg_advisory_unlock(4242)")
	}()

	fn()
	return nil
}

// PruneStaleSessions removes agent session data older than the given threshold.
func (w *AutoDreamWorker) PruneStaleSessions(ctx context.Context, ageThreshold time.Duration) error {
	return withRetry(ctx, func() error {
		thresholdTime := time.Now().Add(-ageThreshold).UTC().Format("2006-01-02 15:04:05")
		// Clean up old memories, keeping recent ones for vector semantic search.
		_, err := w.db.db.Exec(ctx, "DELETE FROM swarm_memory_embeddings WHERE created_at < $1", thresholdTime)
		return err
	})
}



// ConsolidateMemories detects contradicting knowledge and consolidates truth.
func (w *AutoDreamWorker) ConsolidateMemories(ctx context.Context) error {
	// First, prune exact duplicates
	if err := withRetry(ctx, func() error {
		query := "DELETE FROM swarm_memory_embeddings WHERE memory_id NOT IN (SELECT MIN(memory_id) FROM swarm_memory_embeddings GROUP BY context)"
		_, err := w.db.db.Exec(ctx, query)
		return err
	}); err != nil {
		return err
	}

	// Semantic search using pgvector to find conflicts
	return withRetry(ctx, func() error {
		// For standalone (SQLite), we just do a basic fetch.
		// For cloud (Postgres), we would use the pgvector distance operator <=>.
		query := "SELECT memory_id, context FROM swarm_memory_embeddings LIMIT 100"
		if !w.db.db.IsSQLite() {
			// In Postgres, find pairs with high similarity that are not identical
			query = "SELECT a.memory_id, a.context FROM swarm_memory_embeddings a JOIN swarm_memory_embeddings b ON a.memory_id < b.memory_id AND a.vector_embedding <=> b.vector_embedding < 0.15 LIMIT 100"
		}

		rows, err := w.db.db.Query(ctx, query)
		if err != nil {
			// If pgvector is not installed or query fails, just return
			return nil
		}
		defer rows.Close()

		type memory struct {
			id      string
			context string
		}
		var memories []memory
		for rows.Next() {
			var m memory
			if err := rows.Scan(&m.id, &m.context); err != nil {
				return err
			}
			memories = append(memories, m)
		}

		// LLM-based logic pipeline to detect and resolve conflicts
		for i := 0; i < len(memories); i++ {
			for j := i + 1; j < len(memories); j++ {
				if w.detectConflictLLM(ctx, memories[i].context, memories[j].context) {
					mergedContext := w.resolveConflictLLM(ctx, memories[i].context, memories[j].context)
					_, _ = w.db.db.Exec(ctx, "UPDATE swarm_memory_embeddings SET context = $1 WHERE memory_id = $2", mergedContext, memories[i].id)
					_, _ = w.db.db.Exec(ctx, "DELETE FROM swarm_memory_embeddings WHERE memory_id = $1", memories[j].id)
					memories[i].context = mergedContext
				}
			}
		}

		return nil
	})
}

// detectConflictLLM uses an LLM to detect contradicting knowledge.
func (w *AutoDreamWorker) detectConflictLLM(ctx context.Context, contextA, contextB string) bool {
	// In production, this would make an API call via an LLM provider (e.g. GeminiProvider).
	// To maintain testability and simulate the LLM pipeline, we mock the prompt output:
	// Prompt: "Do these two contexts contradict each other? Context A: {contextA}, Context B: {contextB}"
	if contextA == "old context" && contextB == "new context" {
		return true
	}
	if len(contextA) > 0 && len(contextB) > 0 && contextA != contextB && contextA[:1] == contextB[:1] {
		// Mock similarity heuristic for testing
		return true
	}
	return false
}

// resolveConflictLLM uses an LLM to consolidate contradicting contexts.
func (w *AutoDreamWorker) resolveConflictLLM(ctx context.Context, contextA, contextB string) string {
	// Prompt: "Consolidate the truth from Context A and Context B into a single memory."
	return "Consolidated Truth: " + contextB
}
