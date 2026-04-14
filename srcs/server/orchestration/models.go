package orchestration

import "time"

type SharedTaskDecomposition struct {
	ID              string
	OrganizationID  string
	Title           string
	Description     *string
	Status          string
	AssignedAgentID *string
	Priority        string
	Payload         *string
	ParentPlanID    *string
	Dependencies    string
	LockedUntil     *time.Time
	CreatedAt       time.Time
	UpdatedAt       time.Time
}
