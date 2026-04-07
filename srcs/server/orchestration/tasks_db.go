package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/db"
)

// SharedTask represents a global orchestration task.
type SharedTask struct {
	ID              string
	OrganizationID  string
	ParentPlanID    sql.NullString
	Title           string
	Description     sql.NullString
	Status          string
	AssignedAgentID sql.NullString // corresponds to agent_id in DB
	Dependencies    []string // decoded JSONB array
	CreatedAt       string
	UpdatedAt       string
}

// TasksDB handles operations on the shared_tasks table.
type TasksDB struct {
	dbWrapper db.Provider
	mu        sync.Mutex // fallback for SQLite standalone
}

// NewTasksDB creates a new TasksDB instance.
func NewTasksDB(provider db.Provider) *TasksDB {
	return &TasksDB{
		dbWrapper: provider,
	}
}

// ClaimTask attempts to claim a PENDING task for the given organization and assigned agent.
func (t *TasksDB) ClaimTask(ctx context.Context, organizationID, agentID string) (*SharedTask, error) {
	if organizationID == "" || agentID == "" {
		return nil, fmt.Errorf("organizationID and agentID are required")
	}

	if t.dbWrapper.IsSQLite() {
		// SQLite Standalone mode: use application-level mutex and standard transaction isolation
		t.mu.Lock()
		defer t.mu.Unlock()

		tx, err := t.dbWrapper.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		var task SharedTask
		var depBytes sql.NullString // in SQLite, JSONB comes as string/bytes

		query := `SELECT id, organization_id, parent_plan_id, title, description, status, agent_id, dependencies, created_at, updated_at
		          FROM shared_tasks
		          WHERE status = 'PENDING' AND organization_id = ?
		          LIMIT 1`
		err = tx.QueryRow(ctx, query, organizationID).Scan(
			&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
			&task.Description, &task.Status, &task.AssignedAgentID, &depBytes,
			&task.CreatedAt, &task.UpdatedAt,
		)

		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil // No tasks found
			}
            if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
                return nil, nil
            }
			return nil, err
		}

		if depBytes.Valid && len(depBytes.String) > 0 {
			if err := json.Unmarshal([]byte(depBytes.String), &task.Dependencies); err != nil {
				return nil, fmt.Errorf("failed to decode dependencies: %w", err)
			}
		}

		// Update to ASSIGNED
		updateQuery := `UPDATE shared_tasks SET status = 'ASSIGNED', agent_id = ? WHERE id = ?`
		_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}

		if err := tx.Commit(ctx); err != nil {
			return nil, err
		}

		task.Status = "ASSIGNED"
		task.AssignedAgentID = sql.NullString{String: agentID, Valid: true}
		return &task, nil
	}

	// Cloud-Native mode (PostgreSQL): use FOR UPDATE SKIP LOCKED
	tx, err := t.dbWrapper.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	var task SharedTask
	var depBytes sql.NullString

	query := `SELECT id, organization_id, parent_plan_id, title, description, status, agent_id, dependencies, created_at, updated_at
	          FROM shared_tasks
	          WHERE status = 'PENDING' AND organization_id = $1
	          FOR UPDATE SKIP LOCKED LIMIT 1`
	err = tx.QueryRow(ctx, query, organizationID).Scan(
		&task.ID, &task.OrganizationID, &task.ParentPlanID, &task.Title,
		&task.Description, &task.Status, &task.AssignedAgentID, &depBytes,
		&task.CreatedAt, &task.UpdatedAt,
	)

	if err != nil {
		if err == sql.ErrNoRows {
			return nil, nil // No tasks found
		}
        if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
            return nil, nil
        }
		return nil, err
	}

	if depBytes.Valid && len(depBytes.String) > 0 {
		if err := json.Unmarshal([]byte(depBytes.String), &task.Dependencies); err != nil {
			return nil, fmt.Errorf("failed to decode dependencies: %w", err)
		}
	}

	updateQuery := `UPDATE shared_tasks SET status = 'ASSIGNED', agent_id = $1 WHERE id = $2`
	_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = sql.NullString{String: agentID, Valid: true}
	return &task, nil
}
