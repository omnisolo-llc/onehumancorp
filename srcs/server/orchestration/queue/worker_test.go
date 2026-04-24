package queue

import (
	"context"
	"errors"
	"sync"
	"testing"
	"time"
)

// mockQueue is a simple in-memory queue for testing the worker loop
type mockQueue struct {
	mu           sync.Mutex
	jobsToReturn []*Job
	dequeueErr   error
	completed    []string
	failed       map[string]string
}

func (m *mockQueue) Enqueue(ctx context.Context, job *Job) error { return nil }

func (m *mockQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.dequeueErr != nil {
		return nil, m.dequeueErr
	}
	if len(m.jobsToReturn) > 0 {
		j := m.jobsToReturn[0]
		m.jobsToReturn = m.jobsToReturn[1:]
		return j, nil
	}
	return nil, nil
}

func (m *mockQueue) Complete(ctx context.Context, jobID string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	m.completed = append(m.completed, jobID)
	return nil
}

func (m *mockQueue) Fail(ctx context.Context, jobID string, reason string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	if m.failed == nil {
		m.failed = make(map[string]string)
	}
	m.failed[jobID] = reason
	return nil
}

func (m *mockQueue) getCompleted() []string {
	m.mu.Lock()
	defer m.mu.Unlock()
	res := make([]string, len(m.completed))
	copy(res, m.completed)
	return res
}

func (m *mockQueue) getFailed() map[string]string {
	m.mu.Lock()
	defer m.mu.Unlock()
	res := make(map[string]string)
	for k, v := range m.failed {
		res[k] = v
	}
	return res
}

func TestWorkerLoop_Success(t *testing.T) {
	mq := &mockQueue{
		jobsToReturn: []*Job{{ID: "job-1", AgentRole: "test-role"}},
	}

	done := make(chan struct{})

	handler := func(ctx context.Context, job *Job) error {
		if job.ID != "job-1" {
			t.Errorf("Expected job ID 'job-1', got '%s'", job.ID)
		}
		close(done)
		return nil
	}

	worker := NewWorker(mq, []string{"test-role"}, handler)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.Start(ctx)

	select {
	case <-done:
		// wait a tiny bit to ensure complete has run
		time.Sleep(10 * time.Millisecond)
	case <-time.After(2 * time.Second):
		t.Fatal("Test timed out waiting for handler")
	}

	completed := mq.getCompleted()
	if len(completed) != 1 || completed[0] != "job-1" {
		t.Fatalf("Expected job-1 to be completed, got %v", completed)
	}
}

func TestWorkerLoop_Failure(t *testing.T) {
	mq := &mockQueue{
		jobsToReturn: []*Job{{ID: "job-2", AgentRole: "test-role"}},
	}

	done := make(chan struct{})

	handler := func(ctx context.Context, job *Job) error {
		close(done)
		return errors.New("processing failed")
	}

	worker := NewWorker(mq, []string{"test-role"}, handler)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.Start(ctx)

	select {
	case <-done:
		// wait a tiny bit to ensure fail has run
		time.Sleep(10 * time.Millisecond)
	case <-time.After(2 * time.Second):
		t.Fatal("Test timed out waiting for handler")
	}

	completed := mq.getCompleted()
	if len(completed) != 0 {
		t.Fatalf("Expected 0 completed jobs, got %v", completed)
	}

	failed := mq.getFailed()
	if len(failed) != 1 || failed["job-2"] != "processing failed" {
		t.Fatalf("Expected job-2 to be failed with 'processing failed', got %v", failed)
	}
}
