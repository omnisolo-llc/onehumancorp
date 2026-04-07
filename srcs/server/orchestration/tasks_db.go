package orchestration

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"strings"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type TasksDB struct {
	db Provider
}

type Provider interface {
	db.Provider
}

func NewTasksDB(provider db.Provider) *TasksDB {
	return &TasksDB{db: provider}
}

// ClaimTask claims a specific PENDING task securely.
func (tdb *TasksDB) ClaimTask(ctx context.Context, agentID string) (*SharedTask, error) {
	claims := auth.ClaimsFromContext(ctx)
	if claims == nil {
		return nil, errors.New("unauthorized: missing claims")
	}

	tx, err := tdb.db.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	var fetchedTaskID string
	var queryErr error

	if tdb.db.IsSQLite() {
		// SQLite doesn't support FOR UPDATE SKIP LOCKED
		selectQuery := `
			SELECT id
			FROM shared_tasks
			WHERE organization_id = $1 AND status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1
		`
		queryErr = tx.QueryRow(ctx, selectQuery, claims.OrganizationID).Scan(&fetchedTaskID)
	} else {
		// PostgreSQL with FOR UPDATE SKIP LOCKED
		selectQuery := `
			SELECT id
			FROM shared_tasks
			WHERE organization_id = $1 AND status = 'PENDING'
			ORDER BY priority ASC, created_at ASC
			LIMIT 1 FOR UPDATE SKIP LOCKED
		`
		queryErr = tx.QueryRow(ctx, selectQuery, claims.OrganizationID).Scan(&fetchedTaskID)
	}

	if queryErr != nil {
		if errors.Is(queryErr, sql.ErrNoRows) {
			return nil, nil // No task available
		}
		if strings.Contains(queryErr.Error(), "database is locked") || strings.Contains(queryErr.Error(), "SQLITE_BUSY") {
			return nil, fmt.Errorf("database is locked: %w", queryErr)
		}
		return nil, fmt.Errorf("failed to check pending task: %w", queryErr)
	}

	// Update the task to IN_PROGRESS. We use optimistic concurrency for SQLite by verifying rows affected.
	updateQuery := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', agent_id = $1
		WHERE id = $2 AND status = 'PENDING'
	`
	res, err := tx.Exec(ctx, updateQuery, agentID, fetchedTaskID)
	if err != nil {
		return nil, fmt.Errorf("failed to update task status: %w", err)
	}

	rowsAffected := res.RowsAffected()

	if rowsAffected == 0 {
		return nil, nil // Task was already claimed by another agent before we could update
	}

	// Fetch updated task data
	readQuery := `
		SELECT id, organization_id, title, COALESCE(description, ''), status, COALESCE(agent_id, ''), priority, COALESCE(payload, '{}'), created_at, updated_at
		FROM shared_tasks
		WHERE id = $1
	`
	var task SharedTask
	var payloadBytes []byte
	errQuery := tx.QueryRow(ctx, readQuery, fetchedTaskID).Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID, &task.Priority, &payloadBytes, &task.CreatedAt, &task.UpdatedAt,
	)
	if errQuery != nil {
		return nil, fmt.Errorf("failed to read claimed task: %w", errQuery)
	}

	task.Payload = string(payloadBytes)

	// Fetch dependencies from task_dependencies table
	depQuery := `SELECT depends_on_task_id FROM task_dependencies WHERE task_id = $1`
	depRows, err := tx.Query(ctx, depQuery, task.ID)
	if err == nil {
		defer depRows.Close()
		for depRows.Next() {
			var depID string
			if err := depRows.Scan(&depID); err == nil {
				task.Dependencies = append(task.Dependencies, depID)
			}
		}
	} else if !strings.Contains(err.Error(), "no such table") {
		return nil, fmt.Errorf("failed to get dependencies: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &task, nil
}
