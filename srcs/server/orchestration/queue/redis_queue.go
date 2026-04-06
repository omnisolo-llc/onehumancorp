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
	return &RedisTaskQueue{
		client: client,
	}
}

func (q *RedisTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	if job.Status == "" {
		job.Status = "QUEUED"
	}
	if job.RunAfter.IsZero() {
		job.RunAfter = time.Now()
	}

	jobBytes, err := json.Marshal(job)
	if err != nil {
		return err
	}

	queueKey := "sub_agent_jobs:role:" + job.AgentRole

	// Use sorted set if it has a future run_after time, otherwise push to list
	if job.RunAfter.After(time.Now()) {
		delayedKey := queueKey + ":delayed"
		cmd := q.client.B().Zadd().Key(delayedKey).ScoreMember().ScoreMember(float64(job.RunAfter.UnixMilli()), string(jobBytes)).Build()
		err = q.client.Do(ctx, cmd).Error()
	} else {
		cmd := q.client.B().Rpush().Key(queueKey).Element(string(jobBytes)).Build()
		err = q.client.Do(ctx, cmd).Error()
	}

	if err != nil {
		return fmt.Errorf("failed to enqueue to redis: %w", err)
	}

	telemetry.RecordTaskQueueLength(ctx, 1)
	return nil
}

func (q *RedisTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	if len(roles) == 0 {
		return nil, nil
	}

	// Check all roles
	for _, role := range roles {
		queueKey := "sub_agent_jobs:role:" + role
		delayedKey := queueKey + ":delayed"

		now := float64(time.Now().UnixMilli())

		// Move delayed to main list (simplified)
		cmdRange := q.client.B().Zrangebyscore().Key(delayedKey).Min("0").Max(fmt.Sprintf("%f", now)).Build()
		resp := q.client.Do(ctx, cmdRange)
		if err := resp.Error(); err == nil {
			if items, err := resp.AsStrSlice(); err == nil && len(items) > 0 {
				for _, item := range items {
					zremCmd := q.client.B().Zrem().Key(delayedKey).Member(item).Build()
					if removed, _ := q.client.Do(ctx, zremCmd).AsInt64(); removed > 0 {
						rpushCmd := q.client.B().Rpush().Key(queueKey).Element(item).Build()
						q.client.Do(ctx, rpushCmd)
					}
				}
			}
		}

		lpopCmd := q.client.B().Lpop().Key(queueKey).Build()
		itemStr, err := q.client.Do(ctx, lpopCmd).ToString()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				continue // Try next role
			}
			return nil, err
		}

		var job Job
		if err := json.Unmarshal([]byte(itemStr), &job); err != nil {
			return nil, fmt.Errorf("failed to unmarshal job: %w", err)
		}

		job.Status = "RUNNING"
		job.Attempts++

		telemetry.RecordTaskQueueLength(ctx, -1)

		return &job, nil
	}

	return nil, nil
}

func (q *RedisTaskQueue) Complete(ctx context.Context, jobID string) error {
	// In simple LPOP implementation, job is already removed
	return nil
}

func (q *RedisTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	// Requires re-enqueueing the job, but we'd need the full job object.
	// This simple design assumes the orchestrator will handle state, or we need to fetch it.
	// For now, this is a no-op as the job is removed from queue.
	return nil
}
