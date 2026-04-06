package queue

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

// SQLiteTaskQueue implements TaskQueue using SQLite (or generic SQL via db.Provider).
type SQLiteTaskQueue struct {
	db db.Provider
}

// NewSQLiteTaskQueue creates a new SQLiteTaskQueue.
func NewSQLiteTaskQueue(provider db.Provider) *SQLiteTaskQueue {
	return &SQLiteTaskQueue{db: provider}
}

// Enqueue adds a new job to the database.
func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	query := `
		INSERT INTO sub_agent_jobs (
			id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, created_at, updated_at
		) VALUES ($1, $2, $3, $4, 'QUEUED', $5, $6, COALESCE($7, CURRENT_TIMESTAMP), CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
	`

	var runAfter interface{}
	if !job.RunAfter.IsZero() {
		runAfter = job.RunAfter
	}

	_, err := q.db.Exec(ctx, query, job.ID, job.ParentTaskID, job.AgentRole, job.Payload, job.Attempts, job.MaxAttempts, runAfter)
	if err != nil {
		return fmt.Errorf("failed to enqueue job: %w", err)
	}

	telemetry.RecordTaskQueueLength(ctx, 1)
	return nil
}

// Dequeue attempts to fetch and lock an available job.
func (q *SQLiteTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	tx, err := q.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var jobID string
	var query string
	var args []interface{}

	if q.db.IsSQLite() {
		// SQLite logic: SELECT then UPDATE
		query = `
			SELECT id
			FROM sub_agent_jobs
			WHERE status = 'QUEUED'
			  AND run_after <= CURRENT_TIMESTAMP
			  AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
		`

		if len(roles) > 0 {
			placeholders := make([]string, len(roles))
			for i, role := range roles {
				placeholders[i] = fmt.Sprintf("$%d", i+1)
				args = append(args, role)
			}
			query += fmt.Sprintf(" AND agent_role IN (%s)", strings.Join(placeholders, ", "))
		}

		query += " ORDER BY run_after ASC, created_at ASC LIMIT 1"

		err = tx.QueryRow(ctx, query, args...).Scan(&jobID)
		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return nil, nil // No jobs available
			}
			return nil, fmt.Errorf("failed to find runnable job: %w", err)
		}

		// Lock the job for 5 minutes
		updateQuery := `
			UPDATE sub_agent_jobs
			SET status = 'RUNNING', locked_until = datetime(CURRENT_TIMESTAMP, '+5 minutes'), updated_at = CURRENT_TIMESTAMP
			WHERE id = $1 AND status = 'QUEUED'
			RETURNING id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at
		`
		job := &Job{}
		var lockedUntil sql.NullTime
		err = tx.QueryRow(ctx, updateQuery, jobID).Scan(
			&job.ID, &job.ParentTaskID, &job.AgentRole, &job.Payload, &job.Status,
			&job.Attempts, &job.MaxAttempts, &job.RunAfter, &lockedUntil,
			&job.CreatedAt, &job.UpdatedAt,
		)
		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return nil, nil // Another worker grabbed it
			}
			return nil, fmt.Errorf("failed to lock job: %w", err)
		}

		if lockedUntil.Valid {
			job.LockedUntil = &lockedUntil.Time
		}

		if err := tx.Commit(ctx); err != nil {
			return nil, fmt.Errorf("failed to commit dequeue: %w", err)
		}

		telemetry.RecordTaskQueueLength(ctx, -1)
		return job, nil
	}

	// PostgreSQL logic: FOR UPDATE SKIP LOCKED
	query = `
		UPDATE sub_agent_jobs
		SET status = 'RUNNING', locked_until = CURRENT_TIMESTAMP + interval '5 minutes', updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id
			FROM sub_agent_jobs
			WHERE status = 'QUEUED'
			  AND run_after <= CURRENT_TIMESTAMP
			  AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
	`

	if len(roles) > 0 {
		placeholders := make([]string, len(roles))
		for i, role := range roles {
			placeholders[i] = fmt.Sprintf("$%d", i+1)
			args = append(args, role)
		}
		query += fmt.Sprintf(" AND agent_role IN (%s)", strings.Join(placeholders, ", "))
	}

	query += `
			ORDER BY run_after ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		)
		RETURNING id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at
	`

	job := &Job{}
	var lockedUntil sql.NullTime
	err = tx.QueryRow(ctx, query, args...).Scan(
		&job.ID, &job.ParentTaskID, &job.AgentRole, &job.Payload, &job.Status,
		&job.Attempts, &job.MaxAttempts, &job.RunAfter, &lockedUntil,
		&job.CreatedAt, &job.UpdatedAt,
	)

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, nil // No jobs available
		}
		return nil, fmt.Errorf("failed to dequeue job: %w", err)
	}

	if lockedUntil.Valid {
		job.LockedUntil = &lockedUntil.Time
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit dequeue: %w", err)
	}

	telemetry.RecordTaskQueueLength(ctx, -1)
	return job, nil
}

// Complete marks a job as COMPLETED.
func (q *SQLiteTaskQueue) Complete(ctx context.Context, jobID string) error {
	query := `
		UPDATE sub_agent_jobs
		SET status = 'COMPLETED', locked_until = NULL, updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`
	_, err := q.db.Exec(ctx, query, jobID)
	if err != nil {
		return fmt.Errorf("failed to complete job: %w", err)
	}
	return nil
}

// Fail marks a job as FAILED or requeues it if max attempts are not reached.
func (q *SQLiteTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	tx, err := q.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var attempts, maxAttempts int
	err = tx.QueryRow(ctx, "SELECT attempts, max_attempts FROM sub_agent_jobs WHERE id = $1", jobID).Scan(&attempts, &maxAttempts)
	if err != nil {
		return fmt.Errorf("failed to find job: %w", err)
	}

	attempts++

	if attempts >= maxAttempts {
		// Permanently fail
		_, err = tx.Exec(ctx, "UPDATE sub_agent_jobs SET status = 'FAILED', attempts = $1, locked_until = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = $2", attempts, jobID)
	} else {
		// Requeue with backoff (e.g., 1 minute * attempt)
		backoff := time.Duration(attempts) * time.Minute
		if q.db.IsSQLite() {
			_, err = tx.Exec(ctx, fmt.Sprintf("UPDATE sub_agent_jobs SET status = 'QUEUED', attempts = $1, run_after = datetime(CURRENT_TIMESTAMP, '+%d seconds'), locked_until = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = $2", int(backoff.Seconds())), attempts, jobID)
		} else {
			_, err = tx.Exec(ctx, fmt.Sprintf("UPDATE sub_agent_jobs SET status = 'QUEUED', attempts = $1, run_after = CURRENT_TIMESTAMP + interval '%d seconds', locked_until = NULL, updated_at = CURRENT_TIMESTAMP WHERE id = $2", int(backoff.Seconds())), attempts, jobID)
		}
		// If requeued, increment length since we decremented it on dequeue
		telemetry.RecordTaskQueueLength(ctx, 1)
	}

	if err != nil {
		return fmt.Errorf("failed to fail job: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit fail: %w", err)
	}

	return nil
}
