package models

import "time"

type SwarmTask struct {
	ID          string
	Title       string
	Description string
	Status      string
	Priority    string
	AgentID     *string
	CreatedAt   time.Time
	UpdatedAt   time.Time
}

type StateMachineTransition struct {
	ID            string
	TaskID        string
	FromState     string
	ToState       string
	TriggeredBy   string
	TransitionedAt time.Time
}

type TaskDependency struct {
	TaskID          string
	DependsOnTaskID string
}
