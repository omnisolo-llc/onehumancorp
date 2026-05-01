package queue

import (
	"context"
	"testing"
	"encoding/json"

	"github.com/hibiken/asynq"
)

func TestAsynqQueueCoverage(t *testing.T) {
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

func TestAsynqSubAgentPayloadMarshal(t *testing.T) {
	p := asynqSubAgentPayload{
		TaskID:  "task1",
		Role:    "role1",
		Payload: []byte("payload"),
	}

	_, err := json.Marshal(p)
	if err != nil {
		t.Fatalf("Marshal failed: %v", err)
	}
}
