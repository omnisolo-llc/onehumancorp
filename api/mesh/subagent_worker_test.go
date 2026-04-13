package mesh

import (
	"context"
	"errors"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
)

type mockQueue struct {
	jobs      []*queue.Job
	mu        sync.Mutex
	completed []string
	failed    map[string]string
}

func (m *mockQueue) Enqueue(ctx context.Context, job *queue.Job) error {
	m.mu.Lock()
	defer m.mu.Unlock()
	m.jobs = append(m.jobs, job)
	return nil
}

func (m *mockQueue) Dequeue(ctx context.Context, roles []string) (*queue.Job, error) {
	m.mu.Lock()
	defer m.mu.Unlock()
	if len(m.jobs) == 0 {
		return nil, nil
	}
	job := m.jobs[0]
	m.jobs = m.jobs[1:]
	return job, nil
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

type mockExecutor struct {
	executed []*queue.Job
	err      error
}

func (m *mockExecutor) Execute(ctx context.Context, job *queue.Job) error {
	m.executed = append(m.executed, job)
	return m.err
}

func TestSubAgentWorker_poll(t *testing.T) {
	q := &mockQueue{
		jobs: []*queue.Job{
			{ID: "job-1", AgentRole: "role-1"},
		},
	}
	exec := &mockExecutor{}
	worker := NewSubAgentWorker(q, exec, []string{"role-1"}, 100*time.Millisecond)

	t.Run("SuccessfulExecution", func(t *testing.T) {
		worker.poll(context.Background())

		// Wait for async execution
		time.Sleep(50 * time.Millisecond)

		if len(exec.executed) != 1 {
			t.Errorf("expected 1 execution, got %d", len(exec.executed))
		}
		if exec.executed[0].ID != "job-1" {
			t.Errorf("expected job-1, got %s", exec.executed[0].ID)
		}

		q.mu.Lock()
		if len(q.completed) != 1 || q.completed[0] != "job-1" {
			t.Errorf("job not marked complete")
		}
		q.mu.Unlock()
	})

	t.Run("FailedExecution", func(t *testing.T) {
		q.jobs = append(q.jobs, &queue.Job{ID: "job-2", AgentRole: "role-1"})
		exec.err = errors.New("execution failed")

		worker.poll(context.Background())
		time.Sleep(50 * time.Millisecond)

		q.mu.Lock()
		if q.failed["job-2"] != "execution failed" {
			t.Errorf("job not marked failed with correct reason")
		}
		q.mu.Unlock()
	})
}
