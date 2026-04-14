package queue

import (
	"context"
	"fmt"

	"github.com/redis/rueidis"
)

type RedisJobQueue struct {
	client RedisClient
	prefix string
}

func NewRedisJobQueue(client RedisClient, prefix string) *RedisJobQueue {
	if prefix == "" {
		prefix = "ohc:jobs"
	}
	return &RedisJobQueue{client: client, prefix: prefix}
}

func (q *RedisJobQueue) topicKey(topic string) string {
	return fmt.Sprintf("%s:topic:%s", q.prefix, topic)
}

func (q *RedisJobQueue) Push(ctx context.Context, topic string, payload []byte) error {
	cmd := q.client.B().Lpush().Key(q.topicKey(topic)).Element(string(payload)).Build()
	return q.client.Do(ctx, cmd).Error()
}

func (q *RedisJobQueue) Pop(ctx context.Context, topic string) ([]byte, error) {
	cmd := q.client.B().Brpop().Key(q.topicKey(topic)).Timeout(0).Build()
	res, err := q.client.Do(ctx, cmd).AsStrSlice()
	if err != nil {
		if rueidis.IsRedisNil(err) || err == context.Canceled || err == context.DeadlineExceeded {
			return nil, nil // No jobs, or context canceled
		}
		return nil, err
	}
	if len(res) == 2 {
		return []byte(res[1]), nil
	}
	return nil, nil
}
