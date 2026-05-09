package orchestration

import (
	"encoding/json"
	"time"
)

type Task struct {
	ID        string           `json:"id" db:"id"`
	EpicID    *string          `json:"epic_id,omitempty" db:"epic_id"`
	Title     string           `json:"title" db:"title"`
	Status    string           `json:"status" db:"status"` // e.g., PENDING, IN_PROGRESS, COMPLETED, FAILED
	Payload   *json.RawMessage `json:"payload,omitempty" db:"payload"`
	CreatedAt time.Time        `json:"created_at" db:"created_at"`
	UpdatedAt time.Time        `json:"updated_at" db:"updated_at"`
	LockedBy  *string          `json:"locked_by,omitempty" db:"locked_by"`
	LockedAt  *time.Time       `json:"locked_at,omitempty" db:"locked_at"`
}

type TaskDependency struct {
	TaskID          string `json:"task_id" db:"task_id"`
	DependsOnTaskID string `json:"depends_on_task_id" db:"depends_on_task_id"`
}
