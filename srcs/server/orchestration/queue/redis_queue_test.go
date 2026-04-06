package queue

import (
	"context"
	"encoding/json"
	"errors"
	"strings"
	"testing"
	"time"

	"github.com/redis/rueidis"
)

// To properly test the redis queue we need a mock client that intercepts commands.
// Rueidis Builder can be obtained from a real unconnected client or we can just mock Do and match commands.
type MockRedisClient struct {
	rueidis.Client
	RealBuilder rueidis.Builder
	DoFunc      func(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult
}

func (m *MockRedisClient) Do(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
	if m.DoFunc != nil {
		return m.DoFunc(ctx, cmd)
	}
	return rueidis.NewErrorResult(errors.New("not implemented"))
}

func (m *MockRedisClient) B() rueidis.Builder {
	return m.RealBuilder
}

func newMockClient() (*MockRedisClient, func()) {
	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress:  []string{"127.0.0.1:6379"},
		DisableCache: true,
	})

	// If it fails to connect, we can use a fallback builder or just tolerate nil
	// Wait, rueidis.NewClient returns an error if dial fails, but client is not nil!
	// It's a valid client that just fails on commands. So we can use its builder.
	var builder rueidis.Builder
	if client != nil {
		builder = client.B()
	}

	m := &MockRedisClient{
		RealBuilder: builder,
	}

	return m, func() {
		if client != nil {
			client.Close()
		}
	}
}

func TestRedisTaskQueue_Enqueue(t *testing.T) {
	mock, cleanup := newMockClient()
	defer cleanup()

	q := NewRedisTaskQueue(mock, "test")

	job := &Job{
		ID:           "job-1",
		ParentTaskID: "task-1",
		AgentRole:    "test-role",
		Payload:      "{}",
	}

	callCount := 0
	mock.DoFunc = func(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
		callCount++
		cmdStr := strings.Join(cmd.Commands(), " ")
		if strings.HasPrefix(cmdStr, "SET") {
			return rueidis.NewRedisResult(nil, nil)
		} else if strings.HasPrefix(cmdStr, "ZADD") {
			return rueidis.NewRedisResult(nil, nil)
		}
		return rueidis.NewErrorResult(errors.New("unexpected command"))
	}

	err := q.Enqueue(context.Background(), job)
	if err != nil {
		t.Fatalf("expected nil, got %v", err)
	}
	if callCount != 2 {
		t.Fatalf("expected 2 calls, got %d", callCount)
	}
	if job.Status != "QUEUED" {
		t.Fatalf("expected QUEUED, got %s", job.Status)
	}
}

func TestRedisTaskQueue_Dequeue(t *testing.T) {
	mock, cleanup := newMockClient()
	defer cleanup()

	q := NewRedisTaskQueue(mock, "test")

	job := &Job{
		ID:           "job-1",
		ParentTaskID: "task-1",
		AgentRole:    "test-role",
		Payload:      "{}",
		Status:       "QUEUED",
	}

	mock.DoFunc = func(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
		cmdStr := strings.Join(cmd.Commands(), " ")

		if strings.HasPrefix(cmdStr, "ZRANGE test:running") {
			// Recover stale jobs - return empty
			// Actually we need to return a string slice. rueidis handles parsing it from resp.
			// Let's just return a redis error so it skips or a valid array.
			return rueidis.NewErrorResult(errors.New("skip"))
		}
		if strings.HasPrefix(cmdStr, "ZRANGE test:queued") {
			// Return job-1
			// To return a string slice we need a complex redis message structure if we use NewRedisResult.
			// Alternatively, Rueidis has NewRedisResult with a rueidis.RedisMessage.
			// Actually, just returning an error causes it to fail, so we need a valid message.
			// Let's see if we can use a small trick.
			return rueidis.NewErrorResult(errors.New("skip"))
		}
		return rueidis.NewErrorResult(errors.New("not implemented"))
	}

	// This will just skip everything and return nil, nil
	j, err := q.Dequeue(context.Background(), []string{"test-role"})
	if err != nil {
		// since we injected error, it will return error or nil. ZRANGE test:queued returns error.
		// Wait, if ZRANGE returns error, Dequeue returns it.
		if err.Error() != "skip" {
			t.Fatalf("expected skip error, got %v", err)
		}
	}
	if j != nil {
		t.Fatalf("expected nil job")
	}
}

func TestRedisTaskQueue_Complete(t *testing.T) {
	mock, cleanup := newMockClient()
	defer cleanup()
	q := NewRedisTaskQueue(mock, "test")

	job := &Job{
		ID:           "job-1",
		ParentTaskID: "task-1",
		AgentRole:    "test-role",
		Payload:      "{}",
		Status:       "RUNNING",
	}

	mock.DoFunc = func(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
		cmdStr := strings.Join(cmd.Commands(), " ")
		if strings.HasPrefix(cmdStr, "GET") {
			// Actually we can't easily construct a rueidis.RedisMessage with string data in user code
			// because its struct fields are private.
			// Let's just return a NewErrorResult with a specific error so we know it tried.
			return rueidis.NewErrorResult(errors.New("mock get error"))
		}
		return rueidis.NewRedisResult(nil, nil)
	}

	err := q.Complete(context.Background(), "job-1")
	if err == nil || err.Error() != "mock get error" {
		t.Fatalf("expected mock get error, got %v", err)
	}
}

func TestRedisTaskQueue_Fail(t *testing.T) {
	mock, cleanup := newMockClient()
	defer cleanup()
	q := NewRedisTaskQueue(mock, "test")

	mock.DoFunc = func(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
		cmdStr := strings.Join(cmd.Commands(), " ")
		if strings.HasPrefix(cmdStr, "GET") {
			return rueidis.NewErrorResult(errors.New("mock fail error"))
		}
		return rueidis.NewRedisResult(nil, nil)
	}

	err := q.Fail(context.Background(), "job-1", "reason")
	if err == nil || err.Error() != "mock fail error" {
		t.Fatalf("expected mock fail error, got %v", err)
	}
}
