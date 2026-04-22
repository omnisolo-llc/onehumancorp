package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
	"github.com/onehumancorp/mono/srcs/server/db"
)

type SharedTaskOrchestrator struct {
	db db.Provider
}

func NewSharedTaskOrchestrator(provider db.Provider) *SharedTaskOrchestrator {
	return &SharedTaskOrchestrator{db: provider}
}

func (o *SharedTaskOrchestrator) CreateTask(ctx context.Context, task *SharedTask) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}

	depBytes, err := json.Marshal(task.Dependencies)
	if err != nil {
		return fmt.Errorf("failed to marshal dependencies: %w", err)
	}
	depsStr := string(depBytes)

	query := `
		INSERT INTO shared_tasks_v4 (
			id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9, $10
		)
	`
	_, err = o.db.Exec(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status,
		task.AssignedAgentID, task.Priority, task.Payload, task.ParentPlanID, depsStr,
	)
	return err
}

func (o *SharedTaskOrchestrator) GetTask(ctx context.Context, id string) (*SharedTask, error) {
	query := `
		SELECT id, organization_id, title, description, status, agent_id, priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks_v4
		WHERE id = $1
	`
	row := o.db.QueryRow(ctx, query, id)

	var task SharedTask
	var depsStr string
	var createdAt, updatedAt any
	err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &task.Description, &task.Status,
		&task.AssignedAgentID, &task.Priority, &task.Payload, &task.ParentPlanID, &depsStr,
		&createdAt, &updatedAt,
	)
	if err != nil {
		return nil, err
	}

	switch v := createdAt.(type) {
	case time.Time:
		task.CreatedAt = v
	case string:
		task.CreatedAt, _ = time.Parse(time.RFC3339, v)
	}

	switch v := updatedAt.(type) {
	case time.Time:
		task.UpdatedAt = v
	case string:
		task.UpdatedAt, _ = time.Parse(time.RFC3339, v)
	}

	err = json.Unmarshal([]byte(depsStr), &task.Dependencies)
	if err != nil {
		return nil, fmt.Errorf("failed to unmarshal dependencies: %w", err)
	}

	return &task, nil
}

func (o *SharedTaskOrchestrator) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	query := `
		UPDATE shared_tasks_v4
		SET status = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	_, err := o.db.Exec(ctx, query, status, id)
	return err
}
