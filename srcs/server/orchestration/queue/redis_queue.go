package queue

import (
	"context"
	"encoding/json"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/telemetry"
	"github.com/redis/rueidis"
)

type RedisTaskQueue struct {
	client rueidis.Client
}

func NewRedisTaskQueue(client rueidis.Client) *RedisTaskQueue {
	return &RedisTaskQueue{client: client}
}

func (q *RedisTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	jobBytes, err := json.Marshal(job)
	if err != nil {
		return fmt.Errorf("failed to marshal job: %w", err)
	}

	queueName := "sub_agent_queue:" + job.AgentRole

	cmd := q.client.B().Rpush().Key(queueName).Element(string(jobBytes)).Build()
	err = q.client.Do(ctx, cmd).Error()
	if err != nil {
		return fmt.Errorf("failed to enqueue to redis: %w", err)
	}

	telemetry.RecordTaskQueueLength(ctx, 1)
	return nil
}

func (q *RedisTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	// For simplicity, we pop from the first role queue that has items
	if len(roles) == 0 {
		return nil, fmt.Errorf("must specify at least one role to dequeue")
	}

	var jobStr string
	var queueName string

	for _, role := range roles {
		queueName = "sub_agent_queue:" + role

		// Move delayed jobs to ready queue first
		q.processDelayed(ctx, role)

		lpopCmd := q.client.B().Lpop().Key(queueName).Build()
		resp := q.client.Do(ctx, lpopCmd)
		if err := resp.Error(); err != nil {
			if rueidis.IsRedisNil(err) {
				continue // Try next role
			}
			return nil, fmt.Errorf("redis lpop error: %w", err)
		}

		jobStr, _ = resp.ToString()
		if jobStr != "" {
			break // Found a job
		}
	}

	if jobStr == "" {
		return nil, nil // No jobs found in any role queue
	}

	var job Job
	if err := json.Unmarshal([]byte(jobStr), &job); err != nil {
		return nil, fmt.Errorf("failed to unmarshal job: %w", err)
	}

	// We increment attempts here as it's running now
	job.Attempts++

	telemetry.RecordTaskQueueLength(ctx, -1)
	return &job, nil
}

func (q *RedisTaskQueue) processDelayed(ctx context.Context, role string) {
	delayedKey := "sub_agent_queue:" + role + ":delayed"
	readyKey := "sub_agent_queue:" + role
	now := float64(time.Now().UnixMilli())

	cmd := q.client.B().Zrangebyscore().Key(delayedKey).Min("0").Max(fmt.Sprintf("%f", now)).Build()
	resp := q.client.Do(ctx, cmd)
	if err := resp.Error(); err != nil && !rueidis.IsRedisNil(err) {
		return
	}

	items, err := resp.AsStrSlice()
	if err != nil || len(items) == 0 {
		return
	}

	for _, item := range items {
		// Atomically remove from delayed and add to queue
		zremCmd := q.client.B().Zrem().Key(delayedKey).Member(item).Build()
		if removed, _ := q.client.Do(ctx, zremCmd).AsInt64(); removed > 0 {
			rpushCmd := q.client.B().Rpush().Key(readyKey).Element(item).Build()
			q.client.Do(ctx, rpushCmd)
		}
	}
}


func (q *RedisTaskQueue) Complete(ctx context.Context, jobID string) error {
	// LPOP already removed the job, so completion is essentially a no-op
	// unless we implement a more complex tracking system with active job lists.
	return nil
}

func (q *RedisTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	// For redis, we need the full job to fail it back to the queue.
	// Since the interface only takes jobID, a proper implementation would need
	// an active jobs hash to fetch the job.
	// For this exercise, we will just log and return.
	// Real implementation would re-queue to dead-letter or delayed set.
	return fmt.Errorf("not fully implemented for redis without active jobs tracking")
}
