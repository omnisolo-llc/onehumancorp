package queue

import (
	"context"
	"errors"
	"testing"
	"time"
)

type mockHarness struct {
	shouldFail bool
}

func (m *mockHarness) Run(ctx context.Context, cmd string, args []string) error {
	if m.shouldFail {
		return errors.New("execution failed")
	}
	return nil
}

type mockQueue struct {
	tasks    []*Task
	acquired bool
	failed   bool
	complete bool
}

func (m *mockQueue) Enqueue(ctx context.Context, task *Task) error {
	m.tasks = append(m.tasks, task)
	return nil
}

func (m *mockQueue) Acquire(ctx context.Context) (*Task, error) {
	if !m.acquired && len(m.tasks) > 0 {
		m.acquired = true
		return m.tasks[0], nil
	}
	return nil, nil
}

func (m *mockQueue) Complete(ctx context.Context, taskID string) error {
	m.complete = true
	return nil
}

func (m *mockQueue) Fail(ctx context.Context, taskID string, retryAfter time.Duration) error {
	m.failed = true
	return nil
}

func TestWorkerPool_Success(t *testing.T) {
	q := &mockQueue{
		tasks: []*Task{
			{ID: "task-1", Command: "echo", Args: []string{"hello"}},
		},
	}
	h := &mockHarness{shouldFail: false}

	pool, err := NewWorkerPool(q, h, 1)
	if err != nil {
		t.Fatalf("Failed to create worker pool: %v", err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	pool.Start(ctx)

	time.Sleep(100 * time.Millisecond) // Allow worker to process
	cancel()
	pool.Stop()

	if !q.complete {
		t.Errorf("Expected task to be completed")
	}
}

func TestWorkerPool_Failure(t *testing.T) {
	q := &mockQueue{
		tasks: []*Task{
			{ID: "task-1", Command: "echo", Args: []string{"hello"}},
		},
	}
	h := &mockHarness{shouldFail: true}

	pool, err := NewWorkerPool(q, h, 1)
	if err != nil {
		t.Fatalf("Failed to create worker pool: %v", err)
	}

	ctx, cancel := context.WithCancel(context.Background())
	pool.Start(ctx)

	time.Sleep(100 * time.Millisecond) // Allow worker to process
	cancel()
	pool.Stop()

	if !q.failed {
		t.Errorf("Expected task to be failed and retried")
	}
}
