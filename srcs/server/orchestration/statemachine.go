package orchestration

import (
	"context"
	"fmt"
	"log"
)

type State string

const (
	StatePending    State = "PENDING"
	StateReady      State = "READY"
	StateInProgress State = "IN_PROGRESS"
	StateBlocked    State = "BLOCKED"
	StateCompleted  State = "COMPLETED"
	StateFailed     State = "FAILED"
)

var allowedTransitions = map[State]map[State]bool{
	StatePending: {
		StateReady: true,
	},
	StateReady: {
		StateInProgress: true,
	},
	StateInProgress: {
		StateCompleted: true,
		StateBlocked:   true,
		StateFailed:    true,
	},
	StateBlocked: {
		StateInProgress: true,
		StateFailed:     true,
	},
}

type Repository interface {
	GetTaskState(ctx context.Context, taskID string) (State, error)
	UpdateTaskState(ctx context.Context, taskID string, newState State, agentID string) error
}

type StateMachine struct {
	repo Repository
	lock DistributedLock
}

func NewStateMachine(repo Repository, lock DistributedLock) *StateMachine {
	return &StateMachine{
		repo: repo,
		lock: lock,
	}
}

func (sm *StateMachine) TransitionToReady(ctx context.Context, taskID string) error {
	return sm.transition(ctx, taskID, StateReady, "")
}

func (sm *StateMachine) TransitionToInProgress(ctx context.Context, taskID string, agentID string) error {
	return sm.transition(ctx, taskID, StateInProgress, agentID)
}

func (sm *StateMachine) TransitionToCompleted(ctx context.Context, taskID string) error {
	return sm.transition(ctx, taskID, StateCompleted, "")
}

func (sm *StateMachine) TransitionToBlocked(ctx context.Context, taskID string) error {
	return sm.transition(ctx, taskID, StateBlocked, "")
}

func (sm *StateMachine) TransitionToFailed(ctx context.Context, taskID string) error {
	return sm.transition(ctx, taskID, StateFailed, "")
}

func (sm *StateMachine) transition(ctx context.Context, taskID string, newState State, agentID string) error {
	unlock, err := sm.lock.Acquire(ctx, taskID)
	if err != nil {
		return fmt.Errorf("failed to acquire lock for task %s: %w", taskID, err)
	}
	defer func() {
		if err := unlock(); err != nil {
			log.Printf("failed to release lock for task %s: %v", taskID, err)
		}
	}()

	currentState, err := sm.repo.GetTaskState(ctx, taskID)
	if err != nil {
		return fmt.Errorf("failed to get task %s state: %w", taskID, err)
	}

	if transitions, ok := allowedTransitions[currentState]; !ok || !transitions[newState] {
		return fmt.Errorf("invalid transition from %s to %s for task %s", currentState, newState, taskID)
	}

	if err := sm.repo.UpdateTaskState(ctx, taskID, newState, agentID); err != nil {
		return fmt.Errorf("failed to update task %s state to %s: %w", taskID, newState, err)
	}

	// Publish state change event to Teammate Mesh here (simplified for now)

	return nil
}
