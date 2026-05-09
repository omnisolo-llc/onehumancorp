package orchestration

import (
	"encoding/json"
	"time"
)

// Task represents a task in the Shared Task List (ohc_tasks table).
type Task struct {
	ID              string           `json:"id"`
	TenantID        string           `json:"tenant_id"`
	Title           string           `json:"title"`
	Description     *string          `json:"description,omitempty"`
	Status          string           `json:"status"`
	AssignedAgentID *string          `json:"assigned_agent_id,omitempty"`
	Priority        int              `json:"priority"`
	Payload         *json.RawMessage `json:"payload,omitempty"`
	ParentTaskID    *string          `json:"parent_task_id,omitempty"`
	WorkflowState   *string          `json:"workflow_state,omitempty"`
	CreatedAt       time.Time        `json:"created_at"`
	UpdatedAt       time.Time        `json:"updated_at"`
}