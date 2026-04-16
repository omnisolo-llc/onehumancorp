package queue

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
)

type SQLiteTaskQueue struct {
	provider db.Provider
}

func NewSQLiteTaskQueue(provider db.Provider) *SQLiteTaskQueue {
	return &SQLiteTaskQueue{provider: provider}
}

func (q *SQLiteTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	defer func() {
		telemetry.RecordQueueLength(ctx, 1) // Approximation
	}()

	if job.RunAfter.IsZero() {
		job.RunAfter = time.Now()
	}

	query := `
		INSERT INTO sub_agent_jobs (
			id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8
		)
	`
	_, err := q.provider.Exec(ctx, query,
		job.ID, job.ParentTaskID, job.AgentRole, job.Payload,
		"QUEUED", job.Attempts, job.MaxAttempts, job.RunAfter.Format(time.RFC3339Nano),
	)
	return err
}

func (q *SQLiteTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	// In SQLite, we don't have FOR UPDATE SKIP LOCKED.
	// We'll use a transaction with a quick UPDATE to acquire.
	tx, err := q.provider.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var rolePlaceholders []string
	var args []any
	for i, role := range roles {
		rolePlaceholders = append(rolePlaceholders, fmt.Sprintf("$%d", i+1))
		args = append(args, role)
	}

	rolesCondition := ""
	if len(roles) > 0 {
		rolesCondition = fmt.Sprintf("AND agent_role IN (%s)", strings.Join(rolePlaceholders, ", "))
	}

	now := time.Now().Format(time.RFC3339Nano)
	args = append(args, now)
	nowPlaceholder := fmt.Sprintf("$%d", len(args))

	// We check for both QUEUED jobs, and RUNNING jobs that have crashed (locked_until has passed)
	query := fmt.Sprintf(`
		SELECT id, parent_task_id, agent_role, payload, status, attempts, max_attempts, run_after, locked_until, created_at, updated_at
		FROM sub_agent_jobs
		WHERE (status = 'QUEUED' AND run_after <= %s %s)
		   OR (status = 'RUNNING' AND locked_until IS NOT NULL AND locked_until <= %s %s)
		ORDER BY run_after ASC
		LIMIT 1
	`, nowPlaceholder, rolesCondition, nowPlaceholder, rolesCondition)

	row := tx.QueryRow(ctx, query, args...)

	var j Job
	var lockedUntil sql.NullString
	var runAfterStr, createdAtStr, updatedAtStr string
	err = row.Scan(&j.ID, &j.ParentTaskID, &j.AgentRole, &j.Payload, &j.Status, &j.Attempts, &j.MaxAttempts, &runAfterStr, &lockedUntil, &createdAtStr, &updatedAtStr)

	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil // No jobs available
	} else if err != nil {
		return nil, err
	}

	// Update the job to mark it as RUNNING (simulate acquiring lock)
	lockTime := time.Now().Add(5 * time.Minute)
	updateQuery := `
		UPDATE sub_agent_jobs
		SET status = 'RUNNING', locked_until = $1, attempts = attempts + 1, updated_at = $2
		WHERE id = $3 AND (status = 'QUEUED' OR status = 'RUNNING')
	`
	res, err := tx.Exec(ctx, updateQuery, lockTime.Format(time.RFC3339Nano), time.Now().Format(time.RFC3339Nano), j.ID)
	if err != nil {
		return nil, err
	}

	rowsAffected := res
	if rowsAffected == 0 {
		// Someone else grabbed it between our SELECT and UPDATE
		return nil, nil
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	telemetry.RecordQueueLength(ctx, -1) // Job removed from queued state

	parseTime := func(s string) time.Time {
		t, err := time.Parse(time.RFC3339Nano, s)
		if err == nil {
			return t
		}
		// Fallback for SQLite CURRENT_TIMESTAMP format
		t, err = time.Parse(time.DateTime, s)
		if err == nil {
			return t
		}
		return time.Time{}
	}

	j.RunAfter = parseTime(runAfterStr)
	j.CreatedAt = parseTime(createdAtStr)
	j.UpdatedAt = parseTime(updatedAtStr)

	if !j.RunAfter.IsZero() {
		delay := time.Since(j.RunAfter).Seconds()
		telemetry.RecordSubAgentQueueDelay(ctx, delay)
	}
	if lockedUntil.Valid && lockedUntil.String != "" {
		lt := parseTime(lockedUntil.String)
		j.LockedUntil = &lt
	}

	j.Status = "RUNNING"
	j.Attempts++

	return &j, nil
}

func (q *SQLiteTaskQueue) Complete(ctx context.Context, jobID string) error {
	query := `
		UPDATE sub_agent_jobs
		SET status = 'COMPLETED', updated_at = $1, locked_until = NULL
		WHERE id = $2
	`
	_, err := q.provider.Exec(ctx, query, time.Now().Format(time.RFC3339Nano), jobID)
	return err
}

func (q *SQLiteTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	// First fetch the job to check attempts
	query := `SELECT attempts, max_attempts FROM sub_agent_jobs WHERE id = $1`
	var attempts, maxAttempts int
	err := q.provider.QueryRow(ctx, query, jobID).Scan(&attempts, &maxAttempts)
	if err != nil {
		return err
	}

	var status string
	var nextRunAfter string
	var lockUntil interface{}

	if attempts >= maxAttempts {
		status = "FAILED"
		nextRunAfter = time.Now().Format(time.RFC3339Nano)
		lockUntil = nil
	} else {
		status = "QUEUED"
		// Exponential backoff
		backoff := time.Duration(1<<attempts) * time.Second
		nextRunAfter = time.Now().Add(backoff).Format(time.RFC3339Nano)
		lockUntil = nil
		telemetry.RecordQueueLength(ctx, 1) // Job returned to queue
	}

	// Add reason to payload ideally, but for now just update status
	var payload map[string]interface{}
	payloadQuery := `SELECT payload FROM sub_agent_jobs WHERE id = $1`
	var payloadStr string
	if err := q.provider.QueryRow(ctx, payloadQuery, jobID).Scan(&payloadStr); err == nil {
		json.Unmarshal([]byte(payloadStr), &payload)
		if payload == nil {
			payload = make(map[string]interface{})
		}
		payload["last_error"] = reason
		newPayload, _ := json.Marshal(payload)
		payloadStr = string(newPayload)
	}

	updateQuery := `
		UPDATE sub_agent_jobs
		SET status = $1, run_after = $2, locked_until = $3, updated_at = $4, payload = $5
		WHERE id = $6
	`
	_, err = q.provider.Exec(ctx, updateQuery, status, nextRunAfter, lockUntil, time.Now().Format(time.RFC3339Nano), payloadStr, jobID)
	return err
}
