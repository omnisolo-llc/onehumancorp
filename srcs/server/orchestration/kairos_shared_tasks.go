package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

type KairosSharedTask struct {
	ID             string          `json:"id"`
	OrganizationID string          `json:"organization_id"`
	Title          string          `json:"title"`
	Description    *string         `json:"description,omitempty"`
	Status         string          `json:"status"`
	Priority       string          `json:"priority"`
	AssignedAgent  *string         `json:"assigned_agent,omitempty"`
	Dependencies   json.RawMessage `json:"dependencies"`
	CreatedAt      time.Time       `json:"created_at"`
	UpdatedAt      time.Time       `json:"updated_at"`
}

type SharedTaskRepo struct {
	dbProvider db.Provider
	mutex      *sync.Mutex
}

func NewSharedTaskRepo(dbProvider db.Provider) *SharedTaskRepo {
	return &SharedTaskRepo{
		dbProvider: dbProvider,
		mutex:      &sync.Mutex{},
	}
}

func (r *SharedTaskRepo) ClaimTask(ctx context.Context, agentID string) (*KairosSharedTask, error) {
	if r.dbProvider.IsSQLite() {
		return r.claimTaskSQLite(ctx, agentID)
	}
	return r.claimTaskPostgres(ctx, agentID)
}

func (r *SharedTaskRepo) claimTaskSQLite(ctx context.Context, agentID string) (*KairosSharedTask, error) {
	r.mutex.Lock()
	defer r.mutex.Unlock()

	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
        SELECT id, organization_id, title, description, status, priority, assigned_agent, dependencies, created_at, updated_at
        FROM shared_tasks
        WHERE status = 'PENDING'
        LIMIT 1
    `
	row := tx.QueryRow(ctx, query)

	var task KairosSharedTask
	var desc, agent sql.NullString
	var deps sql.NullString
	var createdAtStr, updatedAtStr string
	if err := row.Scan(&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Status, &task.Priority, &agent, &deps, &createdAtStr, &updatedAtStr); err != nil {
		if err == sql.ErrNoRows {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if desc.Valid {
		task.Description = &desc.String
	}
	if agent.Valid {
		task.AssignedAgent = &agent.String
	}
	if deps.Valid && deps.String != "" {
		if err := json.Unmarshal([]byte(deps.String), &task.Dependencies); err != nil {
			task.Dependencies = json.RawMessage("[]")
		}
	} else {
		task.Dependencies = json.RawMessage("[]")
	}
	task.CreatedAt, _ = time.Parse("2006-01-02 15:04:05", createdAtStr)
	task.UpdatedAt, _ = time.Parse("2006-01-02 15:04:05", updatedAtStr)

	_, err = tx.Exec(ctx, `
        UPDATE shared_tasks
        SET status = 'IN_PROGRESS', assigned_agent = $1, updated_at = CURRENT_TIMESTAMP
        WHERE id = $2
    `, agentID, task.ID)

	if err != nil {
		return nil, fmt.Errorf("failed to update task: %w", err)
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	task.Status = "IN_PROGRESS"
	task.AssignedAgent = &agentID

	return &task, nil
}

func (r *SharedTaskRepo) claimTaskPostgres(ctx context.Context, agentID string) (*KairosSharedTask, error) {
	tx, err := r.dbProvider.Begin(ctx)
	if err != nil {
		return nil, fmt.Errorf("failed to begin transaction: %w", err)
	}
	defer tx.Rollback(ctx)

	query := `
        UPDATE shared_tasks
        SET status = 'IN_PROGRESS', assigned_agent = $1, updated_at = CURRENT_TIMESTAMP
        WHERE id = (
            SELECT id FROM shared_tasks
            WHERE status = 'PENDING'
            LIMIT 1
            FOR UPDATE SKIP LOCKED
        )
        RETURNING id, organization_id, title, description, status, priority, assigned_agent, dependencies, created_at, updated_at
    `
	row := tx.QueryRow(ctx, query, agentID)

	var task KairosSharedTask
	var desc, agent sql.NullString
	var deps sql.NullString
	if err := row.Scan(&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Status, &task.Priority, &agent, &deps, &task.CreatedAt, &task.UpdatedAt); err != nil {
		if err.Error() == "sql: no rows in result set" || err.Error() == "no rows in result set" {
			return nil, nil
		}
		return nil, fmt.Errorf("failed to query pending task: %w", err)
	}

	if desc.Valid {
		task.Description = &desc.String
	}
	if agent.Valid {
		task.AssignedAgent = &agent.String
	}
	if deps.Valid && deps.String != "" {
		if err := json.Unmarshal([]byte(deps.String), &task.Dependencies); err != nil {
			task.Dependencies = json.RawMessage("[]")
		}
	} else {
		task.Dependencies = json.RawMessage("[]")
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, fmt.Errorf("failed to commit transaction: %w", err)
	}

	return &task, nil
}
