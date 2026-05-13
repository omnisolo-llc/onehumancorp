package queue

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"strings"

	"github.com/google/uuid"
	"onehumancorp/srcs/server/telemetry"
)

type SQLiteTaskQueue struct {
	db *sql.DB
}

func NewSQLiteTaskQueue(db *sql.DB) (*SQLiteTaskQueue, error) {
	q := &SQLiteTaskQueue{db: db}
	if err := q.InitSchema(); err != nil {
		return nil, err
	}
	return q, nil
}

func (q *SQLiteTaskQueue) InitSchema() error {
	schema := `
	CREATE TABLE IF NOT EXISTS sub_agent_jobs (
		id TEXT PRIMARY KEY,
		parent_task_id TEXT,
		agent_role TEXT NOT NULL,
		payload JSONB NOT NULL,
		status TEXT NOT NULL DEFAULT 'QUEUED',
		attempts INTEGER DEFAULT 0,
		max_attempts INTEGER DEFAULT 3,
		run_after DATETIME DEFAULT CURRENT_TIMESTAMP,
		locked_until DATETIME,
		created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
		updated_at DATETIME DEFAULT CURRENT_TIMESTAMP
	);
	CREATE INDEX IF NOT EXISTS idx_jobs_runnable ON sub_agent_jobs (status, run_after) WHERE status = 'QUEUED';
	`
	_, err := q.db.Exec(schema)
	return err
}

func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	if job.ID == "" {
		job.ID = uuid.New().String()
	}

	query := `
		INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, run_after)
		VALUES (?, ?, ?, ?, 'QUEUED', CURRENT_TIMESTAMP)
	`
	_, err := q.db.ExecContext(ctx, query, job.ID, job.ParentTaskID, job.AgentRole, job.Payload)
	if err == nil {
		_ = telemetry.RecordQueueLength(ctx, 1, "sqlite")
	}
	return err
}

func (q *SQLiteTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	if len(roles) == 0 {
		return nil, errors.New("no roles provided for dequeue")
	}

	tx, err := q.db.BeginTx(ctx, nil)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback()

	placeholders := make([]string, len(roles))
	args := make([]interface{}, len(roles))
	for i, r := range roles {
		placeholders[i] = "?"
		args[i] = r
	}

	selectQuery := fmt.Sprintf(`
		SELECT id, parent_task_id, agent_role, payload
		FROM sub_agent_jobs
		WHERE status = 'QUEUED'
		  AND agent_role IN (%s)
		  AND run_after <= CURRENT_TIMESTAMP
		ORDER BY created_at ASC
		LIMIT 1
	`, strings.Join(placeholders, ","))

	var job Job
	err = tx.QueryRowContext(ctx, selectQuery, args...).Scan(
		&job.ID,
		&job.ParentTaskID,
		&job.AgentRole,
		&job.Payload,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No jobs available
		}
		return nil, err
	}

	// Lock the job for 5 minutes
	updateQuery := `
		UPDATE sub_agent_jobs
		SET status = 'RUNNING',
		    locked_until = datetime('now', '+5 minutes'),
			attempts = attempts + 1,
			updated_at = CURRENT_TIMESTAMP
		WHERE id = ? AND status = 'QUEUED'
	`
	res, err := tx.ExecContext(ctx, updateQuery, job.ID)
	if err != nil {
		return nil, err
	}

	affected, err := res.RowsAffected()
	if err != nil || affected == 0 {
		return nil, nil // Lost the race
	}

	if err := tx.Commit(); err != nil {
		return nil, err
	}

	_ = telemetry.RecordQueueLength(ctx, -1, "sqlite")

	return &job, nil
}

func (q *SQLiteTaskQueue) Complete(ctx context.Context, jobID string) error {
	query := `
		UPDATE sub_agent_jobs
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = ? AND status = 'RUNNING'
	`
	_, err := q.db.ExecContext(ctx, query, jobID)
	return err
}

func (q *SQLiteTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	tx, err := q.db.BeginTx(ctx, nil)
	if err != nil {
		return err
	}
	defer tx.Rollback()

	var attempts, maxAttempts int
	err = tx.QueryRowContext(ctx, "SELECT attempts, max_attempts FROM sub_agent_jobs WHERE id = ?", jobID).Scan(&attempts, &maxAttempts)
	if err != nil {
		return err
	}

	if attempts >= maxAttempts {
		// Poison pill / Dead letter
		_, err = tx.ExecContext(ctx, `
			UPDATE sub_agent_jobs
			SET status = 'FAILED', payload = json_insert(payload, '$.error', ?), updated_at = CURRENT_TIMESTAMP
			WHERE id = ? AND status = 'RUNNING'
		`, reason, jobID)
	} else {
		// Retry with backoff
		backoffSeconds := attempts * attempts * 10
		_, err = tx.ExecContext(ctx, `
			UPDATE sub_agent_jobs
			SET status = 'QUEUED',
			    run_after = datetime('now', ? || ' seconds'),
				updated_at = CURRENT_TIMESTAMP
			WHERE id = ? AND status = 'RUNNING'
		`, fmt.Sprintf("+%d", backoffSeconds), jobID)

		if err == nil {
			_ = telemetry.RecordQueueLength(ctx, 1, "sqlite")
		}
	}

	if err != nil {
		return err
	}

	return tx.Commit()
}
