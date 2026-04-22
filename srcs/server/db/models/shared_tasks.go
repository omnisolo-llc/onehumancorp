package models

import "time"

type SharedTask struct {
	ID              string    `json:"id" db:"id"`
	OrganizationID  string    `json:"organization_id" db:"organization_id"`
	EpicID          *string   `json:"epic_id" db:"epic_id"`
	ParentPlanID    *string   `json:"parent_plan_id" db:"parent_plan_id"`
	Title           string    `json:"title" db:"title"`
	Description     *string   `json:"description" db:"description"`
	Priority        *string   `json:"priority" db:"priority"`
	Status          string    `json:"status" db:"status"`
	AssignedAgentID *string   `json:"assigned_agent_id" db:"assigned_agent_id"`
	Dependencies    *string   `json:"dependencies" db:"dependencies"`
	CreatedAt       time.Time `json:"created_at" db:"created_at"`
	UpdatedAt       time.Time `json:"updated_at" db:"updated_at"`
}
