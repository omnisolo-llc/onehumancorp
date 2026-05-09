package orchestration

import (
	"context"
	"encoding/json"
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
	"github.com/stretchr/testify/mock"
)

type MockTaskQueue struct {
	mock.Mock
}

func (m *MockTaskQueue) Enqueue(ctx context.Context, task *SubAgentTask) error {
	args := m.Called(ctx, task)
	return args.Error(0)
}

func (m *MockTaskQueue) Dequeue(ctx context.Context, workerID string) (*SubAgentTask, error) {
	args := m.Called(ctx, workerID)
	if t := args.Get(0); t != nil {
		return t.(*SubAgentTask), args.Error(1)
	}
	return nil, args.Error(1)
}

func (m *MockTaskQueue) Acknowledge(ctx context.Context, taskID string, status string) error {
	args := m.Called(ctx, taskID, status)
	return args.Error(0)
}

func TestWorkerPool_ProcessTask_Success(t *testing.T) {
	mockQueue := new(MockTaskQueue)
	resolver := NewHarnessResolver()
	wp := NewWorkerPool(mockQueue, resolver, 1)

	task := &SubAgentTask{
		ID:      "task-1",
		Payload: json.RawMessage(`{"agent_id": "agent-1", "command": "echo 'test'"}`),
	}

	err := wp.processTask(context.Background(), task)
	assert.NoError(t, err)
}

func TestWorkerPool_ProcessTask_InvalidPayload(t *testing.T) {
	mockQueue := new(MockTaskQueue)
	resolver := NewHarnessResolver()
	wp := NewWorkerPool(mockQueue, resolver, 1)

	task := &SubAgentTask{
		ID:      "task-1",
		Payload: json.RawMessage(`invalid json`),
	}

	err := wp.processTask(context.Background(), task)
	assert.Error(t, err)
}

func TestWorkerPool_WorkerLoop(t *testing.T) {
	mockQueue := new(MockTaskQueue)
	resolver := NewHarnessResolver()
	wp := NewWorkerPool(mockQueue, resolver, 1)

	task := &SubAgentTask{
		ID:      "task-1",
		Payload: json.RawMessage(`{"agent_id": "agent-1", "command": "echo 'test'"}`),
	}

	// Make dequeue return task once, then block
	mockQueue.On("Dequeue", mock.Anything, mock.AnythingOfType("string")).Return(task, nil).Once()
	mockQueue.On("Dequeue", mock.Anything, mock.AnythingOfType("string")).Return((*SubAgentTask)(nil), nil)
	mockQueue.On("Acknowledge", mock.Anything, "task-1", "COMPLETED").Return(nil).Once()

	ctx, cancel := context.WithCancel(context.Background())
	wp.Start(ctx)

	// wait for processing
	time.Sleep(500 * time.Millisecond)

	cancel()
	wp.Stop()

	mockQueue.AssertExpectations(t)
}

func TestWorkerPool_WorkerLoop_Failure(t *testing.T) {
	mockQueue := new(MockTaskQueue)
	resolver := NewHarnessResolver()
	wp := NewWorkerPool(mockQueue, resolver, 1)

	task := &SubAgentTask{
		ID:      "task-1",
		Payload: json.RawMessage(`invalid payload`),
	}

	// Make dequeue return task once, then block
	mockQueue.On("Dequeue", mock.Anything, mock.AnythingOfType("string")).Return(task, nil).Once()
	mockQueue.On("Dequeue", mock.Anything, mock.AnythingOfType("string")).Return((*SubAgentTask)(nil), nil)
	mockQueue.On("Acknowledge", mock.Anything, "task-1", "FAILED").Return(nil).Once()

	ctx, cancel := context.WithCancel(context.Background())
	wp.Start(ctx)

	// wait for processing
	time.Sleep(500 * time.Millisecond)

	cancel()
	wp.Stop()

	mockQueue.AssertExpectations(t)
}

func TestWorkerPool_WorkerLoop_DequeueError(t *testing.T) {
	mockQueue := new(MockTaskQueue)
	resolver := NewHarnessResolver()
	wp := NewWorkerPool(mockQueue, resolver, 1)

	mockQueue.On("Dequeue", mock.Anything, mock.AnythingOfType("string")).Return((*SubAgentTask)(nil), errors.New("db error")).Once()
	mockQueue.On("Dequeue", mock.Anything, mock.AnythingOfType("string")).Return((*SubAgentTask)(nil), nil)

	ctx, cancel := context.WithCancel(context.Background())
	wp.Start(ctx)

	// wait for processing
	time.Sleep(500 * time.Millisecond)

	cancel()
	wp.Stop()

	mockQueue.AssertExpectations(t)
}

// Ensure mock queue satisfies TaskQueue
var _ TaskQueue = (*MockTaskQueue)(nil)
