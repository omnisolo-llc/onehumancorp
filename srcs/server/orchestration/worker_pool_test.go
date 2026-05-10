package orchestration

import (
	"context"
	"encoding/json"
	"sync"
	"testing"
	"time"

	"onehumancorp/srcs/server/orchestration/harness"
)

// MockTaskQueue implements TaskQueue for testing
type MockTaskQueue struct {
	tasks       []*Task
	mu          sync.Mutex
	ackCount    int
	dequeueChan chan struct{} // To signal when a dequeue happens
}

func NewMockTaskQueue() *MockTaskQueue {
	return &MockTaskQueue{
		tasks:       make([]*Task, 0),
		dequeueChan: make(chan struct{}, 10),
	}
}

func (m *MockTaskQueue) Enqueue(ctx context.Context, task *Task) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.tasks = append(m.tasks, task)
	return nil
}

func (m *MockTaskQueue) Dequeue(ctx context.Context) (*Task, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	select {
	case <-ctx.Done():
		return nil, ctx.Err()
	default:
	}

	if len(m.tasks) == 0 {
		return nil, nil
	}

	task := m.tasks[0]
	m.tasks = m.tasks[1:]

	// Signal dequeue
	select {
	case m.dequeueChan <- struct{}{}:
	default:
	}

	return task, nil
}

func (m *MockTaskQueue) Acknowledge(ctx context.Context, taskID string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.ackCount++
	return nil
}

func (m *MockTaskQueue) FailTask(ctx context.Context, taskID string) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	return nil
}

func (m *MockTaskQueue) AckCount() int {
	m.mu.Lock()
	defer m.mu.Unlock()
	return m.ackCount
}

// WorkerMockHarness implements AgentHarness for testing
type WorkerMockHarness struct {
	runCount int
	lastCmd  string
}

func (m *WorkerMockHarness) RunAttempt(cmd string) (*harness.AttemptResult, error) {
	m.runCount++
	m.lastCmd = cmd
	return &harness.AttemptResult{
		Stdout:   "mock stdout",
		Stderr:   "",
		ExitCode: 0,
	}, nil
}

func (m *WorkerMockHarness) Compact() error {
	return nil
}

func (m *WorkerMockHarness) Reset() error {
	return nil
}

func TestWorkerPool_StartStop(t *testing.T) {
	queue := NewMockTaskQueue()
	pool := NewWorkerPool(queue, 2, 10*time.Millisecond)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	pool.Start(ctx)

	// Verify workers are active (they will be polling empty queue)
	time.Sleep(50 * time.Millisecond)

	pool.Stop()

	// If it doesn't block forever, Stop works
}

func TestWorkerPool_ProcessTask(t *testing.T) {
	queue := NewMockTaskQueue()
	pool := NewWorkerPool(queue, 2, 10*time.Millisecond)

	mockHarness := &WorkerMockHarness{}
	pool.SetHarnessFactory(func() (harness.AgentHarness, error) {
		return mockHarness, nil
	})

	task1 := &Task{ID: "task-1", Payload: json.RawMessage(`{"command": "mock-cmd"}`)}
	task2 := &Task{ID: "task-2", Payload: json.RawMessage(`{"command": "mock-cmd"}`)}

	queue.Enqueue(context.Background(), task1)
	queue.Enqueue(context.Background(), task2)

	ctx, cancel := context.WithCancel(context.Background())
	pool.Start(ctx)

	// Wait for tasks to be processed
	for i := 0; i < 2; i++ {
		select {
		case <-queue.dequeueChan:
		case <-time.After(1 * time.Second):
			t.Fatalf("timeout waiting for dequeue")
		}
	}

	// Give a little time for processing and ack
	time.Sleep(50 * time.Millisecond)

	pool.Stop()
	cancel()

	if mockHarness.runCount != 2 {
		t.Errorf("expected harness to run 2 times, got %d", mockHarness.runCount)
	}

	if queue.AckCount() != 2 {
		t.Errorf("expected 2 tasks to be acknowledged, got %d", queue.AckCount())
	}
}
