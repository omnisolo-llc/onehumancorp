package queue

import (
	"context"
	"testing"
	"github.com/hibiken/asynq"
)

type MockContextKey struct{}
var ClaimsContextKeyForTest = MockContextKey{}

func TestSQLiteQueue(t *testing.T) {
	q, err := NewSQLiteQueue(":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite queue: %v", err)
	}
    defer q.Close()

	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "test_claims")

	err = q.EnqueueSubAgent(ctx, "task1", "role1", []byte("payload"))
	if err != nil {
		t.Fatalf("failed to enqueue: %v", err)
	}

	job := &Job{
		ID:      "1",
		TaskID:  "task1",
		Role:    "role1",
		Payload: []byte("payload"),
	}

	err = q.ProcessSubAgentJob(ctx, job)
	if err != nil {
		t.Fatalf("failed to process job: %v", err)
	}
}

func TestSQLiteQueueErrorHandling(t *testing.T) {
	_, err := NewSQLiteQueue("file:bad.db?mode=ro")
	if err == nil {
	    t.Fatalf("expected error for read-only db operation")
	}
}

func TestSQLiteQueueExecErrorHandling(t *testing.T) {
	q, err := NewSQLiteQueue(":memory:")
	if err != nil {
		t.Fatalf("failed to create sqlite queue: %v", err)
	}
    q.Close() // this will force errors on subsequent commands

	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "test_claims")

	err = q.EnqueueSubAgent(ctx, "task1", "role1", []byte("payload"))
	if err == nil {
		t.Fatalf("expected failure to enqueue: %v", err)
	}

	job := &Job{
		ID:      "1",
		TaskID:  "task1",
		Role:    "role1",
		Payload: []byte("payload"),
	}

	err = q.ProcessSubAgentJob(ctx, job)
	if err == nil {
		t.Fatalf("expected failure to process job: %v", err)
	}
}

func TestAsynqQueue(t *testing.T) {
	opt := asynq.RedisClientOpt{Addr: "localhost:6379"}
	q := NewAsynqQueue(opt)

	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "test_claims")

	err := q.EnqueueSubAgent(ctx, "task1", "role1", []byte("payload"))
	if err != nil {
		t.Logf("Expected failure without redis: %v", err)
	}

	job := &Job{
		ID:      "1",
		TaskID:  "task1",
		Role:    "role1",
		Payload: []byte("payload"),
	}

	err = q.ProcessSubAgentJob(ctx, job)
	if err != nil {
		t.Fatalf("Expected nil from ProcessSubAgentJob: %v", err)
	}
}
