package queue

import (
	"context"
	"encoding/json"
	"fmt"
		"github.com/redis/rueidis"
)

type RedisSubAgentTaskQueue struct {
	client rueidis.Client
	prefix string
	opts   QueueOptions
}

func NewRedisSubAgentTaskQueue(client rueidis.Client, prefix string, opts QueueOptions) *RedisSubAgentTaskQueue {
	if prefix == "" {
		prefix = "ohc:subagent:tasks"
	}
	return &RedisSubAgentTaskQueue{client: client, prefix: prefix, opts: opts}
}

func (q *RedisSubAgentTaskQueue) Enqueue(ctx context.Context, payload *SubAgentTaskQueuePayload) error {
	data, err := json.Marshal(payload)
	if err != nil {
		return err
	}
	hashKey := fmt.Sprintf("%s:job:%s", q.prefix, payload.JobID)
	cmdHash := q.client.B().Hset().Key(hashKey).FieldValue().FieldValue("payload", string(data)).FieldValue("retries", "0").Build()
	if err := q.client.Do(ctx, cmdHash).Error(); err != nil {
		return err
	}
	key := fmt.Sprintf("%s:%s:wait", q.prefix, payload.QueueName)
	cmd := q.client.B().Lpush().Key(key).Element(payload.JobID).Build()
	return q.client.Do(ctx, cmd).Error()
}

func (q *RedisSubAgentTaskQueue) Process(ctx context.Context, queueName string) (*SubAgentTaskQueuePayload, error) {
	key := fmt.Sprintf("%s:%s:wait", q.prefix, queueName)
	activeKey := fmt.Sprintf("%s:%s:active", q.prefix, queueName)
	cmd := q.client.B().Brpoplpush().Source(key).Destination(activeKey).Timeout(0).Build()
	res, err := q.client.Do(ctx, cmd).AsBytes()
	if err != nil {
		if err == context.Canceled || err == context.DeadlineExceeded || rueidis.IsRedisNil(err) {
			return nil, nil
		}
		return nil, err
	}

	jobID := string(res)
	if q.opts.RateLimitRate > 0 {
		rlKey := fmt.Sprintf("%s:%s:ratelimit", q.prefix, queueName)
		cmdIncr := q.client.B().Incr().Key(rlKey).Build()
		count, err := q.client.Do(ctx, cmdIncr).AsInt64()
		if err == nil {
			if count == 1 {
				cmdExp := q.client.B().Expire().Key(rlKey).Seconds(1).Build()
				_ = q.client.Do(ctx, cmdExp).Error()
			}
			if count > int64(q.opts.RateLimitRate) {
				// Rate limit exceeded, requeue and return nil so we don't stall everything endlessly
				// The outer loop will just sleep
				cmdPush := q.client.B().Lpush().Key(key).Element(jobID).Build()
				_ = q.client.Do(ctx, cmdPush).Error()
				cmdRem := q.client.B().Lrem().Key(activeKey).Count(1).Element(jobID).Build()
				_ = q.client.Do(ctx, cmdRem).Error()
				return nil, nil
			}
		}
	}

	hashKey := fmt.Sprintf("%s:job:%s", q.prefix, jobID)
	cmdHget := q.client.B().Hget().Key(hashKey).Field("payload").Build()
	data, err := q.client.Do(ctx, cmdHget).AsBytes()
	if err != nil {
		return nil, err
	}
	var payload SubAgentTaskQueuePayload
	if err := json.Unmarshal(data, &payload); err != nil {
		return nil, err
	}
	return &payload, nil
}

func (q *RedisSubAgentTaskQueue) Complete(ctx context.Context, jobID string, queueName string) error {
	activeKey := fmt.Sprintf("%s:%s:active", q.prefix, queueName)
	cmdRem := q.client.B().Lrem().Key(activeKey).Count(1).Element(jobID).Build()
	_ = q.client.Do(ctx, cmdRem).Error()
	hashKey := fmt.Sprintf("%s:job:%s", q.prefix, jobID)
	cmdDel := q.client.B().Del().Key(hashKey).Build()
	_ = q.client.Do(ctx, cmdDel).Error()
	return nil
}

func (q *RedisSubAgentTaskQueue) Fail(ctx context.Context, jobID string, queueName string, reason string) error {
	hashKey := fmt.Sprintf("%s:job:%s", q.prefix, jobID)
	cmdHincr := q.client.B().Hincrby().Key(hashKey).Field("retries").Increment(1).Build()
	retries, err := q.client.Do(ctx, cmdHincr).AsInt64()
	if err != nil {
		return err
	}
	activeKey := fmt.Sprintf("%s:%s:active", q.prefix, queueName)
	cmdRem := q.client.B().Lrem().Key(activeKey).Count(1).Element(jobID).Build()
	_ = q.client.Do(ctx, cmdRem).Error()
	if retries <= int64(q.opts.MaxRetries) {
		waitKey := fmt.Sprintf("%s:%s:wait", q.prefix, queueName)
		cmdPush := q.client.B().Lpush().Key(waitKey).Element(jobID).Build()
		return q.client.Do(ctx, cmdPush).Error()
	}
	if q.opts.DLQName != "" {
		dlqKey := fmt.Sprintf("%s:%s", q.prefix, q.opts.DLQName)
		cmd := q.client.B().Lpush().Key(dlqKey).Element(jobID).Build()
		return q.client.Do(ctx, cmd).Error()
	}
	cmdDel := q.client.B().Del().Key(hashKey).Build()
	_ = q.client.Do(ctx, cmdDel).Error()
	return nil
}
