package domain

import (
	"encoding/json"
	"time"
)

// SharedTask represents the shared_tasks database table for KAIROS Orchestration
type SharedTask struct {
	ID              string          `json:"id" db:"id"`
	OrganizationID  string          `json:"organization_id" db:"organization_id"`
	ParentPlanID    *string         `json:"parent_plan_id" db:"parent_plan_id"`
	Title           string          `json:"title" db:"title"`
	Description     *string         `json:"description" db:"description"`
	AgentID         *string         `json:"agent_id" db:"agent_id"`
	AssignedAgentID *string         `json:"assigned_agent_id" db:"assigned_agent_id"`
	Status          string          `json:"status" db:"status"`
	Payload         json.RawMessage `json:"payload" db:"payload"`
	Dependencies    []string        `json:"dependencies" db:"dependencies"`
	CreatedAt       time.Time       `json:"created_at" db:"created_at"`
	UpdatedAt       time.Time       `json:"updated_at" db:"updated_at"`
}

func (s *SharedTask) IsPending() bool {
	return s.Status == "PENDING"
}

func (s *SharedTask) Assign(agentID string) {
	s.AgentID = &agentID
	s.AssignedAgentID = &agentID
	s.Status = "IN_PROGRESS"
}

func (s *SharedTask) Complete(payload json.RawMessage) {
	s.Status = "COMPLETED"
	if payload != nil {
		s.Payload = payload
	}
}
