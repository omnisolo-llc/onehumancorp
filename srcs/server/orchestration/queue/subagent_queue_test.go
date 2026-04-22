package queue

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestSQLiteSubAgentTaskQueue(t *testing.T) {
	provider := db.NewTestProvider(t)
	q := NewSQLiteSubAgentTaskQueue(provider, QueueOptions{})
	if q == nil {
		t.Fatalf("q is nil")
	}
}

type mockQ struct{}
func (m *mockQ) Enqueue(ctx context.Context, payload *SubAgentTaskQueuePayload) error { return nil }
func (m *mockQ) Process(ctx context.Context, queueName string) (*SubAgentTaskQueuePayload, error) { return nil, nil }
func (m *mockQ) Complete(ctx context.Context, jobID string, queueName string) error { return nil }
func (m *mockQ) Fail(ctx context.Context, jobID string, queueName string, reason string) error { return nil }
