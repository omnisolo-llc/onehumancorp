package kairos

import (
	"time"

	"github.com/google/uuid"
)

type Mission struct {
	ID              uuid.UUID `json:"id" db:"id"`
	EpicID          *uuid.UUID `json:"epic_id,omitempty" db:"epic_id"`
	Title           string    `json:"title" db:"title"`
	Status          string    `json:"status" db:"status"`
	AssignedAgentID *string   `json:"assigned_agent_id,omitempty" db:"assigned_agent_id"`
	CreatedAt       time.Time `json:"created_at" db:"created_at"`
	UpdatedAt       time.Time `json:"updated_at" db:"updated_at"`
}

type MissionDependency struct {
	TaskID          uuid.UUID `json:"task_id" db:"task_id"`
	DependsOnTaskID uuid.UUID `json:"depends_on_task_id" db:"depends_on_task_id"`
}

type AutodreamVector struct {
	ID        uuid.UUID   `json:"id" db:"id"`
	TaskID    *uuid.UUID  `json:"task_id,omitempty" db:"task_id"`
	Content   string      `json:"content" db:"content"`
	Embedding []float32   `json:"embedding,omitempty" db:"embedding"`
	CreatedAt time.Time   `json:"created_at" db:"created_at"`
}
