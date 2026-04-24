package models

import (
	"time"
)

type SwarmTask struct {
	ID          string    `json:"id" db:"id"`
	Title       string    `json:"title" db:"title"`
	Description string    `json:"description" db:"description"`
	Status      string    `json:"status" db:"status"`
	Priority    string    `json:"priority" db:"priority"`
	AgentID     *string   `json:"agent_id" db:"agent_id"`
	CreatedAt   time.Time `json:"created_at" db:"created_at"`
	UpdatedAt   time.Time `json:"updated_at" db:"updated_at"`
}

type StateMachineTransition struct {
	ID             string    `json:"id" db:"id"`
	TaskID         string    `json:"task_id" db:"task_id"`
	FromState      *string   `json:"from_state" db:"from_state"`
	ToState        string    `json:"to_state" db:"to_state"`
	TriggeredBy    *string   `json:"triggered_by" db:"triggered_by"`
	TransitionedAt time.Time `json:"transitioned_at" db:"transitioned_at"`
}

type TaskDependency struct {
	TaskID           string `json:"task_id" db:"task_id"`
	DependsOnTaskID  string `json:"depends_on_task_id" db:"depends_on_task_id"`
}
