package queue

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/redis/rueidis"
	"github.com/onehumancorp/mono/src/server/telemetry"
)

// RedisClient defines the subset of rueidis.Client needed by the queue.
type RedisClient interface {
	Do(ctx context.Context, cmd rueidis.Completed) rueidis.RedisResult
	B() rueidis.Builder
}

type RedisTaskQueue struct {
	client RedisClient
	prefix string
}

// Ensure the real rueidis.Client satisfies our RedisClient interface
var _ RedisClient = (rueidis.Client)(nil)

func NewRedisTaskQueue(client RedisClient, prefix string) *RedisTaskQueue {
	if prefix == "" {
		prefix = "ohc:subagent:jobs"
	}
	return &RedisTaskQueue{client: client, prefix: prefix}
}

func (q *RedisTaskQueue) jobKey(id string) string {
	return fmt.Sprintf("%s:data:%s", q.prefix, id)
}

func (q *RedisTaskQueue) queueKey() string {
	return fmt.Sprintf("%s:queued", q.prefix) // ZSET for run_after sorting
}

func (q *RedisTaskQueue) runningKey() string {
	return fmt.Sprintf("%s:running", q.prefix) // ZSET for locked_until sorting
}

func (q *RedisTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	defer func() {
		telemetry.RecordQueueLength(ctx, 1) // Approximation
	}()

	if job.RunAfter.IsZero() {
		job.RunAfter = time.Now()
	}

	job.Status = "QUEUED"

	jobData, err := json.Marshal(job)
	if err != nil {
		return err
	}

	// Save job data
	setCmd := q.client.B().Set().Key(q.jobKey(job.ID)).Value(string(jobData)).Build()
	if err := q.client.Do(ctx, setCmd).Error(); err != nil {
		return err
	}

	// Add to queue sorted by run_after
	zaddCmd := q.client.B().Zadd().Key(q.queueKey()).ScoreMember().ScoreMember(float64(job.RunAfter.UnixMilli()), job.ID).Build()
	if err := q.client.Do(ctx, zaddCmd).Error(); err != nil {
		return err
	}

	return nil
}

// recoverStaleJobs finds jobs in RUNNING state whose locked_until has expired, and moves them back to QUEUED.
func (q *RedisTaskQueue) recoverStaleJobs(ctx context.Context, now int64) {
	zrangeCmd := q.client.B().Zrange().Key(q.runningKey()).Min("-inf").Max(fmt.Sprintf("%d", now)).Byscore().Limit(0, 10).Build()
	res, err := q.client.Do(ctx, zrangeCmd).AsStrSlice()
	if err != nil || len(res) == 0 {
		return
	}

	for _, jobID := range res {
		zremCmd := q.client.B().Zrem().Key(q.runningKey()).Member(jobID).Build()
		removed, _ := q.client.Do(ctx, zremCmd).AsInt64()
		if removed > 0 {
			// Successfully claimed the stale job, put it back in queued
			zaddCmd := q.client.B().Zadd().Key(q.queueKey()).ScoreMember().ScoreMember(float64(now), jobID).Build()
			q.client.Do(ctx, zaddCmd)
		}
	}
}

func (q *RedisTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	now := time.Now().UnixMilli()

	q.recoverStaleJobs(ctx, now)

	zrangeCmd := q.client.B().Zrange().Key(q.queueKey()).Min("-inf").Max(fmt.Sprintf("%d", now)).Byscore().Limit(0, 10).Build()
	res, err := q.client.Do(ctx, zrangeCmd).AsStrSlice()
	if err != nil {
		return nil, err
	}

	if len(res) == 0 {
		return nil, nil // No jobs ready
	}

	// Attempt to claim one of the ready jobs
	for _, jobID := range res {
		// Read job data
		getCmd := q.client.B().Get().Key(q.jobKey(jobID)).Build()
		jobDataStr, err := q.client.Do(ctx, getCmd).ToString()
		if err != nil {
			continue
		}

		var job Job
		if err := json.Unmarshal([]byte(jobDataStr), &job); err != nil {
			continue
		}

		// Check role
		roleMatches := len(roles) == 0
		for _, role := range roles {
			if job.AgentRole == role {
				roleMatches = true
				break
			}
		}

		if !roleMatches {
			continue
		}

		// Try to claim it by removing from ZSET
		zremCmd := q.client.B().Zrem().Key(q.queueKey()).Member(jobID).Build()
		removed, err := q.client.Do(ctx, zremCmd).AsInt64()
		if err != nil || removed == 0 {
			// Someone else got it
			continue
		}

		// Claimed successfully! Update status.
		job.Status = "RUNNING"
		job.Attempts++
		job.UpdatedAt = time.Now()
		lt := time.Now().Add(5 * time.Minute)
		job.LockedUntil = &lt

		newData, _ := json.Marshal(job)
		setCmd := q.client.B().Set().Key(q.jobKey(job.ID)).Value(string(newData)).Build()
		q.client.Do(ctx, setCmd) // Best effort update data

		// Add to running ZSET to track locked_until
		zaddRunningCmd := q.client.B().Zadd().Key(q.runningKey()).ScoreMember().ScoreMember(float64(lt.UnixMilli()), jobID).Build()
		q.client.Do(ctx, zaddRunningCmd)

		telemetry.RecordQueueLength(ctx, -1) // Job removed from queued state

		return &job, nil
	}

	return nil, nil
}

func (q *RedisTaskQueue) Complete(ctx context.Context, jobID string) error {
	getCmd := q.client.B().Get().Key(q.jobKey(jobID)).Build()
	jobDataStr, err := q.client.Do(ctx, getCmd).ToString()
	if err != nil {
		return err
	}

	var job Job
	if err := json.Unmarshal([]byte(jobDataStr), &job); err != nil {
		return err
	}

	job.Status = "COMPLETED"
	job.UpdatedAt = time.Now()
	job.LockedUntil = nil

	newData, _ := json.Marshal(job)
	setCmd := q.client.B().Set().Key(q.jobKey(job.ID)).Value(string(newData)).Build()
	err = q.client.Do(ctx, setCmd).Error()

	// Remove from running tracking
	zremRunningCmd := q.client.B().Zrem().Key(q.runningKey()).Member(jobID).Build()
	q.client.Do(ctx, zremRunningCmd)

	return err
}

func (q *RedisTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	getCmd := q.client.B().Get().Key(q.jobKey(jobID)).Build()
	jobDataStr, err := q.client.Do(ctx, getCmd).ToString()
	if err != nil {
		return err
	}

	var job Job
	if err := json.Unmarshal([]byte(jobDataStr), &job); err != nil {
		return err
	}

	var payload map[string]interface{}
	json.Unmarshal([]byte(job.Payload), &payload)
	if payload == nil {
		payload = make(map[string]interface{})
	}
	payload["last_error"] = reason
	newPayload, _ := json.Marshal(telemetry.RedactInterfacePII(payload))
	job.Payload = string(newPayload)

	// Remove from running tracking
	zremRunningCmd := q.client.B().Zrem().Key(q.runningKey()).Member(jobID).Build()
	q.client.Do(ctx, zremRunningCmd)

	if job.Attempts >= job.MaxAttempts {
		job.Status = "FAILED"
		job.LockedUntil = nil
		job.UpdatedAt = time.Now()

		newData, _ := json.Marshal(job)
		setCmd := q.client.B().Set().Key(q.jobKey(job.ID)).Value(string(newData)).Build()
		return q.client.Do(ctx, setCmd).Error()
	} else {
		// Requeue with backoff
		job.Status = "QUEUED"
		backoff := time.Duration(1<<job.Attempts) * time.Second
		job.RunAfter = time.Now().Add(backoff)
		job.LockedUntil = nil
		job.UpdatedAt = time.Now()

		newData, _ := json.Marshal(job)
		setCmd := q.client.B().Set().Key(q.jobKey(job.ID)).Value(string(newData)).Build()
		if err := q.client.Do(ctx, setCmd).Error(); err != nil {
			return err
		}

		zaddCmd := q.client.B().Zadd().Key(q.queueKey()).ScoreMember().ScoreMember(float64(job.RunAfter.UnixMilli()), job.ID).Build()
		err = q.client.Do(ctx, zaddCmd).Error()
		if err == nil {
			telemetry.RecordQueueLength(ctx, 1) // Job returned to queue
		}
		return err
	}
}
// added for Sub-Agent Orchestration Queue
