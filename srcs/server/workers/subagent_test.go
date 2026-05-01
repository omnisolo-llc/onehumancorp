package workers

import (
	"context"
	"testing"

	"ohc/server/queue"
)

type MockContextKey struct{}
var ClaimsContextKeyForTest = MockContextKey{}

type errorQueue struct{}

func (q *errorQueue) EnqueueSubAgent(ctx context.Context, taskID string, role string, payload []byte) error {
	return nil
}

func (q *errorQueue) ProcessSubAgentJob(ctx context.Context, job *queue.Job) error {
	return context.Canceled // simulate an error
}


func TestSubAgentWorker(t *testing.T) {
	q, err := queue.NewSQLiteQueue(":memory:")
	if err != nil {
		t.Fatalf("failed to create queue: %v", err)
	}

	w := NewSubAgentWorker(q)
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "test_claims")

	job := &queue.Job{
		ID:      "1",
		TaskID:  "task1",
		Role:    "role1",
		Payload: []byte("payload"),
	}

	err = w.HandleJob(ctx, job)
	if err != nil {
		t.Fatalf("failed to handle job: %v", err)
	}
}

func TestSubAgentWorkerErrorHandling(t *testing.T) {
	q := &errorQueue{}
	w := NewSubAgentWorker(q)
	ctx := context.WithValue(context.Background(), ClaimsContextKeyForTest, "test_claims")

	job := &queue.Job{
		ID:      "1",
		TaskID:  "task1",
		Role:    "role1",
		Payload: []byte("payload"),
	}

	err := w.HandleJob(ctx, job)
	if err == nil {
		t.Fatalf("expected error handling job")
	}
}
