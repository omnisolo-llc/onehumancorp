package orchestration

import (
	"time"
)

type TaskEntity struct {
	ID              string    `json:"id"`
	Title           string    `json:"title"`
	Description     string    `json:"description"`
	Status          string    `json:"status"`
	AssignedAgentID *string   `json:"assigned_agent_id"`
	Priority        int       `json:"priority"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`
}
