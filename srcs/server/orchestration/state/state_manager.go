package state

import (
	"context"
)

// Task represents a swarm task.
type Task struct {
	ID              string
	MissionID       string
	ParentPlanID    *string
	Dependencies    []string // JSON parsed
	Title           string
	Status          string
	AssignedAgentID *string
}

// StateManager orchestrates task transitions and assignments ensuring DAG sequence.
type StateManager interface {
	TransitionState(ctx context.Context, taskID, agentID, fromState, toState, reason string) error
	ClaimTask(ctx context.Context, agentID string) (*Task, error)
	MarkTaskCompleted(ctx context.Context, taskID string) error
	GetTaskStatus(ctx context.Context, taskID string) (string, error)
}
