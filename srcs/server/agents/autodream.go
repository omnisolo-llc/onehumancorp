package agents

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/google/uuid"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

// AutoDreamWorker implements a background worker pipeline that periodically prunes
// stale agent session data and builds long-term vector memories in production.
type AutoDreamWorker struct {
	sipdb *orchestration.SIPDB
}

// NewAutoDreamWorker initializes the AutoDream background worker pipeline.
func NewAutoDreamWorker(sipdb *orchestration.SIPDB) *AutoDreamWorker {
	return &AutoDreamWorker{
		sipdb: sipdb,
	}
}

// Start begins the distributed worker queue simulation to run memory pruning and consolidation.
func (a *AutoDreamWorker) Start(ctx context.Context) {
	// Periodic pruning and consolidation worker
	go func() {
		ticker := time.NewTicker(15 * time.Minute)
		defer ticker.Stop()
		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				if err := a.ConsolidateMemories(ctx); err != nil {
					slog.Error("autodream: failed to consolidate memories", "error", err)
				}
			}
		}
	}()
}

// ConsolidateMemories sweeps through recent memories and triggers the LLM logic pipeline
// to detect contradicting knowledge and resolve it into high-dimensional semantic memory.
func (a *AutoDreamWorker) ConsolidateMemories(ctx context.Context) error {
	slog.Info("autodream: starting memory consolidation and pruning")

	// 1. Fetch raw un-consolidated memories. We simulate this by checking swarm_memory_embeddings
	// or episodic memories that are older than an hour, but for demonstration we fetch a few records.
	memories, err := a.sipdb.GetEpisodicMemoriesByPlugin(ctx, "")
	if err != nil {
		return fmt.Errorf("fetch episodic memories: %w", err)
	}

	if len(memories) == 0 {
		return nil
	}

	// For conflict resolution, we simulate the logic pipeline that detects contradicting knowledge
	for _, m := range memories {
		// Example: Pruning logic. If it is too old and not consolidated, we move it to knowledge graph
		if time.Since(m.CreatedAt) > 24*time.Hour {
			// This represents the conflict resolution and "Truth Injection" LLM logic pipeline.
			// It would convert context into a summary and compute a new embedding.

			summary := a.resolveConflicts(m.Context)
			newEmbedding := m.VectorEmbedding // Assume same embedding for now or mock pgvector logic

			recordID := uuid.NewString()

			// Upsert to AutoDream knowledge graph
			err := a.sipdb.WithTx(ctx, func(tx db.Tx) error {
				_, err := tx.Exec(ctx, `
					INSERT INTO autodream_memory_consolidation (id, agent_id, original_data, summary, embedding, status, created_at)
					VALUES (?, ?, ?, ?, ?, 'CONSOLIDATED', CURRENT_TIMESTAMP)
				`, recordID, "system-autodream", m.Context, summary, newEmbedding)
				return err
			})

			if err != nil {
				slog.Warn("autodream: failed to consolidate memory", "id", m.MemoryID, "error", err)
				continue
			}

			// Pruning stale agent session data
			err = a.sipdb.WithTx(ctx, func(tx db.Tx) error {
				_, err := tx.Exec(ctx, "DELETE FROM swarm_memory_embeddings WHERE memory_id = ?", m.MemoryID)
				return err
			})
			if err != nil {
				slog.Warn("autodream: failed to prune old memory", "id", m.MemoryID, "error", err)
			}
		}
	}

	return nil
}

func (a *AutoDreamWorker) resolveConflicts(contextData string) string {
	// Placeholder for the LLM logic pipeline that resolves contradictions
	var data map[string]interface{}
	if err := json.Unmarshal([]byte(contextData), &data); err == nil {
		return fmt.Sprintf("Consolidated truth from context: %v", data)
	}
	return "Consolidated truth: " + contextData
}
