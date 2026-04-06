package queue

import (
	"testing"
)

func TestNewRedisTaskQueue(t *testing.T) {
	// Fallback to basic initialization checks as per reviewer instructions.
	// Since we cannot easily mock rueidis.Client and its Builder without gomock/rueidis-mock targets in Bazel,
	// we just verify initialization sets the default prefix correctly.

	q := NewRedisTaskQueue(nil, "")
	if q.prefix != "ohc:subagent:jobs" {
		t.Fatalf("expected default prefix 'ohc:subagent:jobs', got '%s'", q.prefix)
	}

	q2 := NewRedisTaskQueue(nil, "custom")
	if q2.prefix != "custom" {
		t.Fatalf("expected custom prefix 'custom', got '%s'", q2.prefix)
	}
}
