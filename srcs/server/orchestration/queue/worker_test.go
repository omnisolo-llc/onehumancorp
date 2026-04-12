package queue

import (
	"context"
	"errors"
	"testing"
	"time"
)

type mockQueue struct {
	jobsToReturn []*Job
	completed    []string
	failed       map[string]string
}

func (m *mockQueue) Enqueue(ctx context.Context, job *Job) error {
	return nil
}

func (m *mockQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	if len(m.jobsToReturn) > 0 {
		job := m.jobsToReturn[0]
		m.jobsToReturn = m.jobsToReturn[1:]
		return job, nil
	}
	return nil, nil
}

func (m *mockQueue) Complete(ctx context.Context, jobID string) error {
	m.completed = append(m.completed, jobID)
	return nil
}

func (m *mockQueue) Fail(ctx context.Context, jobID string, reason string) error {
	if m.failed == nil {
		m.failed = make(map[string]string)
	}
	m.failed[jobID] = reason
	return nil
}

func TestWorker_Success(t *testing.T) {
	mq := &mockQueue{
		jobsToReturn: []*Job{
			{ID: "job-1"},
		},
	}

	handlerCalled := make(chan struct{})
	handler := func(ctx context.Context, job *Job) error {
		if job.ID != "job-1" {
			t.Errorf("expected job-1, got %s", job.ID)
		}
		close(handlerCalled)
		return nil
	}

	worker := NewWorker(mq, []string{"test-role"}, handler, 10*time.Millisecond)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.Start(ctx)

	select {
	case <-handlerCalled:
	case <-time.After(1 * time.Second):
		t.Fatal("handler was not called in time")
	}

	// small wait to ensure queue.Complete is called
	time.Sleep(10 * time.Millisecond)

	if len(mq.completed) != 1 || mq.completed[0] != "job-1" {
		t.Errorf("expected job-1 to be completed, got %v", mq.completed)
	}
}

func TestWorker_Failure(t *testing.T) {
	mq := &mockQueue{
		jobsToReturn: []*Job{
			{ID: "job-2"},
		},
	}

	expectedErr := errors.New("handler error")
	handlerCalled := make(chan struct{})
	handler := func(ctx context.Context, job *Job) error {
		close(handlerCalled)
		return expectedErr
	}

	worker := NewWorker(mq, []string{"test-role"}, handler, 10*time.Millisecond)

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	go worker.Start(ctx)

	select {
	case <-handlerCalled:
	case <-time.After(1 * time.Second):
		t.Fatal("handler was not called in time")
	}

	// small wait to ensure queue.Fail is called
	time.Sleep(10 * time.Millisecond)

	if len(mq.failed) != 1 || mq.failed["job-2"] != expectedErr.Error() {
		t.Errorf("expected job-2 to fail with error, got %v", mq.failed)
	}
}
