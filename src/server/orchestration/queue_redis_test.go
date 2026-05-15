package orchestration

import (
	"context"
	"encoding/json"
	"testing"
	"time"

	"github.com/redis/rueidis"
	"github.com/redis/rueidis/mock"
	gomock "go.uber.org/mock/gomock"
)

func TestPgRedisQueue_EnqueueDequeue(t *testing.T) {
	ctx := context.Background()
	ctrl := gomock.NewController(t)
	defer ctrl.Finish()

	mockClient := mock.NewClient(ctrl)

	q := NewPgRedisQueue(mockClient)

	job := SubAgentJob{
		ID:           "test-job",
		ParentTaskID: "parent-task",
		Payload:      json.RawMessage(`{"key":"value"}`),
		ScheduledAt:  time.Now(),
	}

	// Enqueue test
	mockClient.EXPECT().Do(ctx, gomock.Any()).Return(mock.Result(mock.RedisInt64(1)))
	err := q.Enqueue(ctx, job)
	if err != nil {
		t.Fatalf("Failed to enqueue: %v", err)
	}

	// Dequeue test (success)
	data, _ := json.Marshal(job)
	mockClient.EXPECT().Do(ctx, gomock.Any()).Return(mock.Result(mock.RedisArray(mock.RedisString(string(data)))))
	mockClient.EXPECT().Do(ctx, gomock.Any()).Return(mock.Result(mock.RedisInt64(1)))

	dequeuedJob, err := q.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Failed to dequeue: %v", err)
	}
	if dequeuedJob == nil {
		t.Fatal("Expected job, got nil")
	}
	if dequeuedJob.ID != job.ID {
		t.Fatalf("Expected job ID %s, got %s", job.ID, dequeuedJob.ID)
	}

	// Dequeue test (race condition - lost the job)
	mockClient.EXPECT().Do(ctx, gomock.Any()).Return(mock.Result(mock.RedisArray(mock.RedisString(string(data)))))
	mockClient.EXPECT().Do(ctx, gomock.Any()).Return(mock.Result(mock.RedisInt64(0)))

	racedJob, err := q.Dequeue(ctx)
	if err != nil {
		t.Fatalf("Expected no error on race, got: %v", err)
	}
	if racedJob != nil {
		t.Fatal("Expected nil job when ZREM returns 0 (race lost)")
	}
}

// Ensure mock matches interface properly
var _ rueidis.Client = (*mock.Client)(nil)
