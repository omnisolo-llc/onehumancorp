package models

import (
	"time"
)

// Task represents a task in the shared queue.
type Task struct {
	ID              string    `json:"id"`
	MissionID       string    `json:"mission_id"`
	Title           string    `json:"title"`
	Description     string    `json:"description,omitempty"`
	AssignedAgentID string    `json:"assigned_agent_id,omitempty"`
	Status          string    `json:"status"`
	Priority        string    `json:"priority"`
	CreatedAt       time.Time `json:"created_at"`
	UpdatedAt       time.Time `json:"updated_at"`
	Result          string    `json:"result,omitempty"` // Included to capture the output, though it might not be in the base schema directly; often stored separately or we can just pass it during CompleteTask.
}

// TaskDependency represents a relationship where TaskID depends on DependsOnTaskID.
type TaskDependency struct {
	TaskID          string `json:"task_id"`
	DependsOnTaskID string `json:"depends_on_task_id"`
}
