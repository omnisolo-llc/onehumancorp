package queue

import (
	"context"
	"crypto/rand"
	"database/sql"
	"encoding/hex"
	"errors"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// SQLiteTaskQueue implements TaskQueue using the database (either SQLite or Postgres).
type SQLiteTaskQueue struct {
	db db.Provider
}

// ensureTable sets up the `sub_agent_jobs` table for SQLite queue execution.
func (q *SQLiteTaskQueue) ensureTable(ctx context.Context) error {
	// The problem statement requires TIMESTAMPTZ for the hybrid fallback queue schema.
	// We'll create the schema with TIMESTAMPTZ, but if it's SQLite, we create it as DATETIME
	// because SQLite doesn't natively parse TIMESTAMPTZ for comparisons in UPDATE subqueries.

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

	if !q.db.IsSQLite() {
		schema = `
		CREATE TABLE IF NOT EXISTS sub_agent_jobs (
			id TEXT PRIMARY KEY,
			parent_task_id TEXT,
			agent_role TEXT NOT NULL,
			payload JSONB NOT NULL,
			status TEXT NOT NULL DEFAULT 'QUEUED',
			attempts INTEGER DEFAULT 0,
			max_attempts INTEGER DEFAULT 3,
			run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			locked_until TIMESTAMPTZ,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		);
		CREATE INDEX IF NOT EXISTS idx_jobs_runnable ON sub_agent_jobs (status, run_after) WHERE status = 'QUEUED';
		`
	}

	_, err := q.db.Exec(ctx, schema)
	return err
}

func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	if err := q.ensureTable(ctx); err != nil {
		return fmt.Errorf("failed to ensure sub_agent_jobs table: %w", err)
	}

	if job.ID == "" {
		b := make([]byte, 16)
		_, _ = rand.Read(b)
		job.ID = hex.EncodeToString(b)
	}

	if job.MaxAttempts == 0 {
		job.MaxAttempts = 3
	}

	query := `
		INSERT INTO sub_agent_jobs (id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after)
		VALUES ($1, $2, $3, $4, 'QUEUED', 0, $5, CURRENT_TIMESTAMP)
	`
	_, err := q.db.Exec(ctx, query, job.ID, job.ParentTaskID, job.AgentRole, job.Payload, job.MaxAttempts)
	if err != nil {
		return fmt.Errorf("failed to enqueue job: %w", err)
	}

	telemetry.RecordQueueLength(ctx, "sub_agent_jobs", 1)
	return nil
}

func (q *SQLiteTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	if err := q.ensureTable(ctx); err != nil {
		return nil, fmt.Errorf("failed to ensure sub_agent_jobs table: %w", err)
	}

	if len(roles) == 0 {
		return nil, nil
	}

	tx, err := q.db.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	// Build placeholders for roles IN clause
	placeholders := make([]string, len(roles))
	args := make([]interface{}, len(roles))
	for i, role := range roles {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = role
	}
	rolesInClause := strings.Join(placeholders, ", ")

	var query string
	if q.db.IsSQLite() {
		query = fmt.Sprintf(`
			UPDATE sub_agent_jobs
			SET status = 'RUNNING', attempts = attempts + 1, locked_until = datetime(CURRENT_TIMESTAMP, '+5 minutes'), updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM sub_agent_jobs
				WHERE status = 'QUEUED' AND run_after <= CURRENT_TIMESTAMP AND agent_role IN (%s)
				ORDER BY run_after ASC LIMIT 1
			)
			RETURNING id, COALESCE(parent_task_id, ''), agent_role, payload, status, attempts, max_attempts
		`, rolesInClause)
	} else {
		query = fmt.Sprintf(`
			UPDATE sub_agent_jobs
			SET status = 'RUNNING', attempts = attempts + 1, locked_until = CURRENT_TIMESTAMP + interval '5 minutes', updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM sub_agent_jobs
				WHERE status = 'QUEUED' AND run_after <= CURRENT_TIMESTAMP AND agent_role IN (%s)
				ORDER BY run_after ASC LIMIT 1 FOR UPDATE SKIP LOCKED
			)
			RETURNING id, COALESCE(parent_task_id, ''), agent_role, payload, status, attempts, max_attempts
		`, rolesInClause)
	}

	job := &Job{}
	var parentTaskID sql.NullString

	err = tx.QueryRow(ctx, query, args...).Scan(
		&job.ID, &parentTaskID, &job.AgentRole, &job.Payload, &job.Status, &job.Attempts, &job.MaxAttempts,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No jobs found
		}
		return nil, fmt.Errorf("failed to dequeue job: %w", err)
	}

	if parentTaskID.Valid {
		job.ParentTaskID = parentTaskID.String
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	telemetry.RecordQueueLength(ctx, "sub_agent_jobs", -1)

	return job, nil
}

func (q *SQLiteTaskQueue) Complete(ctx context.Context, jobID string) error {
	query := `UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1`
	_, err := q.db.Exec(ctx, query, jobID)
	return err
}

func (q *SQLiteTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	query := `
		UPDATE sub_agent_jobs
		SET
			status = CASE WHEN attempts >= max_attempts THEN 'FAILED' ELSE 'QUEUED' END,
			run_after = CASE WHEN attempts < max_attempts THEN datetime(CURRENT_TIMESTAMP, '+1 minute') ELSE run_after END,
			updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`
	if !q.db.IsSQLite() {
		query = `
			UPDATE sub_agent_jobs
			SET
				status = CASE WHEN attempts >= max_attempts THEN 'FAILED' ELSE 'QUEUED' END,
				run_after = CASE WHEN attempts < max_attempts THEN CURRENT_TIMESTAMP + interval '1 minute' ELSE run_after END,
				updated_at = CURRENT_TIMESTAMP
			WHERE id = $1
		`
	}
	_, err := q.db.Exec(ctx, query, jobID)
	return err
}
