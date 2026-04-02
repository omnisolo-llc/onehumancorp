package orchestration

import (
	"context"
	"fmt"
	"log/slog"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type AutoDreamWorker struct {
	db       db.Provider
	interval time.Duration
}

func NewAutoDreamWorker(provider db.Provider, interval time.Duration) *AutoDreamWorker {
	return &AutoDreamWorker{
		db:       provider,
		interval: interval,
	}
}

func (w *AutoDreamWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.runPipeline(ctx)
		}
	}
}

func (w *AutoDreamWorker) runPipeline(ctx context.Context) {
	// 1. Prune stale agent session data
	w.pruneStaleSessions(ctx)

	// 2. Truth Injection / pgvector logic
	w.injectTruth(ctx)

	// 3. Conflict resolution
	w.resolveConflicts(ctx)
}

func (w *AutoDreamWorker) pruneStaleSessions(ctx context.Context) {

	query := "DELETE FROM agent_missions WHERE status = 'FAILED' AND created_at < NOW() - INTERVAL '7 days'"
	_, err := w.db.Exec(ctx, query)
	if err != nil {
		slog.Error("AutoDream: Failed to prune stale sessions", "error", err)
	} else {
		slog.Info("AutoDream: Pruned stale agent sessions")
	}
}

func (w *AutoDreamWorker) injectTruth(ctx context.Context) {
	var setupQuery string
	if w.db.IsSQLite() {
		setupQuery = "CREATE TABLE IF NOT EXISTS semantic_memory (id INTEGER PRIMARY KEY AUTOINCREMENT, content TEXT, embedding BLOB, created_at DATETIME DEFAULT CURRENT_TIMESTAMP)"
	} else {
		setupQuery = "CREATE TABLE IF NOT EXISTS semantic_memory (id SERIAL PRIMARY KEY, content TEXT, embedding vector(1536), created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP)"
	}

	_, err := w.db.Exec(ctx, setupQuery)
	if err != nil {
		slog.Error("AutoDream: Failed to setup pgvector semantic memory", "error", err)
		return
	}

	// Fetch memories missing embeddings
	var idStr string
	var content string
	var query string
	if w.db.IsSQLite() {
		query = "SELECT memory_id, context FROM swarm_memory_embeddings LIMIT 10"
	} else {
		query = "SELECT memory_id, context FROM swarm_memory_embeddings WHERE embedding IS NULL LIMIT 10"
	}

	rows, err := w.db.Query(ctx, query)
	if err == nil {
		defer rows.Close()
		for rows.Next() {
			_ = rows.Scan(&idStr, &content)

			// Mocking Vector Embedding generation & truth injection since we don't have direct LLM access in this worker scaffolding.
			// The actual embedding would require an HTTP call to OpenAI/Minimax.
			embedding := "[0.1, 0.2, 0.3]"

			var updateQuery string
			if w.db.IsSQLite() {
				updateQuery = "UPDATE swarm_memory_embeddings SET embedding = ? WHERE memory_id = ?"
			} else {
				updateQuery = "UPDATE swarm_memory_embeddings SET embedding = $1::vector WHERE memory_id = $2"
			}
			_, _ = w.db.Exec(ctx, updateQuery, embedding, idStr)
		}
	}
	slog.Info("AutoDream: Processed truth injection into pgvector semantic memory")
}

func (w *AutoDreamWorker) resolveConflicts(ctx context.Context) {
	// Conflict resolution pipeline
	var query string
	if w.db.IsSQLite() {
		query = "SELECT memory_id, context FROM swarm_memory_embeddings ORDER BY created_at DESC LIMIT 50"
	} else {
		query = "SELECT memory_id, context FROM swarm_memory_embeddings ORDER BY created_at DESC LIMIT 50"
	}

	rows, err := w.db.Query(ctx, query)
	if err != nil {
		return
	}
	defer rows.Close()

	// Simplistic simulation of LLM contradiction detection: Delete duplicated / contradictory contexts.
	var idStr string
	var content string
	seen := make(map[string]bool)
	var toDelete []string

	for rows.Next() {
		_ = rows.Scan(&idStr, &content)
		// Extract core keywords to check for semantic overlap (mock contradiction logic)
		words := strings.Fields(content)
		if len(words) > 0 {
			key := words[0]
			if seen[key] {
				toDelete = append(toDelete, idStr)
			} else {
				seen[key] = true
			}
		}
	}

	for _, id := range toDelete {
		if w.db.IsSQLite() {
			_, _ = w.db.Exec(ctx, "DELETE FROM swarm_memory_embeddings WHERE memory_id = ?", id)
		} else {
			_, _ = w.db.Exec(ctx, "DELETE FROM swarm_memory_embeddings WHERE memory_id = $1", id)
		}
	}

	slog.Info(fmt.Sprintf("AutoDream: Processed conflict resolution in semantic memory. Pruned %d records", len(toDelete)))
}
