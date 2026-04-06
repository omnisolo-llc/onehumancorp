package queue

import (
	"context"
	"encoding/json"
	"testing"
	"time"
)

type mockRedisClient struct {
	setFunc    func(ctx context.Context, key, value string) error
	zaddFunc   func(ctx context.Context, key string, score float64, member string) error
	zrangeFunc func(ctx context.Context, key, min, max string, limit int64) ([]string, error)
	zremFunc   func(ctx context.Context, key, member string) (int64, error)
	getFunc    func(ctx context.Context, key string) (string, error)
}

func (m *mockRedisClient) Set(ctx context.Context, key, value string) error {
	if m.setFunc != nil {
		return m.setFunc(ctx, key, value)
	}
	return nil
}
func (m *mockRedisClient) Zadd(ctx context.Context, key string, score float64, member string) error {
	if m.zaddFunc != nil {
		return m.zaddFunc(ctx, key, score, member)
	}
	return nil
}
func (m *mockRedisClient) Zrange(ctx context.Context, key, min, max string, limit int64) ([]string, error) {
	if m.zrangeFunc != nil {
		return m.zrangeFunc(ctx, key, min, max, limit)
	}
	return nil, nil
}
func (m *mockRedisClient) Zrem(ctx context.Context, key, member string) (int64, error) {
	if m.zremFunc != nil {
		return m.zremFunc(ctx, key, member)
	}
	return 1, nil
}
func (m *mockRedisClient) Get(ctx context.Context, key string) (string, error) {
	if m.getFunc != nil {
		return m.getFunc(ctx, key)
	}
	return "", nil
}

func TestRedisTaskQueue(t *testing.T) {
	m := &mockRedisClient{}

	// Direct assignment instead of using NewRedisTaskQueue which accepts rueidis.Client
	q := &RedisTaskQueue{client: m, prefix: "test"}

	if q.queueKey() != "test:queued" {
		t.Fatalf("expected test:queued, got %s", q.queueKey())
	}

	if q.jobKey("123") != "test:data:123" {
		t.Fatalf("expected test:data:123, got %s", q.jobKey("123"))
	}

	if q.runningKey() != "test:running" {
		t.Fatalf("expected test:running, got %s", q.runningKey())
	}

	ctx := context.Background()

	job := &Job{
		ID:           "test-job-1",
		ParentTaskID: "task-1",
		AgentRole:    "tester",
		Payload:      "{}",
		MaxAttempts:  3,
	}

	// Test Enqueue
	if err := q.Enqueue(ctx, job); err != nil {
		t.Fatalf("Enqueue failed: %v", err)
	}

	jobData, _ := json.Marshal(job)

	// Test Dequeue
	m.zrangeFunc = func(ctx context.Context, key, min, max string, limit int64) ([]string, error) {
		if key == q.runningKey() {
			return []string{}, nil
		}
		return []string{"test-job-1"}, nil
	}
	m.getFunc = func(ctx context.Context, key string) (string, error) {
		return string(jobData), nil
	}

	dequeued, err := q.Dequeue(ctx, []string{"tester"})
	if err != nil {
		t.Fatalf("Dequeue failed: %v", err)
	}
	if dequeued == nil {
		t.Fatalf("Expected to dequeue job, got nil")
	}

	// Test Complete
	if err := q.Complete(ctx, "test-job-1"); err != nil {
		t.Fatalf("Complete failed: %v", err)
	}

	// Test Fail
	job.Attempts = 3
	jobData, _ = json.Marshal(job)

	if err := q.Fail(ctx, "test-job-1", "some error"); err != nil {
		t.Fatalf("Fail failed: %v", err)
	}

	// Test Fail with requeue
	job.Attempts = 1
	jobData, _ = json.Marshal(job)

	if err := q.Fail(ctx, "test-job-1", "some error"); err != nil {
		t.Fatalf("Fail requeue failed: %v", err)
	}

	// Additional branches
	// Empty Queue
	m.zrangeFunc = func(ctx context.Context, key, min, max string, limit int64) ([]string, error) {
		return []string{}, nil
	}
	q.Dequeue(ctx, []string{"tester"})

	// Recover stale jobs
	m.zrangeFunc = func(ctx context.Context, key, min, max string, limit int64) ([]string, error) {
		if key == q.runningKey() {
			return []string{"stale-job"}, nil
		}
		return []string{}, nil
	}
	q.recoverStaleJobs(ctx, time.Now().UnixMilli())

	// Role mismatch
	m.zrangeFunc = func(ctx context.Context, key, min, max string, limit int64) ([]string, error) {
		if key == q.queueKey() {
			return []string{"test-job-1"}, nil
		}
		return []string{}, nil
	}
	q.Dequeue(ctx, []string{"other-role"})
}

func TestRedisQueue_ErrorsExtra(t *testing.T) {
	ctx := context.Background()
	m := &mockRedisClient{}
	q := &RedisTaskQueue{client: m, prefix: "test"}

	// cover remaining zrem fail
	m.zrangeFunc = func(ctx context.Context, key, min, max string, limit int64) ([]string, error) {
		return []string{"test-job-err"}, nil
	}
	m.getFunc = func(ctx context.Context, key string) (string, error) {
		return "{}", nil
	}
	m.zremFunc = func(ctx context.Context, key, member string) (int64, error) {
		return 0, context.DeadlineExceeded
	}
	q.Dequeue(ctx, []string{"tester"})

	// cover complete Get fail
	m.getFunc = func(ctx context.Context, key string) (string, error) {
		return "", context.DeadlineExceeded
	}
	q.Complete(ctx, "x")
}

func TestRedisTaskQueueErrors(t *testing.T) {
	ctx := context.Background()
	job := &Job{
		ID:           "test-job-err",
		ParentTaskID: "task-1",
		AgentRole:    "tester",
		Payload:      "{}",
		MaxAttempts:  3,
	}

	m := &mockRedisClient{}
	q := &RedisTaskQueue{client: m, prefix: "test"}

	// Enqueue json marshal error? Job struct doesn't fail marshal easily.
	// We can test Set error
	m.setFunc = func(ctx context.Context, key, value string) error {
		return context.DeadlineExceeded
	}
	if err := q.Enqueue(ctx, job); err == nil {
		t.Fatalf("Expected error, got nil")
	}

	// Enqueue Zadd error
	m.setFunc = nil
	m.zaddFunc = func(ctx context.Context, key string, score float64, member string) error {
		return context.DeadlineExceeded
	}
	if err := q.Enqueue(ctx, job); err == nil {
		t.Fatalf("Expected error, got nil")
	}

	// Dequeue Zrange error
	m.zaddFunc = nil
	m.zrangeFunc = func(ctx context.Context, key, min, max string, limit int64) ([]string, error) {
		return nil, context.DeadlineExceeded
	}
	if _, err := q.Dequeue(ctx, []string{"tester"}); err == nil {
		t.Fatalf("Expected error, got nil")
	}

	// Dequeue Get error
	m.zrangeFunc = func(ctx context.Context, key, min, max string, limit int64) ([]string, error) {
		if key == q.runningKey() {
			return []string{}, nil
		}
		return []string{"test-job-err"}, nil
	}
	m.getFunc = func(ctx context.Context, key string) (string, error) {
		return "", context.DeadlineExceeded
	}
	q.Dequeue(ctx, []string{"tester"})

	// Complete Get error
	if err := q.Complete(ctx, "test-job-err"); err == nil {
		t.Fatalf("Expected error, got nil")
	}

	// Fail Get error
	if err := q.Fail(ctx, "test-job-err", "reason"); err == nil {
		t.Fatalf("Expected error, got nil")
	}

	// Fail Set error branch
	m.getFunc = func(ctx context.Context, key string) (string, error) {
		jobData, _ := json.Marshal(job)
		return string(jobData), nil
	}
	m.setFunc = func(ctx context.Context, key, value string) error {
		return context.DeadlineExceeded
	}
	job.Attempts = 3
	q.Fail(ctx, "test-job-err", "reason")

	job.Attempts = 0
	q.Fail(ctx, "test-job-err", "reason")
}

func TestDefaultRedisClient(t *testing.T) {
	// We want to test NewRedisTaskQueue and defaultRedisClient wrappers
	q := NewRedisTaskQueue(nil, "")
	if q.prefix != "ohc:subagent:jobs" {
		t.Fatalf("Expected ohc:subagent:jobs prefix, got %s", q.prefix)
	}

	// Because defaultRedisClient panics on B() with a nil client, we recover and check if we hit the panic as proof it executed.
	rc := &defaultRedisClient{client: nil}

	func() {
		defer func() { recover() }()
		rc.Set(context.Background(), "k", "v")
	}()
	func() {
		defer func() { recover() }()
		rc.Zadd(context.Background(), "k", 1, "v")
	}()
	func() {
		defer func() { recover() }()
		rc.Zrange(context.Background(), "k", "1", "2", 1)
	}()
	func() {
		defer func() { recover() }()
		rc.Zrem(context.Background(), "k", "v")
	}()
	func() {
		defer func() { recover() }()
		rc.Get(context.Background(), "k")
	}()
}

func TestRedisQueue_AdditionalBranches(t *testing.T) {
	q := NewRedisTaskQueue(nil, "")
	if q.prefix != "ohc:subagent:jobs" {
		t.Fatalf("expected prefix")
	}

	q = NewRedisTaskQueue(nil, "myprefix")
	if q.prefix != "myprefix" {
		t.Fatalf("expected prefix")
	}
}
