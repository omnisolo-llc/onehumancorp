package queue

import (
	"context"
	"encoding/json"
	"errors"
	"fmt"
	"strconv"
	"time"

	"github.com/google/uuid"
	"github.com/redis/rueidis"
	"onehumancorp/srcs/server/telemetry"
)

type RedisTaskQueue struct {
	client rueidis.Client
	prefix string
}

func NewRedisTaskQueue(client rueidis.Client, prefix string) *RedisTaskQueue {
	if prefix == "" {
		prefix = "ohc:queue:"
	}
	return &RedisTaskQueue{
		client: client,
		prefix: prefix,
	}
}

type jobData struct {
	Job
	Attempts    int `json:"attempts"`
	MaxAttempts int `json:"max_attempts"`
}

func (q *RedisTaskQueue) Enqueue(ctx context.Context, job *Job) error {
	if job.ID == "" {
		job.ID = uuid.New().String()
	}

	jd := jobData{
		Job:         *job,
		Attempts:    0,
		MaxAttempts: 3,
	}

	data, err := json.Marshal(jd)
	if err != nil {
		return err
	}

	jobKey := q.prefix + "job:" + job.ID
	setCmd := q.client.B().Set().Key(jobKey).Value(string(data)).Build()
	err = q.client.Do(ctx, setCmd).Error()
	if err != nil {
		return err
	}

	// Add to sorted set for delayed execution (now)
	now := float64(time.Now().UnixMilli())
	queueKey := q.prefix + "queued:" + job.AgentRole
	zaddCmd := q.client.B().Zadd().Key(queueKey).ScoreMember().ScoreMember(now, job.ID).Build()
	err = q.client.Do(ctx, zaddCmd).Error()
	if err != nil {
		return err
	}

	_ = telemetry.RecordQueueLength(ctx, 1, "redis")
	return nil
}

func (q *RedisTaskQueue) Dequeue(ctx context.Context, roles []string) (*Job, error) {
	if len(roles) == 0 {
		return nil, errors.New("no roles provided for dequeue")
	}

	now := float64(time.Now().UnixMilli())

	for _, role := range roles {
		queueKey := q.prefix + "queued:" + role

		// Attempt to grab a job
		// BZPOPMIN isn't ideal here because we want to limit by score (time <= now)
		// We use Lua script or atomic operations to safely dequeue

		// 1. Find a job ready to run
		zrangeCmd := q.client.B().Zrangebyscore().Key(queueKey).Min("-inf").Max(strconv.FormatFloat(now, 'f', -1, 64)).Limit(0, 1).Build()
		jobs, err := q.client.Do(ctx, zrangeCmd).AsStrSlice()
		if err != nil || len(jobs) == 0 {
			continue
		}

		jobID := jobs[0]

		// 2. Safely remove it from queue and move to running using ZPOPMIN logic or explicit ZREM
		zremCmd := q.client.B().Zrem().Key(queueKey).Member(jobID).Build()
		removed, err := q.client.Do(ctx, zremCmd).AsInt64()
		if err != nil || removed == 0 {
			continue // Lost the race
		}

		// It's ours now.
		jobKey := q.prefix + "job:" + jobID
		getCmd := q.client.B().Get().Key(jobKey).Build()
		data, err := q.client.Do(ctx, getCmd).AsBytes()
		if err != nil {
			// Job data missing?
			continue
		}

		var jd jobData
		if err := json.Unmarshal(data, &jd); err != nil {
			continue
		}

		jd.Attempts++

		// Save updated attempts
		newData, _ := json.Marshal(jd)
		setCmd := q.client.B().Set().Key(jobKey).Value(string(newData)).Build()
		_ = q.client.Do(ctx, setCmd).Error()

		// Add to running set to keep track
		runningKey := q.prefix + "running"
		zaddCmd := q.client.B().Zadd().Key(runningKey).ScoreMember().ScoreMember(float64(time.Now().Add(5*time.Minute).UnixMilli()), jobID).Build()
		_ = q.client.Do(ctx, zaddCmd).Error()

		_ = telemetry.RecordQueueLength(ctx, -1, "redis")

		return &jd.Job, nil
	}

	return nil, nil
}

func (q *RedisTaskQueue) Complete(ctx context.Context, jobID string) error {
	runningKey := q.prefix + "running"
	zremCmd := q.client.B().Zrem().Key(runningKey).Member(jobID).Build()
	_ = q.client.Do(ctx, zremCmd).Error()

	jobKey := q.prefix + "job:" + jobID
	delCmd := q.client.B().Del().Key(jobKey).Build()
	return q.client.Do(ctx, delCmd).Error()
}

func (q *RedisTaskQueue) Fail(ctx context.Context, jobID string, reason string) error {
	jobKey := q.prefix + "job:" + jobID
	getCmd := q.client.B().Get().Key(jobKey).Build()
	data, err := q.client.Do(ctx, getCmd).AsBytes()
	if err != nil {
		return err
	}

	var jd jobData
	if err := json.Unmarshal(data, &jd); err != nil {
		return err
	}

	// Remove from running
	runningKey := q.prefix + "running"
	zremCmd := q.client.B().Zrem().Key(runningKey).Member(jobID).Build()
	_ = q.client.Do(ctx, zremCmd).Error()

	if jd.Attempts >= jd.MaxAttempts {
		// Poison pill
		deadKey := q.prefix + "dead"
		zaddCmd := q.client.B().Zadd().Key(deadKey).ScoreMember().ScoreMember(float64(time.Now().UnixMilli()), jobID).Build()
		_ = q.client.Do(ctx, zaddCmd).Error()

		// Save reason
		jd.Payload = fmt.Sprintf(`{"error": %q, "original": %s}`, reason, jd.Payload)
		newData, _ := json.Marshal(jd)
		setCmd := q.client.B().Set().Key(jobKey).Value(string(newData)).Build()
		_ = q.client.Do(ctx, setCmd).Error()

	} else {
		// Retry
		backoffSeconds := jd.Attempts * jd.Attempts * 10
		runAfter := time.Now().Add(time.Duration(backoffSeconds) * time.Second)

		queueKey := q.prefix + "queued:" + jd.AgentRole
		zaddCmd := q.client.B().Zadd().Key(queueKey).ScoreMember().ScoreMember(float64(runAfter.UnixMilli()), jobID).Build()
		if err := q.client.Do(ctx, zaddCmd).Error(); err == nil {
			_ = telemetry.RecordQueueLength(ctx, 1, "redis")
		}
	}

	return nil
}
