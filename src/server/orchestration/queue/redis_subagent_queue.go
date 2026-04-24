package queue

import (
	"github.com/onehumancorp/mono/src/server/telemetry"
	"context"
	"encoding/json"
	"fmt"
	"github.com/redis/rueidis"
)

type RedisSubAgentTaskQueue struct {
	client RedisClient
	prefix string
}

func NewRedisSubAgentTaskQueue(client RedisClient, prefix string) *RedisSubAgentTaskQueue {
	if prefix == "" {
		prefix = "ohc:subagent:tasks"
	}
	return &RedisSubAgentTaskQueue{client: client, prefix: prefix}
}

func (q *RedisSubAgentTaskQueue) Enqueue(ctx context.Context, payload *SubAgentTaskQueuePayload) error {
	data, err := json.Marshal(telemetry.RedactInterfacePII(payload))
	if err != nil {
		return err
	}
	key := fmt.Sprintf("%s:%s", q.prefix, payload.QueueName)
	cmd := q.client.B().Lpush().Key(key).Element(string(data)).Build()
	return q.client.Do(ctx, cmd).Error()
}

func (q *RedisSubAgentTaskQueue) Process(ctx context.Context, queueName string) (*SubAgentTaskQueuePayload, error) {
	key := fmt.Sprintf("%s:%s", q.prefix, queueName)
	cmd := q.client.B().Brpop().Key(key).Timeout(0).Build()
	res, err := q.client.Do(ctx, cmd).AsStrSlice()
	if err != nil {
		if err == context.Canceled || err == context.DeadlineExceeded || rueidis.IsRedisNil(err) {
			return nil, nil
		}
		return nil, err
	}

	if len(res) == 2 {
		var payload SubAgentTaskQueuePayload
		if err := json.Unmarshal([]byte(res[1]), &payload); err != nil {
			return nil, err
		}
		return &payload, nil
	}

	return nil, nil
}
