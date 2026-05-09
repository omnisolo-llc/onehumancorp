package mesh

import (
	"context"
	"fmt"
	"time"

	"github.com/redis/go-redis/v9"
	"onehumancorp/srcs/server/pb"
)

type RedisTransport struct {
	client *redis.Client
}

func NewRedisTransport(url string) (*RedisTransport, error) {
	opts, err := redis.ParseURL(url)
	if err != nil {
		return nil, fmt.Errorf("failed to parse redis url: %w", err)
	}

	client := redis.NewClient(opts)
	if err := client.Ping(context.Background()).Err(); err != nil {
		return nil, fmt.Errorf("failed to ping redis: %w", err)
	}

	return &RedisTransport{client: client}, nil
}

func (t *RedisTransport) Publish(ctx context.Context, channel string, data []byte) error {
	var retries int
	for {
		err := t.client.Publish(ctx, channel, data).Err()
		if err == nil {
			return nil
		}
		if retries >= 3 {
			return fmt.Errorf("failed to publish to redis after retries: %w", err)
		}
		retries++
		time.Sleep(time.Duration(100*retries) * time.Millisecond)
	}
}

func (t *RedisTransport) Subscribe(ctx context.Context, channel string, handler func(data []byte)) error {
	pubsub := t.client.Subscribe(ctx, channel)
	ch := pubsub.Channel()

	go func() {
		for {
			select {
			case <-ctx.Done():
				pubsub.Close()
				return
			case msg, ok := <-ch:
				if !ok {
					return
				}
				go handler([]byte(msg.Payload))
			}
		}
	}()

	return nil
}

func (t *RedisTransport) AcquireLock(ctx context.Context, resource, owner string, ttlSeconds int) (bool, error) {
	ttl := time.Duration(ttlSeconds) * time.Second
	return t.client.SetNX(ctx, resource, owner, ttl).Result()
}

func (t *RedisTransport) ReleaseLock(ctx context.Context, resource, owner string) error {
	script := `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`
	return t.client.Eval(ctx, script, []string{resource}, owner).Err()
}

func (t *RedisTransport) AdvertiseCapabilities(ctx context.Context, agent pb.Agent) error {
	return nil
}

func (t *RedisTransport) DiscoverAgents(ctx context.Context, skill string) ([]pb.Agent, error) {
	return nil, nil
}

func (t *RedisTransport) StartHeartbeat(ctx context.Context, agent pb.Agent) {}
