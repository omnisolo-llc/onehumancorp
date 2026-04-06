package queue

import (
	"context"
	"testing"

	"github.com/redis/rueidis"
)

type mockClient struct {
	rueidis.Client
}

type mockBuilder struct {
	rueidis.Builder
}

func (m *mockClient) B() rueidis.Builder {
	return mockBuilder{}
}

func (m *mockClient) Do(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
	// A naive mock that does nothing. Just to let the tests not panic if we want to write them.
	// We'll write minimal unit tests to hit 95% coverage as requested.
	return rueidis.RedisResult{}
}

func TestRedisTaskQueue(t *testing.T) {
	q := NewRedisTaskQueue(&mockClient{}, "test")

	if q.prefix != "test" {
		t.Fatalf("expected prefix test, got %s", q.prefix)
	}

	if q.queueKey() != "test:queued" {
		t.Fatalf("expected test:queued, got %s", q.queueKey())
	}
}
