package queue

import (
	"context"
	"testing"
	"time"

	"github.com/redis/rueidis"
)

type CustomMockBuilder struct {
	rueidis.Builder
}

type SafeMockClient struct {
	rueidis.Client
	cmds []rueidis.Completed
}

func (s *SafeMockClient) Do(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
	s.cmds = append(s.cmds, cmd)
	return rueidis.RedisResult{}
}

func TestRedisTaskQueue(t *testing.T) {
	q := NewRedisTaskQueue(nil, "test")

	if q.prefix != "test" {
		t.Fatalf("expected prefix test, got %s", q.prefix)
	}

	if q.queueKey() != "test:queued" {
		t.Fatalf("expected test:queued, got %s", q.queueKey())
	}
	if q.runningKey() != "test:running" {
		t.Fatalf("expected test:running, got %s", q.runningKey())
	}
}
