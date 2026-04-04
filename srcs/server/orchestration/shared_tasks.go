package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"encoding/json"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// InterAgentSharedTask represents a task in the shared_tasks table.
type InterAgentSharedTask struct {
	ID             string
	OrganizationID string
	Title          string
	Description    string
	Status         string
	AgentID        string
	Priority       string
	Payload        string
	LockedUntil    sql.NullTime
	CreatedAt      time.Time
	UpdatedAt      time.Time
}

// CreateSharedTask creates a new inter-agent shared task.
func (tm *TaskManager) CreateSharedTask(ctx context.Context, orgID, title, description, priority string) (*InterAgentSharedTask, error) {
	if priority == "" {
		priority = "P2"
	}

	id := generateID()
	payloadMap := map[string]string{"description": description, "priority": priority}
	payloadBytes, err := json.Marshal(payloadMap)
	if err != nil {
		return nil, fmt.Errorf("failed to encode payload: %w", err)
	}
	payload := string(payloadBytes)

	var task InterAgentSharedTask
	var query string

	if tm.db.IsSQLite() {
		query = `
			INSERT INTO shared_tasks (id, organization_id, title, description, priority, payload, status, created_at, updated_at)
			VALUES ($1, $2, $3, $4, $5, $6, 'PENDING', CURRENT_TIMESTAMP, CURRENT_TIMESTAMP)
			RETURNING id, organization_id, title, description, priority, payload, status, created_at, updated_at
		`
	} else {
		query = `
			INSERT INTO shared_tasks (id, organization_id, title, description, priority, payload, status)
			VALUES ($1, $2, $3, $4, $5, $6, 'PENDING')
			RETURNING id, organization_id, title, description, priority, payload, status, created_at, updated_at
		`
	}

	var desc sql.NullString
	err = tm.db.QueryRow(ctx, query, id, orgID, title, description, priority, payload).Scan(
		&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Priority, &task.Payload, &task.Status, &task.CreatedAt, &task.UpdatedAt,
	)

	if desc.Valid {
		task.Description = desc.String
	}

	if err != nil {
		return nil, fmt.Errorf("failed to create shared task: %w", err)
	}

	return &task, nil
}

// ClaimSharedTask claims a pending shared task.
func (tm *TaskManager) ClaimSharedTask(ctx context.Context, orgID, agentID string) (*InterAgentSharedTask, error) {
	tx, err := tm.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var task InterAgentSharedTask
	var errQuery error

	if tm.db.IsSQLite() {
		query := `
			SELECT id, organization_id, title, description, priority, payload, status, agent_id, locked_until, created_at, updated_at
			FROM shared_tasks
			WHERE organization_id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`
		var desc, aID sql.NullString
		errQuery = tx.QueryRow(ctx, query, orgID).Scan(
			&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Priority, &task.Payload, &task.Status, &aID, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
		if desc.Valid { task.Description = desc.String }
		if aID.Valid { task.AgentID = aID.String }
	} else {
		query := `
			SELECT id, organization_id, title, description, priority, payload, status, agent_id, locked_until, created_at, updated_at
			FROM shared_tasks
			WHERE organization_id = $1 AND status = 'PENDING' AND (locked_until IS NULL OR locked_until < CURRENT_TIMESTAMP)
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
			FOR UPDATE SKIP LOCKED
		`
		var desc, aID sql.NullString
		errQuery = tx.QueryRow(ctx, query, orgID).Scan(
			&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Priority, &task.Payload, &task.Status, &aID, &task.LockedUntil, &task.CreatedAt, &task.UpdatedAt,
		)
		if desc.Valid { task.Description = desc.String }
		if aID.Valid { task.AgentID = aID.String }
	}

	if errQuery != nil {
		if errors.Is(errQuery, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("failed to find pending shared task: %w", errQuery)
	}

	updateQuery := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2 AND status = 'PENDING'
	`
	rowsAffected, err := tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, fmt.Errorf("failed to update shared task status: %w", err)
	}

	if rowsAffected == 0 {
		return nil, nil
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AgentID = agentID

	return &task, nil
}

// CompleteSharedTask marks a shared task as completed.
func (tm *TaskManager) CompleteSharedTask(ctx context.Context, taskID, agentID string) error {
	query := `
		UPDATE shared_tasks
		SET status = 'COMPLETED', updated_at = CURRENT_TIMESTAMP
		WHERE id = $1 AND agent_id = $2 AND status = 'IN_PROGRESS'
	`
	rowsAffected, err := tm.db.Exec(ctx, query, taskID, agentID)
	if err != nil {
		return fmt.Errorf("failed to complete shared task: %w", err)
	}

	if rowsAffected == 0 {
		return errors.New("shared task not found or not assigned to agent")
	}

	return nil
}
