package models

import (
	"time"
)

type SharedTaskV4 struct {
	ID             string    `json:"id" db:"id"`
	OrganizationID string    `json:"organization_id" db:"organization_id"`
	Title          string    `json:"title" db:"title"`
	Description    *string   `json:"description" db:"description"`
	Status         string    `json:"status" db:"status"`
	AgentID        *string   `json:"agent_id" db:"agent_id"`
	Priority       string    `json:"priority" db:"priority"`
	Payload        *string   `json:"payload" db:"payload"`
	ParentPlanID   *string   `json:"parent_plan_id" db:"parent_plan_id"`
	Dependencies   string    `json:"dependencies" db:"dependencies"` // JSON string
	CreatedAt      time.Time `json:"created_at" db:"created_at"`
	UpdatedAt      time.Time `json:"updated_at" db:"updated_at"`
}

type SubAgentJob struct {
	ID             string    `json:"id" db:"id"`
	OrganizationID string    `json:"organization_id" db:"organization_id"`
	ParentTaskID   string    `json:"parent_task_id" db:"parent_task_id"`
	Payload        *string   `json:"payload" db:"payload"` // JSON string
	Status         string    `json:"status" db:"status"`
	WorkerID       *string   `json:"worker_id" db:"worker_id"`
	CreatedAt      time.Time `json:"created_at" db:"created_at"`
	UpdatedAt      time.Time `json:"updated_at" db:"updated_at"`
}

type StateMachineAudit struct {
	ID         string    `json:"id" db:"id"`
	EntityID   string    `json:"entity_id" db:"entity_id"`
	EntityType string    `json:"entity_type" db:"entity_type"`
	FromState  string    `json:"from_state" db:"from_state"`
	ToState    string    `json:"to_state" db:"to_state"`
	AgentID    *string   `json:"agent_id" db:"agent_id"`
	Reason     *string   `json:"reason" db:"reason"`
	OccurredAt time.Time `json:"occurred_at" db:"occurred_at"`
}
