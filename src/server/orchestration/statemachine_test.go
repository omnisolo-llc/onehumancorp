package orchestration

import (
	"context"
	"fmt"
	"sync"
	"testing"
)

type MockRepository struct {
	mu    sync.Mutex
	tasks map[string]State
}

func NewMockRepository() *MockRepository {
	return &MockRepository{
		tasks: make(map[string]State),
	}
}

func (m *MockRepository) GetTaskState(ctx context.Context, taskID string) (State, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	state, ok := m.tasks[taskID]
	if !ok {
		return StatePending, nil // Default for mock
	}
	return state, nil
}

func (m *MockRepository) UpdateTaskState(ctx context.Context, taskID string, newState State, agentID string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.tasks[taskID] = newState
	return nil
}

func TestStateMachine_ValidTransitions(t *testing.T) {
	repo := NewMockRepository()
	lock := NewStandaloneLock()
	sm := NewStateMachine(repo, lock)

	ctx := context.Background()
	taskID := "task1"

	// Pending -> Ready
	err := sm.TransitionToReady(ctx, taskID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	state, _ := repo.GetTaskState(ctx, taskID)
	if state != StateReady {
		t.Fatalf("expected state READY, got %v", state)
	}

	// Ready -> InProgress
	err = sm.TransitionToInProgress(ctx, taskID, "agent1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	state, _ = repo.GetTaskState(ctx, taskID)
	if state != StateInProgress {
		t.Fatalf("expected state IN_PROGRESS, got %v", state)
	}

	// InProgress -> Blocked
	err = sm.TransitionToBlocked(ctx, taskID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	state, _ = repo.GetTaskState(ctx, taskID)
	if state != StateBlocked {
		t.Fatalf("expected state BLOCKED, got %v", state)
	}

	// Blocked -> InProgress
	err = sm.TransitionToInProgress(ctx, taskID, "agent1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	// InProgress -> Completed
	err = sm.TransitionToCompleted(ctx, taskID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	state, _ = repo.GetTaskState(ctx, taskID)
	if state != StateCompleted {
		t.Fatalf("expected state COMPLETED, got %v", state)
	}
}

func TestStateMachine_InvalidTransition(t *testing.T) {
	repo := NewMockRepository()
	lock := NewStandaloneLock()
	sm := NewStateMachine(repo, lock)

	ctx := context.Background()
	taskID := "task2"

	// Pending -> InProgress (Invalid)
	err := sm.TransitionToInProgress(ctx, taskID, "agent1")
	if err == nil {
		t.Fatalf("expected error for invalid transition")
	}
}

func TestStateMachine_ConcurrentTransitions(t *testing.T) {
	repo := NewMockRepository()
	lock := NewStandaloneLock()
	sm := NewStateMachine(repo, lock)

	ctx := context.Background()
	taskID := "task3"

	// Move to Ready first
	_ = sm.TransitionToReady(ctx, taskID)

	var wg sync.WaitGroup
	errs := make(chan error, 2)

	// Try to transition to InProgress concurrently
	for i := 0; i < 2; i++ {
		wg.Add(1)
		go func(agentID string) {
			defer wg.Done()
			errs <- sm.TransitionToInProgress(ctx, taskID, agentID)
		}(fmt.Sprintf("agent%d", i))
	}

	wg.Wait()
	close(errs)

	var successCount int
	var errorCount int

	for err := range errs {
		if err == nil {
			successCount++
		} else {
			errorCount++
		}
	}

	if successCount != 1 {
		t.Fatalf("expected exactly 1 successful transition, got %d", successCount)
	}
	if errorCount != 1 {
		t.Fatalf("expected exactly 1 failed transition, got %d", errorCount)
	}
}

func TestStateMachine_TransitionToFailed(t *testing.T) {
	repo := NewMockRepository()
	lock := NewStandaloneLock()
	sm := NewStateMachine(repo, lock)

	ctx := context.Background()
	taskID := "task4"

	// Pending -> Ready
	_ = sm.TransitionToReady(ctx, taskID)
	// Ready -> InProgress
	_ = sm.TransitionToInProgress(ctx, taskID, "agent1")

	// InProgress -> Failed
	err := sm.TransitionToFailed(ctx, taskID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	state, _ := repo.GetTaskState(ctx, taskID)
	if state != StateFailed {
		t.Fatalf("expected state FAILED, got %v", state)
	}
}
