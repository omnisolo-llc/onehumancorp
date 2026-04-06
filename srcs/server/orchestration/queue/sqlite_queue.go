package queue

import (
	"context"
	"database/sql"
	"fmt"
	"strings"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SQLiteTaskQueue struct {
	db db.Provider
	mu sync.Mutex
}

func NewSQLiteTaskQueue(provider db.Provider) *SQLiteTaskQueue {
	return &SQLiteTaskQueue{
		db: provider,
	}
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
			run_after TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			locked_until TIMESTAMPTZ,
			created_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP,
			updated_at TIMESTAMPTZ DEFAULT CURRENT_TIMESTAMP
		);
		CREATE INDEX IF NOT EXISTS idx_jobs_runnable ON sub_agent_jobs (status, run_after) WHERE status = 'QUEUED';
	`
	// Simple split by ; for SQLite since it might not support multi-statement properly in Exec
	parts := strings.Split(query, ";")
	for _, part := range parts {
		part = strings.TrimSpace(part)
		if part == "" {
			continue
		}
		_, err := q.db.Exec(ctx, part)
		if err != nil {
			return err
		}
	}
	return nil
}

func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	if err := q.ensureTable(ctx); err != nil {
		return err
	}

	if job.Status == "" {
		job.Status = "QUEUED"
	}
	if job.RunAfter.IsZero() {
		job.RunAfter = time.Now().UTC()
	}

	query := `
		INSERT INTO sub_agent_jobs (
			id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, created_at, updated_at
		) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10)
	`
	_, err := q.db.Exec(ctx, query,
		job.ID,
		job.ParentTaskID,
		job.AgentRole,
		job.Payload,
		job.Status,
		job.Attempts,
		job.MaxAttempts,
		job.RunAfter,
		time.Now().UTC().Format(time.RFC3339),
		time.Now().UTC().Format(time.RFC3339),
	)

	if err == nil {
		telemetry.RecordTaskQueueLength(ctx, 1)
	}
	return err
}

func (q *SQLiteTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	if err := q.ensureTable(ctx); err != nil {
		return nil, err
	}

	if len(roles) == 0 {
		return nil, nil
	}

	// Just a simple wrapper for mu if we are in sqlite
	var unlock func()
	if q.db.IsSQLite() {
		q.mu.Lock()
		unlock = func() { q.mu.Unlock() }
	} else {
		unlock = func() {}
	}
	defer unlock()

	// In SQLite, we do select and update. In Postgres, we use FOR UPDATE SKIP LOCKED
	var id, parentTaskID, agentRole, payloadStr, status string
	var attempts, maxAttempts int
	var runAfterStr, lockedUntilStr, createdAtStr, updatedAtStr sql.NullString

	// Placeholders for roles
	placeholders := make([]string, len(roles))
	args := make([]interface{}, len(roles))
	for i, role := range roles {
		placeholders[i] = fmt.Sprintf("$%d", i+1)
		args[i] = role
	}
	rolesClause := strings.Join(placeholders, ", ")

	var query string

	// Add current time as the last argument
	args = append(args, time.Now().UTC())
	timeArg := fmt.Sprintf("$%d", len(args))

	if q.db.IsSQLite() {
		// SQLite: First select the highest priority runnable job, then update it
		query = fmt.Sprintf(`
			SELECT id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at
			FROM sub_agent_jobs
			WHERE status = 'QUEUED' AND run_after <= %s AND agent_role IN (%s)
			ORDER BY run_after ASC
			LIMIT 1
		`, timeArg, rolesClause)

		err := q.db.QueryRow(ctx, query, args...).Scan(
			&id, &parentTaskID, &agentRole, &payloadStr, &status, &attempts, &maxAttempts, &runAfterStr, &lockedUntilStr, &createdAtStr, &updatedAtStr,
		)

		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}

		// Update to mark as RUNNING
		lockedTime := time.Now().UTC().Add(5 * time.Minute) // Lock for 5 minutes
		updateQuery := `
			UPDATE sub_agent_jobs
			SET status = 'RUNNING', locked_until = $1, attempts = attempts + 1, updated_at = CURRENT_TIMESTAMP
			WHERE id = $2 AND status = 'QUEUED'
		`
		_, err = q.db.Exec(ctx, updateQuery, lockedTime, id)
		if err != nil {
			return nil, err
		}

		status = "RUNNING"
		attempts++

	} else {
		// Postgres
		query = fmt.Sprintf(`
			UPDATE sub_agent_jobs
			SET status = 'RUNNING', locked_until = CURRENT_TIMESTAMP + INTERVAL '5 minutes', attempts = attempts + 1, updated_at = CURRENT_TIMESTAMP
			WHERE id = (
				SELECT id FROM sub_agent_jobs
				WHERE status = 'QUEUED' AND run_after <= %s AND agent_role IN (%s)
				ORDER BY run_after ASC
				FOR UPDATE SKIP LOCKED
				LIMIT 1
			)
			RETURNING id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at
		`, timeArg, rolesClause)

		err := q.db.QueryRow(ctx, query, args...).Scan(
			&id, &parentTaskID, &agentRole, &payloadStr, &status, &attempts, &maxAttempts, &runAfterStr, &lockedUntilStr, &createdAtStr, &updatedAtStr,
		)

		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}
	}

	telemetry.RecordTaskQueueLength(ctx, -1)

	return &Job{
		ID:           id,
		ParentTaskID: parentTaskID,
		AgentRole:    agentRole,
		Payload:      payloadStr,
		Status:       status,
		Attempts:     attempts,
		MaxAttempts:  maxAttempts,
		RunAfter:     time.Now().UTC(),
		LockedUntil:  time.Now().UTC(),
		CreatedAt:    time.Now().UTC(),
		UpdatedAt:    time.Now().UTC(),
	}, nil
}

func (q *SQLiteTaskQueue) Complete(ctx context.Context, jobID string) error {
	query := `UPDATE sub_agent_jobs SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP WHERE id = $1`
	_, err := q.db.Exec(ctx, query, jobID)
	return err
}

func (q *SQLiteTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	// If attempts < max_attempts, set back to QUEUED, otherwise FAILED.
	// In a real system we'd probably add an exponential backoff to run_after.
	query := `
		UPDATE sub_agent_jobs
		SET status = CASE WHEN attempts < max_attempts THEN 'QUEUED' ELSE 'FAILED' END,
		    updated_at = CURRENT_TIMESTAMP
		WHERE id = $1
	`
	_, err := q.db.Exec(ctx, query, jobID)
	return err
}
