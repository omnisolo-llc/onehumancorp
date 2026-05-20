package orchestration

import (
	"time"
)

// Task represents a task in the Shared Task List.
type Task struct {
	ID              string    `json:"id"`
	Title           string    `json:"title"`
	Description     *string   `json:"description"`
	Status          string    `json:"status"` // e.g., 'PENDING', 'IN_PROGRESS', 'DONE', 'FAILED'
	AssignedAgentID *string   `json:"assigned_agent_id"`
	Priority        int       `json:"priority"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`
}
