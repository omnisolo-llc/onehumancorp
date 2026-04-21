package workers

import (
	"context"
	"database/sql"
	"fmt"
	"log"
	"time"
)

type AutoDreamWorker struct {
	db *sql.DB
}

func NewAutoDreamWorker(db *sql.DB) *AutoDreamWorker {
	return &AutoDreamWorker{db: db}
}

func (w *AutoDreamWorker) Run(ctx context.Context) {
	ticker := time.NewTicker(1 * time.Minute)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.processTasks(ctx)
		}
	}
}

func (w *AutoDreamWorker) processTasks(ctx context.Context) {
	// Simple query to get completed tasks for vectorization
	rows, err := w.db.QueryContext(ctx, "SELECT id, title FROM shared_tasks WHERE status = 'COMPLETED'")
	if err != nil {
		log.Printf("autodream: failed to query tasks: %v", err)
		return
	}
	defer rows.Close()

	for rows.Next() {
		var id, title string
		if err := rows.Scan(&id, &title); err != nil {
			log.Printf("autodream: failed to scan task: %v", err)
			continue
		}

		// Stub out vector embedding logic
		log.Printf("autodream: stubbed vectorization for task %s (%s)", id, title)
		_ = w.stubVectorEmbeddings(id, title)
	}
}

func (w *AutoDreamWorker) stubVectorEmbeddings(id, title string) error {
	// Do not hardcode API keys. In real implementation, pass through config or use local stub.
	// We handle empty results gracefully by returning nil.
	if id == "" {
		return fmt.Errorf("id cannot be empty")
	}
	return nil
}
