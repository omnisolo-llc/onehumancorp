package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/orchestration/queue"
	"github.com/stretchr/testify/assert"
)

type mockTaskQueue struct {
	jobToReturn *queue.Job
	dequeueErr  error
	completed   string
	failed      string
	failReason  string
}

func (m *mockTaskQueue) Enqueue(ctx context.Context, job *queue.Job) error { return nil }
func (m *mockTaskQueue) Dequeue(ctx context.Context, roles []string) (*queue.Job, error) {
	j := m.jobToReturn
	m.jobToReturn = nil
	return j, m.dequeueErr
}
func (m *mockTaskQueue) Complete(ctx context.Context, jobID string) error {
	m.completed = jobID
	return nil
}
func (m *mockTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	m.failed = jobID
	m.failReason = reason
	return nil
}

type mockSpawner struct {
	errToReturn error
	spawnedTask *SharedTask
}

func (m *mockSpawner) Spawn(ctx context.Context, task *SharedTask) error {
	m.spawnedTask = task
	return m.errToReturn
}

func (m *mockSpawner) Monitor(ctx context.Context) error {
	return nil
}

func TestSubAgentWorker_ProcessJob_Success(t *testing.T) {
	mq := &mockTaskQueue{}
	ms := &mockSpawner{}
	worker := NewSubAgentWorker(mq, ms)

	payload := map[string]interface{}{"organization_id": "org123"}
	payloadBytes, _ := json.Marshal(payload)
	job := &queue.Job{
		ID:           "job1",
		ParentTaskID: "task1",
		Payload:      string(payloadBytes),
	}

	worker.processJob(context.Background(), job)

	assert.Equal(t, "job1", mq.completed)
	assert.Empty(t, mq.failed)
	assert.NotNil(t, ms.spawnedTask)
	assert.Equal(t, "task1", ms.spawnedTask.ID)
	assert.Equal(t, "org123", ms.spawnedTask.OrganizationID)
}

func TestSubAgentWorker_ProcessJob_Fail(t *testing.T) {
	mq := &mockTaskQueue{}
	ms := &mockSpawner{errToReturn: errors.New("spawn error")}
	worker := NewSubAgentWorker(mq, ms)

	payload := map[string]interface{}{"organization_id": "org123"}
	payloadBytes, _ := json.Marshal(payload)
	job := &queue.Job{
		ID:           "job2",
		ParentTaskID: "task2",
		Payload:      string(payloadBytes),
	}

	worker.processJob(context.Background(), job)

	assert.Empty(t, mq.completed)
	assert.Equal(t, "job2", mq.failed)
	assert.Equal(t, "spawn error", mq.failReason)
}

func TestSubAgentWorker_ProcessJob_InvalidPayload(t *testing.T) {
	mq := &mockTaskQueue{}
	ms := &mockSpawner{}
	worker := NewSubAgentWorker(mq, ms)

	job := &queue.Job{
		ID:           "job3",
		ParentTaskID: "task3",
		Payload:      "{invalid json}",
	}

	worker.processJob(context.Background(), job)

	assert.Empty(t, mq.completed)
	assert.Equal(t, "job3", mq.failed)
	assert.Contains(t, mq.failReason, "invalid payload")
}
