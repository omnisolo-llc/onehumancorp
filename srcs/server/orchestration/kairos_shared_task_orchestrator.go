package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
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
	AgentID        *string         `json:"agent_id,omitempty"`
	Dependencies   json.RawMessage `json:"dependencies"`
	CreatedAt      time.Time       `json:"created_at"`
	UpdatedAt      time.Time       `json:"updated_at"`
}

var sqliteClaimMu sync.Mutex

func ClaimSharedTask(ctx context.Context, database db.Provider, organizationID string, agentID string) (*KairosSharedTask, error) {
	var task KairosSharedTask
	var desc sql.NullString
	var agent sql.NullString
	var depsStr string
	var createdAt, updatedAt string

	if database.IsSQLite() {
		sqliteClaimMu.Lock()
		defer sqliteClaimMu.Unlock()

		tx, err := database.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		query := `SELECT id, organization_id, title, description, status, priority, agent_id, dependencies, created_at, updated_at FROM shared_tasks WHERE status = 'PENDING' AND organization_id = $1 LIMIT 1`
		err = tx.QueryRow(ctx, query, organizationID).Scan(&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Status, &task.Priority, &agent, &depsStr, &createdAt, &updatedAt)
		if err != nil {
			if err == sql.ErrNoRows {
				return nil, nil
			}
			return nil, err
		}

		updateQuery := `UPDATE shared_tasks SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`
		_, err = tx.Exec(ctx, updateQuery, agentID, task.ID)
		if err != nil {
			return nil, err
		}

		err = tx.Commit(ctx)
		if err != nil {
			return nil, err
		}

		if desc.Valid {
			task.Description = &desc.String
		}
		task.Status = "IN_PROGRESS"
		task.AgentID = &agentID
		task.Dependencies = json.RawMessage(depsStr)
		if task.Dependencies == nil {
			task.Dependencies = json.RawMessage("[]")
		}

		task.CreatedAt, _ = time.Parse("2006-01-02 15:04:05", createdAt)
		task.UpdatedAt, _ = time.Parse("2006-01-02 15:04:05", updatedAt)

		return &task, nil
	}

	query := `
		UPDATE shared_tasks
		SET status = 'IN_PROGRESS', agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks
			WHERE status = 'PENDING' AND organization_id = $2
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, title, description, status, priority, agent_id, dependencies, created_at, updated_at
	`

	var tCreated, tUpdated time.Time
	err := database.QueryRow(ctx, query, agentID, organizationID).Scan(
		&task.ID,
		&task.OrganizationID,
		&task.Title,
		&desc,
		&task.Status,
		&task.Priority,
		&agent,
		&depsStr,
		&tCreated,
		&tUpdated,
	)
	if err != nil {
		if err == sql.ErrNoRows || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return nil, nil
		}
		return nil, err
	}

	if desc.Valid {
		task.Description = &desc.String
	}
	if agent.Valid {
		task.AgentID = &agent.String
	}
	task.Dependencies = json.RawMessage(depsStr)
	if task.Dependencies == nil {
		task.Dependencies = json.RawMessage("[]")
	}
	task.CreatedAt = tCreated
	task.UpdatedAt = tUpdated

	return &task, nil
}
