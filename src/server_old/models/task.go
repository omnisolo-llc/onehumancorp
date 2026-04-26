package models

import "time"

// Task represents a task in the shared queue.
type Task struct {
	ID              string
	MissionID       string
	ParentPlanID    string
	Dependencies    []string
	Title           string
	Description     string
	Priority        string
	Status          string // PENDING, READY, IN_PROGRESS, COMPLETED, BLOCKED, FAILED
	AssignedAgentID string
	Payload         string
	CreatedAt       time.Time
	UpdatedAt       time.Time

	ActionRisk      string   `json:"action_risk,omitempty"`
	ApprovalStatus  string   `json:"approval_status,omitempty"`
	ProposedContent string   `json:"proposed_content,omitempty"`
}

// TaskDependency represents a dependency relationship between tasks.
type TaskDependency struct {
	TaskID          string
	DependsOnTaskID string
}

// SharedTask is used primarily by TaskManager for multi-tenant queue operations.
type SharedTask struct {
	ID              string   `json:"id"`
	OrganizationID  string   `json:"organization_id"`
	ParentPlanID    string   `json:"parent_plan_id"`
	Dependencies    []string `json:"dependencies"`
	Title           string   `json:"title"`
	Description     string   `json:"description,omitempty"`
	Status          string   `json:"status"`
	AssignedAgentID string   `json:"assigned_agent_id,omitempty"`
	Priority        string   `json:"priority"`
	Payload         string   `json:"payload"`
	LockedUntil     *time.Time `json:"locked_until,omitempty"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`

	ActionRisk      string   `json:"action_risk,omitempty"`
	ApprovalStatus  string   `json:"approval_status,omitempty"`
	ProposedContent string   `json:"proposed_content,omitempty"`
}
