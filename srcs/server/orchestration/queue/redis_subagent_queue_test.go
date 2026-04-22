package queue

import (
	"testing"
	"github.com/redis/rueidis"
)

func TestRedisSubAgentTaskQueue(t *testing.T) {
	// A simple compilation check
	opts := QueueOptions{MaxRetries: 1, RateLimitRate: 10, DLQName: "dlq"}
	var client rueidis.Client
	q := NewRedisSubAgentTaskQueue(client, "test", opts)
	if q == nil {
		t.Fatalf("q is nil")
	}
}
