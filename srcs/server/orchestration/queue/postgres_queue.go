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

type PostgresTaskQueue struct {
	provider db.Provider
}

func NewPostgresTaskQueue(provider db.Provider) *PostgresTaskQueue {
	return &PostgresTaskQueue{provider: provider}
}

func (q *PostgresTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	defer func() {
		telemetry.RecordQueueLength(ctx, 1) // Approximation
	}()

	if job.RunAfter.IsZero() {
		job.RunAfter = time.Now()
	}

	// Make sure payload contains the agent_role
	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(job.Payload), &payloadMap); err != nil {
		payloadMap = map[string]interface{}{}
	}
	payloadMap["agent_role"] = job.AgentRole
	payloadMap["attempts"] = job.Attempts
	payloadMap["max_attempts"] = job.MaxAttempts

	newPayload, _ := json.Marshal(payloadMap)

	orgID := ""
	if val, ok := payloadMap["organization_id"].(string); ok {
		orgID = val
	}

	query := `
		INSERT INTO sub_agent_queue (
			id, organization_id, parent_task_id, payload, status, scheduled_at
		) VALUES (
			$1, $2, $3, $4, $5, $6
		)
	`
	_, err := q.provider.Exec(ctx, query,
		job.ID, orgID, job.ParentTaskID, string(newPayload),
		"PENDING", job.RunAfter,
	)
	return err
}

func (q *PostgresTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	// Select a PENDING job. We use FOR UPDATE SKIP LOCKED
	// Agent role is in the payload. We extract it using JSON ops ->> 'agent_role'

	rolePlaceholders := make([]string, len(roles))
	args := []any{}

	for i, role := range roles {
		rolePlaceholders[i] = fmt.Sprintf("$%d", i+1)
		args = append(args, role)
	}

	roleFilter := ""
	if len(roles) > 0 {
		roleFilter = fmt.Sprintf("AND payload::json->>'agent_role' IN (%s)", strings.Join(rolePlaceholders, ", "))
	}

	now := time.Now()
	args = append(args, now)
	nowIdx := len(args)

	query := fmt.Sprintf(`
		UPDATE sub_agent_queue
		SET status = 'RUNNING'
		WHERE id = (
			SELECT id
			FROM sub_agent_queue
			WHERE status = 'PENDING'
			  AND scheduled_at <= $%d
			  %s
			ORDER BY scheduled_at ASC
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, parent_task_id, payload, status, scheduled_at
	`, nowIdx, roleFilter)

	var j Job
	var scheduledAt time.Time
	var orgID string

	err := q.provider.QueryRow(ctx, query, args...).Scan(
		&j.ID, &orgID, &j.ParentTaskID, &j.Payload, &j.Status, &scheduledAt,
	)

	if errors.Is(err, sql.ErrNoRows) {
		return nil, nil // No jobs available
	} else if err != nil {
		return nil, err
	}

	telemetry.RecordQueueLength(ctx, -1)
		telemetry.RecordSubAgentQueueDelay(ctx, time.Since(scheduledAt).Seconds())

	j.RunAfter = scheduledAt

	// Reconstruct other fields from payload
	var payloadMap map[string]interface{}
	if err := json.Unmarshal([]byte(j.Payload), &payloadMap); err == nil {
		if role, ok := payloadMap["agent_role"].(string); ok {
			j.AgentRole = role
		}
		if attempts, ok := payloadMap["attempts"].(float64); ok {
			j.Attempts = int(attempts)
		}
		if maxAttempts, ok := payloadMap["max_attempts"].(float64); ok {
			j.MaxAttempts = int(maxAttempts)
		}
	}

	j.Attempts++

	return &j, nil
}

func (q *PostgresTaskQueue) Complete(ctx context.Context, jobID string) error {
	query := `
		UPDATE sub_agent_queue
		SET status = 'COMPLETED', completed_at = $1
		WHERE id = $2
	`
	_, err := q.provider.Exec(ctx, query, time.Now(), jobID)
	return err
}

func (q *PostgresTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	// First fetch the payload
	query := `SELECT payload FROM sub_agent_queue WHERE id = $1`
	var payloadStr string
	err := q.provider.QueryRow(ctx, query, jobID).Scan(&payloadStr)
	if err != nil {
		return err
	}

	var payload map[string]interface{}
	json.Unmarshal([]byte(payloadStr), &payload)
	if payload == nil {
		payload = make(map[string]interface{})
	}

	attempts := 1
	if a, ok := payload["attempts"].(float64); ok {
		attempts = int(a)
	}

	maxAttempts := 3
	if m, ok := payload["max_attempts"].(float64); ok {
		maxAttempts = int(m)
	}

	var status string
	var nextRunAfter time.Time

	if attempts >= maxAttempts {
		status = "FAILED"
		nextRunAfter = time.Now()
	} else {
		status = "PENDING"
		backoff := time.Duration(1<<attempts) * time.Second
		nextRunAfter = time.Now().Add(backoff)
		telemetry.RecordQueueLength(ctx, 1) // Job returned to queue
	}

	payload["last_error"] = reason
	newPayload, _ := json.Marshal(payload)

	updateQuery := `
		UPDATE sub_agent_queue
		SET status = $1, scheduled_at = $2, payload = $3
		WHERE id = $4
	`
	_, err = q.provider.Exec(ctx, updateQuery, status, nextRunAfter, string(newPayload), jobID)
	return err
}
