package tasks

import "time"

type TaskStatus string

const (
	TaskStatusPending TaskStatus = "PENDING"
	TaskStatusClaimed TaskStatus = "CLAIMED"
	TaskStatusDone    TaskStatus = "DONE"
	TaskStatusFailed  TaskStatus = "FAILED"
)

type SharedTaskDecomposition struct {
	ID              string     `json:"id"`
	OrganizationID  string     `json:"organization_id"`
	Title           string     `json:"title"`
	Description     string     `json:"description"`
	Status          TaskStatus `json:"status"`
	AssignedAgentID *string    `json:"assigned_agent_id"`
	Priority        string     `json:"priority"`
	Payload         []byte     `json:"payload"`
	ParentPlanID    *string    `json:"parent_plan_id"`
	Dependencies    []byte     `json:"dependencies"`
	LockedUntil     *time.Time `json:"locked_until"`
	CreatedAt       time.Time  `json:"created_at"`
	UpdatedAt       time.Time  `json:"updated_at"`
}
