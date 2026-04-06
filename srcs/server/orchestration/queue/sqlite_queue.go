package queue

import (
	"context"
	"database/sql"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SQLiteTaskQueue struct {
	db db.Provider
}

func NewSQLiteTaskQueue(provider db.Provider) *SQLiteTaskQueue {
	return &SQLiteTaskQueue{db: provider}
}

func (q *SQLiteTaskQueue) ensureTable(ctx context.Context) error {
	query := `
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
		)
	`
	_, err := q.db.Exec(ctx, query)
	if err == nil {
		_, _ = q.db.Exec(ctx, "CREATE INDEX IF NOT EXISTS idx_jobs_runnable ON sub_agent_jobs (status, run_after) WHERE status = 'QUEUED'")
	}
	return err
}

func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	if err := q.ensureTable(ctx); err != nil {
		return fmt.Errorf("failed to ensure table: %w", err)
	}

	query := `
		INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, max_attempts)
		VALUES ($1, $2, $3, $4, 'QUEUED', $5)
	`
	_, err := q.db.Exec(ctx, query, job.ID, job.ParentTaskID, job.AgentRole, job.Payload, job.MaxAttempts)
	if err != nil {
		return fmt.Errorf("failed to enqueue job: %w", err)
	}

	telemetry.RecordTaskQueueLength(ctx, 1)
	return nil
}

func (q *SQLiteTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	if err := q.ensureTable(ctx); err != nil {
		return nil, fmt.Errorf("failed to ensure table: %w", err)
	}

	tx, err := q.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var query string
	var args []interface{}

	rolesCondition := ""
	if len(roles) > 0 {
		rolesCondition = "AND agent_role IN ("
		for i, role := range roles {
			if i > 0 {
				rolesCondition += ", "
			}
			rolesCondition += "?"
			args = append(args, role)
			_ = role
		}
		rolesCondition += ")"
	}

	if q.db.IsSQLite() {
		selectQuery := fmt.Sprintf(`
			SELECT id
			FROM sub_agent_jobs
			WHERE status = 'QUEUED' AND run_after <= CURRENT_TIMESTAMP %s
			ORDER BY created_at ASC
			LIMIT 1
		`, rolesCondition)

		var id string
		err := tx.QueryRow(ctx, selectQuery, args...).Scan(&id)
		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, fmt.Errorf("failed to find job: %w", err)
		}

		updateQuery := `
			UPDATE sub_agent_jobs
			SET status = 'RUNNING', attempts = attempts + 1, updated_at = CURRENT_TIMESTAMP
			WHERE id = ?
			RETURNING id, parent_task_id, agent_role, payload, attempts, max_attempts
		`
		job := &Job{}
		err = tx.QueryRow(ctx, updateQuery, id).Scan(&job.ID, &job.ParentTaskID, &job.AgentRole, &job.Payload, &job.Attempts, &job.MaxAttempts)
		if err != nil {
			return nil, fmt.Errorf("failed to claim job: %w", err)
		}

		if err := tx.Commit(ctx); err != nil {
			return nil, fmt.Errorf("failed to commit transaction: %w", err)
		}

		telemetry.RecordTaskQueueLength(ctx, -1)
		return job, nil
	}

	// Postgres with SKIP LOCKED
	rolesConditionPostgres := ""
	if len(roles) > 0 {
		rolesConditionPostgres = "AND agent_role IN ("
		for i, role := range roles {
			if i > 0 {
				rolesConditionPostgres += ", "
			}
			rolesConditionPostgres += fmt.Sprintf("$%d", i+1)
			_ = role
		}
		rolesConditionPostgres += ")"
	}

	query = fmt.Sprintf(`
		UPDATE sub_agent_jobs
		SET status = 'RUNNING', attempts = attempts + 1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM sub_agent_jobs
			WHERE status = 'QUEUED' AND run_after <= CURRENT_TIMESTAMP %s
			ORDER BY created_at ASC
			LIMIT 1 FOR UPDATE SKIP LOCKED
		)
		RETURNING id, parent_task_id, agent_role, payload, attempts, max_attempts
	`, rolesConditionPostgres)

	job := &Job{}
	err = tx.QueryRow(ctx, query, args...).Scan(&job.ID, &job.ParentTaskID, &job.AgentRole, &job.Payload, &job.Attempts, &job.MaxAttempts)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to dequeue job: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	telemetry.RecordTaskQueueLength(ctx, -1)
	return job, nil
}

func (q *SQLiteTaskQueue) Complete(ctx context.Context, jobID string) error {
	query := `UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1`
	_, err := q.db.Exec(ctx, query, jobID)
	return err
}

func (q *SQLiteTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	tx, err := q.db.Begin(ctx)
	if err != nil {
		return err
	}
	defer tx.Rollback(ctx)

	var attempts, maxAttempts int
	err = tx.QueryRow(ctx, "SELECT attempts, max_attempts FROM sub_agent_jobs WHERE id = $1", jobID).Scan(&attempts, &maxAttempts)
	if err != nil {
		return err
	}

	status := "QUEUED"
	if attempts >= maxAttempts {
		status = "FAILED"
	}

	// Simple exponential backoff: 5s * 2^attempts
	delay := time.Duration(5*(1<<uint(attempts))) * time.Second
	runAfter := time.Now().Add(delay)

	query := `
		UPDATE sub_agent_jobs
		SET status = $1, run_after = $2, updated_at = CURRENT_TIMESTAMP
		WHERE id = $3
	`
	_, err = tx.Exec(ctx, query, status, runAfter, jobID)
	if err != nil {
		return err
	}

	if status == "QUEUED" {
		telemetry.RecordTaskQueueLength(ctx, 1)
	}

	return tx.Commit(ctx)
}
