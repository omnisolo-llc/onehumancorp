package onboarding

import (
	"context"
	"database/sql"
	"log"
	"time"
)

type Worker struct {
	db *sql.DB
}

func NewWorker(db *sql.DB) *Worker {
	return &Worker{db: db}
}

func (w *Worker) Start(ctx context.Context) {
	go func() {
		ticker := time.NewTicker(2 * time.Second)
		defer ticker.Stop()

		for {
			select {
			case <-ctx.Done():
				return
			case <-ticker.C:
				w.processTasks(ctx)
			}
		}
	}()
}

func (w *Worker) processTasks(ctx context.Context) {
	tx, err := w.db.BeginTx(ctx, nil)
	if err != nil {
		log.Printf("Worker error starting tx: %v", err)
		return
	}
	defer tx.Rollback()

	// Try to claim a task related to Onboarding using SKIP LOCKED
	query := `
		SELECT id, title
		FROM shared_tasks
		WHERE status = 'PENDING' AND title = 'Generate Storefront'
		LIMIT 1
		FOR UPDATE SKIP LOCKED
	`
	var taskID, title string
	err = tx.QueryRowContext(ctx, query).Scan(&taskID, &title)
	if err == sql.ErrNoRows {
		// No pending tasks
		return
	} else if err != nil {
		log.Printf("Worker error claiming task: %v", err)
		return
	}

	// Mock the AI generation process
	time.Sleep(1 * time.Second)

	// Update status to COMPLETED
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`
	_, err = tx.ExecContext(ctx, updateQuery, taskID)
	if err != nil {
		log.Printf("Worker error updating task: %v", err)
		return
	}

	if err := tx.Commit(); err != nil {
		log.Printf("Worker error committing tx: %v", err)
		return
	}
	log.Printf("Worker successfully processed task: %s", title)
}
