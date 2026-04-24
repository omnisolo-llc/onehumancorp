package repository

import (
	"time"
)

type Task struct {
	ID                string    `json:"id" db:"id"`
	OrganizationID    string    `json:"organization_id" db:"organization_id"`
	ParentTaskID      *string   `json:"parent_task_id" db:"parent_task_id"`
	Title             string    `json:"title" db:"title"`
	Description       string    `json:"description" db:"description"`
	Status            string    `json:"status" db:"status"`
	AssignedAgentRole string    `json:"assigned_agent_role" db:"assigned_agent_role"`
	CreatedAt         time.Time `json:"created_at" db:"created_at"`
	UpdatedAt         time.Time `json:"updated_at" db:"updated_at"`
}

type TaskDependency struct {
	TaskID           string `json:"task_id" db:"task_id"`
	DependsOnTaskID  string `json:"depends_on_task_id" db:"depends_on_task_id"`
}
