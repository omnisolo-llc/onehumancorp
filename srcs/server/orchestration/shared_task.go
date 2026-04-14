package orchestration

import "time"

// SharedTask represents a shared task distributed across agents.
type SharedTask struct {
	ID              string     `json:"id"`
	OrganizationID  string     `json:"organization_id"`
	ParentPlanID    string     `json:"parent_plan_id"`
	Dependencies    []string   `json:"dependencies"`
	Title           string     `json:"title"`
	Description     string     `json:"description,omitempty"`
	AssignedAgentID string     `json:"assigned_agent_id,omitempty"`
	Status          string     `json:"status"` // PENDING, IN_PROGRESS, COMPLETED, FAILED, BLOCKED
	Priority        string     `json:"priority"`
	Payload         string     `json:"payload"`
	LockedUntil     *time.Time `json:"locked_until,omitempty"`
	CreatedAt       time.Time  `json:"created_at"`
	UpdatedAt       time.Time  `json:"updated_at"`
}
