package models

import "time"

// Task represents a task in the shared queue.
type Task struct {
	ID              string
	MissionID       string
	Title           string
	Description     string
	Priority        string
	Status          string // PENDING, READY, IN_PROGRESS, COMPLETED, BLOCKED, FAILED
	AssignedAgentID string
	Payload         string
	CreatedAt       time.Time
	UpdatedAt       time.Time
}

// TaskDependency represents a dependency relationship between tasks.
type TaskDependency struct {
	TaskID          string
	DependsOnTaskID string
}
