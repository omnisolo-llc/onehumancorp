package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"fmt"
	"time"

	"github.com/google/uuid"
)

// SharedTask represents a task in the shared_tasks_v4 table.
type SharedTask struct {
	ID             string    `json:"id"`
	OrganizationID string    `json:"organization_id"`
	Title          string    `json:"title"`
	Description    string    `json:"description"`
	Status         string    `json:"status"`
	AgentID        string    `json:"agent_id"`
	Priority       string    `json:"priority"`
	Payload        string    `json:"payload"`
	ParentPlanID   string    `json:"parent_plan_id"`
	Dependencies   []string  `json:"dependencies"`
	CreatedAt      time.Time `json:"created_at"`
	UpdatedAt      time.Time `json:"updated_at"`
}

// SharedTaskOrchestrator manages operations for shared_tasks_v4.
type SharedTaskOrchestrator struct {
	db *sql.DB
}

// NewSharedTaskOrchestrator creates a new orchestrator.
func NewSharedTaskOrchestrator(db *sql.DB) *SharedTaskOrchestrator {
	return &SharedTaskOrchestrator{db: db}
}

// CreateTask inserts a new task.
func (o *SharedTaskOrchestrator) CreateTask(ctx context.Context, task *SharedTask) error {
	if task.ID == "" {
		task.ID = uuid.New().String()
	}

	if task.Status == "" {
		task.Status = "PENDING"
	}

	if task.Priority == "" {
		task.Priority = "P2"
	}

	if task.Dependencies == nil {
		task.Dependencies = []string{}
	}

	depsJSON, err := json.Marshal(task.Dependencies)
	if err != nil {
		return fmt.Errorf("failed to marshal dependencies: %w", err)
	}

	query := `
		INSERT INTO shared_tasks_v4 (
			id, organization_id, title, description, status, agent_id,
			priority, payload, parent_plan_id, dependencies, created_at, updated_at
		) VALUES (
			$1, $2, $3, $4, $5, $6, $7, $8, $9, $10, CURRENT_TIMESTAMP, CURRENT_TIMESTAMP
		)
	`

	_, err = o.db.ExecContext(ctx, query,
		task.ID, task.OrganizationID, task.Title, task.Description, task.Status,
		task.AgentID, task.Priority, task.Payload, task.ParentPlanID, string(depsJSON),
	)
	if err != nil {
		return fmt.Errorf("failed to insert task: %w", err)
	}

	now := time.Now().UTC()
	task.CreatedAt = now
	task.UpdatedAt = now

	return nil
}

// parseTime handles parsing time from database which might be a string (SQLite) or time.Time (Postgres)
func parseTime(t interface{}) (time.Time, error) {
	if t == nil {
		return time.Time{}, nil
	}
	switch v := t.(type) {
	case time.Time:
		return v, nil
	case string:
		// Try parsing SQLite timestamp format
		parsed, err := time.Parse("2006-01-02 15:04:05-07:00", v)
		if err != nil {
			parsed, err = time.Parse("2006-01-02 15:04:05", v) // fallback without timezone
			if err != nil {
				parsed, err = time.Parse(time.RFC3339, v)
			}
		}
		return parsed, err
	case []byte:
		str := string(v)
		parsed, err := time.Parse("2006-01-02 15:04:05-07:00", str)
		if err != nil {
			parsed, err = time.Parse("2006-01-02 15:04:05", str) // fallback
			if err != nil {
				parsed, err = time.Parse(time.RFC3339, str)
			}
		}
		return parsed, err
	default:
		return time.Time{}, fmt.Errorf("unexpected type for time: %T", v)
	}
}

// GetTask retrieves a task by ID.
func (o *SharedTaskOrchestrator) GetTask(ctx context.Context, id string) (*SharedTask, error) {
	query := `
		SELECT id, organization_id, title, description, status, agent_id,
		       priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks_v4
		WHERE id = $1
	`
	row := o.db.QueryRowContext(ctx, query, id)

	var task SharedTask
	var depsJSON string
	var desc, agentID, payload, parentPlanID sql.NullString
	var createdAt, updatedAt interface{}

	err := row.Scan(
		&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Status, &agentID,
		&task.Priority, &payload, &parentPlanID, &depsJSON, &createdAt, &updatedAt,
	)
	if err != nil {
		if err == sql.ErrNoRows {
			return nil, fmt.Errorf("task not found")
		}
		return nil, fmt.Errorf("failed to scan task: %w", err)
	}

	if desc.Valid { task.Description = desc.String }
	if agentID.Valid { task.AgentID = agentID.String }
	if payload.Valid { task.Payload = payload.String }
	if parentPlanID.Valid { task.ParentPlanID = parentPlanID.String }

	if depsJSON != "" {
		if err := json.Unmarshal([]byte(depsJSON), &task.Dependencies); err != nil {
			return nil, fmt.Errorf("failed to unmarshal dependencies: %w", err)
		}
	}

	if parsed, err := parseTime(createdAt); err == nil {
		task.CreatedAt = parsed
	}
	if parsed, err := parseTime(updatedAt); err == nil {
		task.UpdatedAt = parsed
	}

	return &task, nil
}

// UpdateTaskStatus updates a task's status.
func (o *SharedTaskOrchestrator) UpdateTaskStatus(ctx context.Context, id string, status string) error {
	query := `
		UPDATE shared_tasks_v4
		SET status = $1, updated_at = CURRENT_TIMESTAMP
		WHERE id = $2
	`
	result, err := o.db.ExecContext(ctx, query, status, id)
	if err != nil {
		return fmt.Errorf("failed to update task status: %w", err)
	}

	rows, err := result.RowsAffected()
	if err != nil {
		return err
	}
	if rows == 0 {
		return fmt.Errorf("task not found")
	}

	return nil
}

// ListTasksByAgent retrieves tasks assigned to a specific agent.
func (o *SharedTaskOrchestrator) ListTasksByAgent(ctx context.Context, agentID string) ([]*SharedTask, error) {
	query := `
		SELECT id, organization_id, title, description, status, agent_id,
		       priority, payload, parent_plan_id, dependencies, created_at, updated_at
		FROM shared_tasks_v4
		WHERE agent_id = $1
	`
	rows, err := o.db.QueryContext(ctx, query, agentID)
	if err != nil {
		return nil, fmt.Errorf("failed to query tasks: %w", err)
	}
	defer rows.Close()

	var tasks []*SharedTask
	for rows.Next() {
		var task SharedTask
		var depsJSON string
		var desc, aID, payload, parentPlanID sql.NullString
		var createdAt, updatedAt interface{}

		err := rows.Scan(
			&task.ID, &task.OrganizationID, &task.Title, &desc, &task.Status, &aID,
			&task.Priority, &payload, &parentPlanID, &depsJSON, &createdAt, &updatedAt,
		)
		if err != nil {
			return nil, fmt.Errorf("failed to scan task: %w", err)
		}

		if desc.Valid { task.Description = desc.String }
		if aID.Valid { task.AgentID = aID.String }
		if payload.Valid { task.Payload = payload.String }
		if parentPlanID.Valid { task.ParentPlanID = parentPlanID.String }

		if depsJSON != "" {
			if err := json.Unmarshal([]byte(depsJSON), &task.Dependencies); err != nil {
				return nil, fmt.Errorf("failed to unmarshal dependencies: %w", err)
			}
		}

		if parsed, err := parseTime(createdAt); err == nil {
			task.CreatedAt = parsed
		}
		if parsed, err := parseTime(updatedAt); err == nil {
			task.UpdatedAt = parsed
		}

		tasks = append(tasks, &task)
	}

	if err := rows.Err(); err != nil {
		return nil, fmt.Errorf("rows iteration error: %w", err)
	}

	return tasks, nil
}
