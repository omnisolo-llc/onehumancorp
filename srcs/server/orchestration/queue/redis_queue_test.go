package queue

import (
	"context"
	"testing"

	"github.com/redis/rueidis"
)

type dummyResult struct {
	err error
}
func (r dummyResult) Error() error { return r.err }
func (r dummyResult) Val() rueidis.RedisMessage { return rueidis.RedisMessage{} }
func (r dummyResult) ToArray() ([]rueidis.RedisMessage, error) { return nil, r.err }
func (r dummyResult) AsStrSlice() ([]string, error) { return []string{}, r.err }
func (r dummyResult) AsInt64() (int64, error) { return 0, r.err }
func (r dummyResult) ToString() (string, error) { return "", r.err }
func (r dummyResult) AsBool() (bool, error) { return false, r.err }
func (r dummyResult) AsFloat64() (float64, error) { return 0, r.err }
func (r dummyResult) AsInt() (int, error) { return 0, r.err }
func (r dummyResult) AsBytes() ([]byte, error) { return nil, r.err }
func (r dummyResult) AsStrMap() (map[string]string, error) { return nil, r.err }
func (r dummyResult) IsCacheHit() bool { return false }
func (r dummyResult) NonRedisError() error { return r.err }
func (r dummyResult) ToAny() (any, error) { return nil, r.err }
func (r dummyResult) ToMessage() (rueidis.RedisMessage, error) { return rueidis.RedisMessage{}, r.err }
func (r dummyResult) AsZScores() ([]rueidis.ZScore, error) { return nil, r.err }
func (r dummyResult) IsMuxed() bool { return false }
func (r dummyResult) AsReader() (rueidis.RedisResultReader, error) { return nil, r.err }

// Implement RedisResult by ensuring it conforms.
var _ rueidis.RedisResult = dummyResult{}

type dummyClient struct {
	rueidis.Client
	err error
}

func (c *dummyClient) Do(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult {
	return dummyResult{err: c.err}
}

func TestRedisTaskQueue(t *testing.T) {
	q := NewRedisTaskQueue(&dummyClient{}, "test")

	if q.prefix != "test" {
		t.Fatalf("expected prefix test, got %s", q.prefix)
	}

	if q.queueKey() != "test:queued" {
		t.Fatalf("expected test:queued, got %s", q.queueKey())
	}

	jobID := "123"
	if q.jobKey(jobID) != "test:data:123" {
		t.Fatalf("expected test:data:123, got %s", q.jobKey(jobID))
	}
	if q.runningKey() != "test:running" {
		t.Fatalf("expected test:running, got %s", q.runningKey())
	}
}
