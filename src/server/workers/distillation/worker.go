package distillation

import (
	"context"
	"encoding/json"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/src/server/checkpointer"
	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory/autodream"
)

// SemanticDistillationWorker distills stale checkpoints into durable vector summaries
type SemanticDistillationWorker struct {
	provider     db.Provider
	checkpointer checkpointer.CheckpointSaver
	autodream    autodream.MemoryConsolidator
}

// NewSemanticDistillationWorker creates a new instance
func NewSemanticDistillationWorker(provider db.Provider, cp checkpointer.CheckpointSaver, ad autodream.MemoryConsolidator) *SemanticDistillationWorker {
	return &SemanticDistillationWorker{
		provider:     provider,
		checkpointer: cp,
		autodream:    ad,
	}
}

// ProcessStaleThreads queries the database for inactive threads and distills them
func (w *SemanticDistillationWorker) ProcessStaleThreads(ctx context.Context) error {
	slog.Info("DistillationWorker: Scanning for stale threads to distill")

	// Find threads that haven't been updated in the last 24 hours
	query := `
		SELECT DISTINCT thread_id
		FROM swarm_checkpoints
		WHERE created_at < $1
	`

	// We'll use 24 hours as the threshold for a "stale" thread
	staleThreshold := time.Now().Add(-24 * time.Hour)

	rows, err := w.provider.Query(ctx, query, staleThreshold)
	if err != nil {
		return fmt.Errorf("failed to query stale threads: %w", err)
	}
	defer rows.Close()

	var staleThreads []string
	for rows.Next() {
		var threadID string
		if err := rows.Scan(&threadID); err != nil {
			slog.Error("DistillationWorker: failed to scan thread_id", "error", err)
			continue
		}
		staleThreads = append(staleThreads, threadID)
	}

	if err := rows.Err(); err != nil {
		return fmt.Errorf("error iterating stale threads: %w", err)
	}

	for _, threadID := range staleThreads {
		if err := w.DistillThread(ctx, threadID); err != nil {
			slog.Error("DistillationWorker: failed to distill thread", "threadID", threadID, "error", err)
		} else {
			// After distillation, we could delete the stale checkpoints, but for now we just distill
			slog.Info("DistillationWorker: successfully distilled thread", "threadID", threadID)
		}
	}

	return nil
}

// DistillThread analyzes checkpoints for a thread and creates a distilled vector memory summary
func (w *SemanticDistillationWorker) DistillThread(ctx context.Context, threadID string) error {
	slog.Info("DistillationWorker: Distilling thread", "threadID", threadID)

	checkpoints, err := w.checkpointer.ListCheckpoints(ctx, threadID)
	if err != nil {
		return fmt.Errorf("failed to list checkpoints for thread %s: %w", threadID, err)
	}

	if len(checkpoints) == 0 {
		return nil
	}

	var logs []string
	for _, cp := range checkpoints {
		data, err := json.Marshal(cp.Data)
		if err == nil {
			logs = append(logs, string(data))
		}
	}

	if w.autodream != nil {
		err = w.autodream.Consolidate(ctx, threadID, logs)
		if err != nil {
			return fmt.Errorf("failed to consolidate logs for thread %s: %w", threadID, err)
		}
	}

	slog.Info("DistillationWorker: Successfully distilled thread", "threadID", threadID, "checkpoints", len(checkpoints))
	return nil
}
