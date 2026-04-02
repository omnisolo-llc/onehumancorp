package models

import (
	"database/sql"
	"time"
)

// Task models the shared_tasks and swarm_tasks structures.
type Task struct {
	ID              string       `json:"id"`
	MissionID       string       `json:"mission_id"`
	Title           string       `json:"title"`
	Description     string       `json:"description"`
	AssignedAgentID string       `json:"assigned_agent_id"`
	Status          string       `json:"status"` // PENDING, READY, IN_PROGRESS, COMPLETED, BLOCKED, FAILED
	Priority        string       `json:"priority"`
	Payload         string       `json:"payload"`
	LockedUntil     sql.NullTime `json:"locked_until"`
	CreatedAt       time.Time    `json:"created_at"`
	UpdatedAt       time.Time    `json:"updated_at"`
}

// TaskDependency models the task_dependencies structure.
type TaskDependency struct {
	TaskID          string `json:"task_id"`
	DependsOnTaskID string `json:"depends_on_task_id"`
}
