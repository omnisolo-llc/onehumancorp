package models

import (
	"database/sql"
	"time"
)

// Task represents a task in the shared_tasks / swarm_tasks queue
type Task struct {
	ID              string       `json:"id"`
	MissionID       string       `json:"mission_id"`
	Title           string       `json:"title"`
	Description     string       `json:"description"`
	Priority        string       `json:"priority"`
	Status          string       `json:"status"` // PENDING, READY, IN_PROGRESS, COMPLETED, BLOCKED, FAILED
	AssignedAgentID string       `json:"assigned_agent_id,omitempty"`
	LockedUntil     sql.NullTime `json:"-"`
	Payload         string       `json:"payload,omitempty"`
	Capabilities    []string     `json:"capabilities,omitempty"`
	CreatedAt       time.Time    `json:"created_at"`
	UpdatedAt       time.Time    `json:"updated_at"`
}

// TaskDependency represents a directed dependency between two tasks.
type TaskDependency struct {
	TaskID          string `json:"task_id"`
	DependsOnTaskID string `json:"depends_on_task_id"`
}
