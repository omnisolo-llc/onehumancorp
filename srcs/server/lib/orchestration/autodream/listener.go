package autodream

import (
	"context"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type Listener struct {
	db db.Provider
}

func NewListener(provider db.Provider) *Listener {
	return &Listener{
		db: provider,
	}
}

// BatchCompletedTasks is a background listener that batches completed tasks into the pgvector pipeline
func (l *Listener) BatchCompletedTasks(ctx context.Context) error {
	// For now this is just a stub that queries for completed tasks
	// In a real implementation this would generate embeddings and insert into pgvector

	query := `
		SELECT id, title, description, status
		FROM shared_tasks_v2
		WHERE status = 'DONE'
		ORDER BY updated_at ASC
		LIMIT 100
	`

	rows, err := l.db.Query(ctx, query)
	if err != nil {
		return fmt.Errorf("failed to query completed tasks: %w", err)
	}
	defer rows.Close()

	for rows.Next() {
		var id, title, desc, status string
		if err := rows.Scan(&id, &title, &desc, &status); err != nil {
			continue
		}
		// Dummy operation
		_ = fmt.Sprintf("Processing completed task: %s", id)
	}

	return nil
}

// StartBackgroundListener starts the AutoDream listener in the background
func (l *Listener) StartBackgroundListener(ctx context.Context, interval time.Duration) {
	go func() {
		ticker := time.NewTicker(interval)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				_ = l.BatchCompletedTasks(ctx)
			}
		}
	}()
}
