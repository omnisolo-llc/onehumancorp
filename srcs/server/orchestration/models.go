package orchestration

import "time"

type SharedTaskDecomposition struct {
    ID              string     `json:"id"`
    OrganizationID  string     `json:"organization_id"`
    Title           string     `json:"title"`
    Description     *string    `json:"description,omitempty"`
    Status          string     `json:"status"`
    AssignedAgentID *string    `json:"assigned_agent_id,omitempty"`
    Priority        string     `json:"priority"`
    Payload         []byte     `json:"payload,omitempty"`
    ParentPlanID    *string    `json:"parent_plan_id,omitempty"`
    Dependencies    []byte     `json:"dependencies"`
    LockedUntil     *time.Time `json:"locked_until,omitempty"`
    CreatedAt       time.Time  `json:"created_at"`
    UpdatedAt       time.Time  `json:"updated_at"`
}
