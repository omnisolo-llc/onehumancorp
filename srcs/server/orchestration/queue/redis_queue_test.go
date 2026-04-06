package queue

import (
	"context"
	"testing"
	"time"

	"github.com/redis/rueidis"
)

// mockRedisClient safely satisfies rueidis.Client and rueidis.Builder without networking.
type mockRedisClient struct {
	rueidis.Client
	cmd *mockRedisCmds
}

type mockRedisCmds struct {
	rueidis.Builder
}

func newMockRedisClient() *mockRedisClient {
	// rueidis.NewBuilder will return a builder that does not use networking for building commands.
	return &mockRedisClient{
		cmd: &mockRedisCmds{Builder: rueidis.NewBuilder(rueidis.ClientOption{})},
	}
}

func (m *mockRedisClient) B() rueidis.Builder {
	return m.cmd.Builder
}

func (m *mockRedisClient) Do(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
	// We return an empty RedisResult
	return rueidis.RedisResult{}
}

func TestRedisTaskQueue(t *testing.T) {
	mockClient := newMockRedisClient()

	q := NewRedisTaskQueue(mockClient, "test")

	if q.prefix != "test" {
		t.Fatalf("expected prefix test, got %s", q.prefix)
	}

	if q.queueKey() != "test:queued" {
		t.Fatalf("expected test:queued, got %s", q.queueKey())
	}

	ctx := context.Background()

	job := &Job{
		ID:           "test-job-1",
		ParentTaskID: "task-1",
		AgentRole:    "tester",
		Payload:      "{}",
		MaxAttempts:  3,
	}

	// Because Do() returns a nil/empty RedisResult, it will return an error when parsed by rueidis.
	// We accept the error, but verify the job's internal state mutation.
	_ = q.Enqueue(ctx, job)
	if job.Status != "QUEUED" {
		t.Fatalf("Expected job status to be QUEUED after Enqueue, got %s", job.Status)
	}
	if job.RunAfter.IsZero() {
		t.Fatalf("Expected job RunAfter to be set")
	}

	// Ensure these do not panic
	_, err := q.Dequeue(ctx, []string{"tester"})
	if err != nil {
		// Expecting error because rueidis parse fails on empty result, but no panic!
	}

	err = q.Complete(ctx, "test-job-1")
	if err == nil {
		t.Fatalf("Expected error due to mock empty result, got nil")
	}

	err = q.Fail(ctx, "test-job-1", "mock fail")
	if err == nil {
		t.Fatalf("Expected error due to mock empty result, got nil")
	}
}
