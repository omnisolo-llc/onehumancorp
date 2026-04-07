package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"errors"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
)

var ErrNoTasksAvailable = errors.New("no tasks available to claim")

type TasksDB struct {
	dbWrapper *db.DB
	mu        sync.Mutex
}

func NewTasksDB(dbWrapper *db.DB) *TasksDB {
	return &TasksDB{
		dbWrapper: dbWrapper,
	}
}

var PublishMeshEventFunc func(ctx context.Context, topic string, payload []byte) error

func SetPublishMeshEventFunc(f func(ctx context.Context, topic string, payload []byte) error) {
	PublishMeshEventFunc = f
}

func (t *TasksDB) CreateTask(ctx context.Context, task *SharedTask) error {
	depsJSON, err := json.Marshal(task.Dependencies)
	if err != nil {
		return err
	}

	query := `
		INSERT INTO shared_tasks_v2 (
			id, organization_id, parent_plan_id, title, description,
			status, assigned_agent_id, dependencies, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9, $10
		)
	`
	now := time.Now()
	if task.CreatedAt.IsZero() {
		task.CreatedAt = now
	}
	if task.UpdatedAt.IsZero() {
		task.UpdatedAt = now
	}
	if task.Status == "" {
		task.Status = "PENDING"
	}

	_, err = t.dbWrapper.Exec(ctx, query,
		task.ID, task.OrganizationID, task.ParentPlanID, task.Title, task.Description,
		task.Status, task.AssignedAgentID, string(depsJSON), task.CreatedAt, task.UpdatedAt)
	return err
}

func (t *TasksDB) ClaimTask(ctx context.Context, orgID string, agentID string) (*SharedTask, error) {
	if t.dbWrapper.IsSQLite() {
		t.mu.Lock()
		defer t.mu.Unlock()

		// SQLite: First select pending task
		query := `SELECT id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
				  FROM shared_tasks_v2
				  WHERE organization_id = $1 AND status = 'PENDING'
				  LIMIT 1`

		row := t.dbWrapper.QueryRow(ctx, query, orgID)
		var task SharedTask
		task.OrganizationID = orgID
		var depsStr string
	var createdAtStr, updatedAtStr sql.NullString
		err := row.Scan(&task.ID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID, &depsStr, &createdAtStr, &updatedAtStr)

		if createdAtStr.Valid {
			if t, err := time.Parse(time.RFC3339, createdAtStr.String); err == nil {
				task.CreatedAt = t
			}
		}
		if updatedAtStr.Valid {
			if t, err := time.Parse(time.RFC3339, updatedAtStr.String); err == nil {
				task.UpdatedAt = t
			}
		}

		if err != nil {
			if errors.Is(err, sql.ErrNoRows) {
				return nil, ErrNoTasksAvailable
			}
			return nil, err
		}

		// Update the task to ASSIGNED
		updateQuery := `UPDATE shared_tasks_v2 SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = $2 WHERE id = $3`
		now := time.Now()
		_, err = t.dbWrapper.Exec(ctx, updateQuery, agentID, now, task.ID)
		if err != nil {
			return nil, err
		}

		task.Status = "ASSIGNED"
		task.AssignedAgentID = agentID
		task.UpdatedAt = now
		if PublishMeshEventFunc != nil {
			PublishMeshEventFunc(ctx, "mesh:tasks", []byte(`{"event_type": "TASK_CLAIMED", "task_id": "`+task.ID+`"}`))
		}
		if depsStr != "" {
			_ = json.Unmarshal([]byte(depsStr), &task.Dependencies)
		}

		return &task, nil
	}

	// PostgreSQL: SELECT FOR UPDATE SKIP LOCKED
	tx, err := t.dbWrapper.Begin(ctx)
	if err != nil {
		return nil, err
	}
	defer tx.Rollback(ctx)

	query := `SELECT id, parent_plan_id, title, description, status, assigned_agent_id, dependencies, created_at, updated_at
			  FROM shared_tasks_v2
			  WHERE organization_id = $1 AND status = 'PENDING'
			  FOR UPDATE SKIP LOCKED LIMIT 1`

	row := tx.QueryRow(ctx, query, orgID)
	var task SharedTask
	task.OrganizationID = orgID
	var depsStr string
	var createdAtStr, updatedAtStr sql.NullString
	err = row.Scan(&task.ID, &task.ParentPlanID, &task.Title, &task.Description, &task.Status, &task.AssignedAgentID, &depsStr, &createdAtStr, &updatedAtStr)

		if createdAtStr.Valid {
			if t, err := time.Parse(time.RFC3339, createdAtStr.String); err == nil {
				task.CreatedAt = t
			}
		}
		if updatedAtStr.Valid {
			if t, err := time.Parse(time.RFC3339, updatedAtStr.String); err == nil {
				task.UpdatedAt = t
			}
		}

	if err != nil {
		if errors.Is(err, sql.ErrNoRows) {
			return nil, ErrNoTasksAvailable
		}
		return nil, err
	}

	updateQuery := `UPDATE shared_tasks_v2 SET status = 'ASSIGNED', assigned_agent_id = $1, updated_at = $2 WHERE id = $3`
	now := time.Now()
	_, err = tx.Exec(ctx, updateQuery, agentID, now, task.ID)
	if err != nil {
		return nil, err
	}

	if err := tx.Commit(ctx); err != nil {
		return nil, err
	}

	task.Status = "ASSIGNED"
	task.AssignedAgentID = agentID
	task.UpdatedAt = now
	if PublishMeshEventFunc != nil {
		PublishMeshEventFunc(ctx, "mesh:tasks", []byte(`{"event_type": "TASK_CLAIMED", "task_id": "`+task.ID+`"}`))
	}
	if depsStr != "" {
		_ = json.Unmarshal([]byte(depsStr), &task.Dependencies)
	}

	return &task, nil
}
