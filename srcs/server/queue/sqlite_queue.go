package queue

import (
	"context"
	"database/sql"
	"fmt"

	"github.com/google/uuid"
	_ "github.com/mattn/go-sqlite3"
)

// SQLiteQueue implements the Queue interface for Standalone Mode
type SQLiteQueue struct {
	db *sql.DB
}

// NewSQLiteQueue creates a new SQLite-backed queue
func NewSQLiteQueue(dbPath string) (*SQLiteQueue, error) {
	db, err := sql.Open("sqlite3", dbPath)
	if err != nil {
		return nil, fmt.Errorf("open sqlite db: %w", err)
	}

	_, err = db.Exec(`
		CREATE TABLE IF NOT EXISTS local_queue_jobs (
			id TEXT PRIMARY KEY,
			task_id TEXT NOT NULL,
			role TEXT NOT NULL,
			payload BLOB,
			status TEXT DEFAULT 'PENDING'
		);
	`)
	if err != nil {
		db.Close()
		return nil, fmt.Errorf("create table: %w", err)
	}

	return &SQLiteQueue{db: db}, nil
}

// EnqueueSubAgent inserts a job into the SQLite table
func (q *SQLiteQueue) EnqueueSubAgent(ctx context.Context, taskID string, role string, payload []byte) error {
	id := uuid.New().String()
	_, err := q.db.ExecContext(ctx, "INSERT INTO local_queue_jobs (id, task_id, role, payload) VALUES (?, ?, ?, ?)", id, taskID, role, payload)
	if err != nil {
	    return fmt.Errorf("insert job: %w", err)
	}
	return nil
}

// ProcessSubAgentJob marks a job as processing/completed.
// A real worker would poll the db for 'PENDING' jobs.
func (q *SQLiteQueue) ProcessSubAgentJob(ctx context.Context, job *Job) error {
	_, err := q.db.ExecContext(ctx, "UPDATE local_queue_jobs SET status = 'COMPLETED' WHERE id = ?", job.ID)
	if err != nil {
	    return fmt.Errorf("update job status: %w", err)
	}
	return nil
}

func (q *SQLiteQueue) Close() error {
	return q.db.Close()
}
