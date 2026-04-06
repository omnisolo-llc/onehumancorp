package queue

import (
	"context"
	"encoding/json"
	"fmt"
	"strconv"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

// RedisTaskQueue implements TaskQueue using Redis via rueidis.
type RedisTaskQueue struct {
	client rueidis.Client
	prefix string
}

// NewRedisTaskQueue creates a new RedisTaskQueue.
func NewRedisTaskQueue(client rueidis.Client, prefix string) *RedisTaskQueue {
	if prefix == "" {
		prefix = "ohc:queue:"
	}
	return &RedisTaskQueue{client: client, prefix: prefix}
}

// jobKey returns the Redis key for a specific job payload
func (q *RedisTaskQueue) jobKey(jobID string) string {
	return q.prefix + "job:" + jobID
}

// queueKey returns the Redis key for a specific role's queue (using Sorted Sets for scheduling)
func (q *RedisTaskQueue) queueKey(role string) string {
	return q.prefix + "pending:" + role
}

// activeKey returns the Redis key for active (running) jobs
func (q *RedisTaskQueue) activeKey() string {
	return q.prefix + "active"
}

// Enqueue adds a job to Redis.
func (q *RedisTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	if job.RunAfter.IsZero() {
		job.RunAfter = time.Now()
	}

	jobBytes, err := json.Marshal(job)
	if err != nil {
		return fmt.Errorf("failed to marshal job: %w", err)
	}

	cmds := make(rueidis.Commands, 0, 2)

	// 1. Store the job payload
	cmds = append(cmds, q.client.B().Set().Key(q.jobKey(job.ID)).Value(string(jobBytes)).Build())

	// 2. Add to sorted set for scheduling
	cmds = append(cmds, q.client.B().Zadd().Key(q.queueKey(job.AgentRole)).ScoreMember().ScoreMember(
		float64(job.RunAfter.UnixMilli()), job.ID,
	).Build())

	for _, res := range q.client.DoMulti(ctx, cmds...) {
		if err := res.Error(); err != nil {
			return fmt.Errorf("redis enqueue failed: %w", err)
		}
	}

	telemetry.RecordTaskQueueLength(ctx, 1)
	return nil
}

// Dequeue attempts to fetch a job for the specified roles.
func (q *RedisTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	if len(roles) == 0 {
		return nil, nil // We require roles to know which queues to poll
	}

	now := float64(time.Now().UnixMilli())

	// Simple polling across roles. A more robust implementation might use ZPOPMIN with Lua scripts
	// but for simplicity we iterate roles.
	for _, role := range roles {
		// Use a Lua script for atomic fetch-and-move-to-active
		script := `
			local queue_key = KEYS[1]
			local active_key = KEYS[2]
			local now = tonumber(ARGV[1])
			local lock_until = tonumber(ARGV[2])

			local items = redis.call('ZRANGEBYSCORE', queue_key, '-inf', now, 'LIMIT', 0, 1)
			if #items == 0 then
				return nil
			end

			local job_id = items[1]
			redis.call('ZREM', queue_key, job_id)
			redis.call('ZADD', active_key, lock_until, job_id)
			return job_id
		`

		lockUntil := float64(time.Now().Add(5 * time.Minute).UnixMilli())

		cmd := q.client.B().Eval().Script(script).Numkeys(2).Key(q.queueKey(role), q.activeKey()).Arg(
			strconv.FormatFloat(now, 'f', -1, 64),
			strconv.FormatFloat(lockUntil, 'f', -1, 64),
		).Build()

		res := q.client.Do(ctx, cmd)
		if res.Error() != nil {
			if rueidis.IsRedisNil(res.Error()) {
				continue // Try next role
			}
			return nil, fmt.Errorf("redis script failed: %w", res.Error())
		}

		jobID, err := res.ToString()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				continue // Try next role
			}
			return nil, fmt.Errorf("failed to parse job ID: %w", err)
		}

		if jobID == "" {
			continue // No job found, try next role
		}

		// We claimed a job ID, fetch its payload
		payloadRes := q.client.Do(ctx, q.client.B().Get().Key(q.jobKey(jobID)).Build())
		if payloadRes.Error() != nil {
			return nil, fmt.Errorf("failed to fetch job payload: %w", payloadRes.Error())
		}

		payloadStr, err := payloadRes.ToString()
		if err != nil {
			return nil, fmt.Errorf("failed to stringify payload: %w", err)
		}

		var job Job
		if err := json.Unmarshal([]byte(payloadStr), &job); err != nil {
			return nil, fmt.Errorf("failed to unmarshal job: %w", err)
		}

		job.Status = "RUNNING"
		lockTime := time.UnixMilli(int64(lockUntil))
		job.LockedUntil = &lockTime

		// Update job state in Redis
		updatedBytes, _ := json.Marshal(job)
		q.client.Do(ctx, q.client.B().Set().Key(q.jobKey(job.ID)).Value(string(updatedBytes)).Build())

		telemetry.RecordTaskQueueLength(ctx, -1)
		return &job, nil
	}

	return nil, nil // No jobs found across specified roles
}

// Complete marks a job as successfully completed.
func (q *RedisTaskQueue) Complete(ctx context.Context, jobID string) error {
	cmds := make(rueidis.Commands, 0, 2)
	cmds = append(cmds, q.client.B().Zrem().Key(q.activeKey()).Member(jobID).Build())
	cmds = append(cmds, q.client.B().Del().Key(q.jobKey(jobID)).Build()) // Cleanup completed jobs

	for _, res := range q.client.DoMulti(ctx, cmds...) {
		if err := res.Error(); err != nil {
			return fmt.Errorf("failed to complete job in redis: %w", err)
		}
	}
	return nil
}

// Fail marks a job as failed, potentially requeuing it.
func (q *RedisTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	// 1. Fetch job
	payloadRes := q.client.Do(ctx, q.client.B().Get().Key(q.jobKey(jobID)).Build())
	if payloadRes.Error() != nil {
		return fmt.Errorf("failed to fetch job for failing: %w", payloadRes.Error())
	}

	payloadStr, err := payloadRes.ToString()
	if err != nil {
		return err
	}

	var job Job
	if err := json.Unmarshal([]byte(payloadStr), &job); err != nil {
		return err
	}

	job.Attempts++

	// Remove from active queue
	remCmd := q.client.B().Zrem().Key(q.activeKey()).Member(jobID).Build()
	if err := q.client.Do(ctx, remCmd).Error(); err != nil {
		return fmt.Errorf("failed to remove from active: %w", err)
	}

	if job.Attempts >= job.MaxAttempts {
		// Permanently fail, move to a failed queue for dead lettering
		job.Status = "FAILED"
		job.LockedUntil = nil
		updatedBytes, _ := json.Marshal(job)

		cmds := make(rueidis.Commands, 0, 2)
		cmds = append(cmds, q.client.B().Set().Key(q.jobKey(jobID)).Value(string(updatedBytes)).Build())
		cmds = append(cmds, q.client.B().Sadd().Key(q.prefix+"failed").Member(jobID).Build())

		for _, res := range q.client.DoMulti(ctx, cmds...) {
			if err := res.Error(); err != nil {
				return err
			}
		}
	} else {
		// Requeue with backoff
		backoff := time.Duration(job.Attempts) * time.Minute
		job.RunAfter = time.Now().Add(backoff)
		job.Status = "QUEUED"
		job.LockedUntil = nil
		updatedBytes, _ := json.Marshal(job)

		cmds := make(rueidis.Commands, 0, 2)
		cmds = append(cmds, q.client.B().Set().Key(q.jobKey(jobID)).Value(string(updatedBytes)).Build())
		cmds = append(cmds, q.client.B().Zadd().Key(q.queueKey(job.AgentRole)).ScoreMember().ScoreMember(
			float64(job.RunAfter.UnixMilli()), job.ID,
		).Build())

		for _, res := range q.client.DoMulti(ctx, cmds...) {
			if err := res.Error(); err != nil {
				return err
			}
		}
		telemetry.RecordTaskQueueLength(ctx, 1)
	}

	return nil
}
