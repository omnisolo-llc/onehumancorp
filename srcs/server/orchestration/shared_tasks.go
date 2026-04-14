package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SharedTaskDecomposition struct {
	ID             string
	OrganizationID string
	Title          string
	Description    sql.NullString
	Status         string
	AgentID        sql.NullString
	Priority       string
	Payload        json.RawMessage
	ParentPlanID   sql.NullString
	Dependencies   json.RawMessage
	LockedUntil    sql.NullTime
	CreatedAt      time.Time
	UpdatedAt      time.Time
}

func ClaimTask(ctx context.Context, database db.Provider, agentID string) (*SharedTaskDecomposition, error) {
	var task SharedTaskDecomposition
	var desc, agent, parent sql.NullString
	var locked sql.NullTime
    var createdAt, updatedAt string
    var payload, dependencies sql.NullString

	var query string
	if database.IsSQLite() {
		tx, err := database.Begin(ctx)
		if err != nil {
			return nil, err
		}
		defer tx.Rollback(ctx)

		query = `SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at FROM shared_tasks_v4 WHERE status = 'PENDING' LIMIT 1`
		err = tx.QueryRow(ctx, query).Scan(&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Status, &agent, &task.Priority, &payload, &parent, &dependencies, &locked, &createdAt, &updatedAt)
		if err != nil {
			return nil, err
		}

		_, err = tx.Exec(ctx, `UPDATE shared_tasks_v4 SET status = 'ASSIGNED', agent_id = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, agentID, task.ID)
		if err != nil {
			return nil, err
		}
		err = tx.Commit(ctx)
		if err != nil {
			return nil, err
		}

        // update memory struct
        agent.String = agentID
        agent.Valid = true
        task.Status = "ASSIGNED"

	} else {
		query = `
		UPDATE shared_tasks_v4
		SET status = 'ASSIGNED', agent_id = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = (
			SELECT id FROM shared_tasks_v4
			WHERE status = 'PENDING'
			FOR UPDATE SKIP LOCKED
			LIMIT 1
		)
		RETURNING id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, locked_until, created_at, updated_at
	    `

		err := database.QueryRow(ctx, query, agentID).Scan(
			&task.ID,
			&task.OrganizationID,
			&task.Title,
			&desc,
			&task.Status,
			&agent,
			&task.Priority,
			&payload,
			&parent,
			&dependencies,
			&locked,
			&task.CreatedAt,
			&task.UpdatedAt,
		)
		if err != nil {
			return nil, err
		}
	}
	task.Description = desc
	task.AgentID = agent
	task.ParentPlanID = parent
	task.LockedUntil = locked
    if payload.Valid && payload.String != "" {
        task.Payload = json.RawMessage(payload.String)
    } else {
        task.Payload = json.RawMessage("{}")
    }

    if dependencies.Valid && dependencies.String != "" {
        task.Dependencies = json.RawMessage(dependencies.String)
    } else {
        task.Dependencies = json.RawMessage("[]")
    }

    if database.IsSQLite() {
        task.CreatedAt, _ = time.Parse("2006-01-02 15:04:05", createdAt)
        task.UpdatedAt, _ = time.Parse("2006-01-02 15:04:05", updatedAt)
    }

	return &task, nil
}

func TransitionTask(ctx context.Context, database db.Provider, taskID, status string) error {
	_, err := database.Exec(ctx, `UPDATE shared_tasks_v4 SET status = $1, updated_at = CURRENT_TIMESTAMP WHERE id = $2`, status, taskID)
	return err
}

func CreateTask(ctx context.Context, database db.Provider, task *SharedTaskDecomposition) error {
	var id string

    if task.ID == "" {
        task.ID = uuid.New().String()
    }

    payloadStr := "{}"
    if len(task.Payload) > 0 {
        payloadStr = string(task.Payload)
    }

    depStr := "[]"
    if len(task.Dependencies) > 0 {
        depStr = string(task.Dependencies)
    }

	if database.IsSQLite() {
		query := `INSERT INTO shared_tasks_v4 (id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, locked_until) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11) RETURNING id`
		err := database.QueryRow(ctx, query, task.ID, task.OrganizationID, task.Title, task.Description, task.Status, task.AgentID, task.Priority, payloadStr, task.ParentPlanID, depStr, task.LockedUntil).Scan(&id)
		if err != nil {
			return err
		}
	} else {
		query := `
		INSERT INTO shared_tasks_v4 (id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, locked_until)
		VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11)
		RETURNING id
	    `
		err := database.QueryRow(ctx, query,
            task.ID,
			task.OrganizationID,
			task.Title,
			task.Description,
			task.Status,
			task.AgentID,
			task.Priority,
			payloadStr,
			task.ParentPlanID,
			depStr,
			task.LockedUntil,
		).Scan(&id)
		if err != nil {
			return err
		}
	}
	task.ID = id
	return nil
}
