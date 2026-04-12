package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// TaskStatus represents the current state of a task
type TaskStatus string

const (
	TaskStatusPending    TaskStatus = "PENDING"
	TaskStatusInProgress TaskStatus = "IN_PROGRESS"
	TaskStatusCompleted  TaskStatus = "COMPLETED"
	TaskStatusBlocked    TaskStatus = "BLOCKED"
	TaskStatusFailed     TaskStatus = "FAILED"
)

// DistributedTask represents a distributed task in the system
type DistributedTask struct {
	ID             string          `json:"id"`
	OrganizationID string          `json:"organization_id"`
	ParentPlanID   *string         `json:"parent_plan_id,omitempty"`
	Title          string          `json:"title"`
	Description    string          `json:"description"`
	Status         TaskStatus      `json:"status"`
	AgentID        *string         `json:"agent_id,omitempty"`
	Dependencies   json.RawMessage `json:"dependencies"`
	CreatedAt      time.Time       `json:"created_at"`
	UpdatedAt      time.Time       `json:"updated_at"`
}

// Orchestrator manages shared tasks and their dependencies
type Orchestrator struct {
	pool db.Provider
}

// NewOrchestrator creates a new task orchestrator
func NewOrchestrator(pool db.Provider) *Orchestrator {
	return &Orchestrator{pool: pool}
}

// ClaimTask attempts to claim an available task, handling distributed locking
// and DAG dependencies
func (o *Orchestrator) ClaimTask(ctx context.Context, agentID string) (*DistributedTask, error) {
	tx, err := o.pool.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("begin claim tx: %w", err)
	}
	defer func() { _ = tx.Rollback(ctx) }()

	// Query looks for pending tasks where ALL dependencies are COMPLETED
	// or tasks that have no dependencies
	query := `
		SELECT st.id, st.organization_id, st.parent_plan_id, st.title, st.description, st.status, st.agent_id, st.dependencies, st.created_at, st.updated_at
		FROM shared_tasks st
		WHERE st.status = 'PENDING'
		AND NOT EXISTS (
			SELECT 1 FROM task_dependencies td
			JOIN shared_tasks dep ON td.depends_on_task_id = dep.id
			WHERE td.task_id = st.id AND dep.status != 'COMPLETED'
		)
		LIMIT 1`

	if !o.pool.IsSQLite() {
		query += " FOR UPDATE SKIP LOCKED"
	}

	var t DistributedTask
	var agentIDNull sql.NullString
	var parentPlanIDNull sql.NullString
	var descNull sql.NullString
	var createdAt, updatedAt db.FlexTime
	var depsStr string

	err = tx.QueryRow(ctx, query).Scan(
		&t.ID, &t.OrganizationID, &parentPlanIDNull, &t.Title, &descNull,
		&t.Status, &agentIDNull, &depsStr, &createdAt, &updatedAt,
	)

	if err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil // No task available
		}
		return nil, fmt.Errorf("query claim task: %w", err)
	}

	if parentPlanIDNull.Valid {
		t.ParentPlanID = &parentPlanIDNull.String
	}
	if descNull.Valid {
		t.Description = descNull.String
	}
	if agentIDNull.Valid {
		t.AgentID = &agentIDNull.String
	}
	t.Dependencies = json.RawMessage(depsStr)
	t.CreatedAt = createdAt.Time
	t.UpdatedAt = updatedAt.Time

	// Update the task to IN_PROGRESS and assign to agent
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2`

	res, err := tx.Exec(ctx, updateQuery, agentID, t.ID)
	if err != nil {
		return nil, fmt.Errorf("update claim task: %w", err)
	}
	if res == 0 {
		return nil, nil // Another worker might have grabbed it in SQLite
	}

	t.Status = TaskStatusInProgress
	t.AgentID = &agentID
	t.UpdatedAt = time.Now().UTC()

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("commit claim tx: %w", err)
	}

	return &t, nil
}
