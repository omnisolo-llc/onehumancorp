package domain

import (
	"encoding/json"
	"time"
)

// SharedTask represents the shared_tasks database table for KAIROS Orchestration
type SharedTask struct {
	ID        string          `json:"id" db:"id"`
	AgentID   *string         `json:"agent_id" db:"agent_id"`
	Status    string          `json:"status" db:"status"`
	Payload   json.RawMessage `json:"payload" db:"payload"`
	CreatedAt time.Time       `json:"created_at" db:"created_at"`
}

func (s *SharedTask) IsPending() bool {
	return s.Status == "PENDING"
}

func (s *SharedTask) Assign(agentID string) {
	s.AgentID = &agentID
	s.Status = "IN_PROGRESS"
}

func (s *SharedTask) Complete(payload json.RawMessage) {
	s.Status = "COMPLETED"
	if payload != nil {
		s.Payload = payload
	}
}
