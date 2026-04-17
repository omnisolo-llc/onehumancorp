package mesh

import (
	"context"
	"errors"
	"time"

	"github.com/redis/go-redis/v9"
)

// RedisMesh is the Cloud implementation using Redis Pub/Sub and Redlock principles.
type RedisMesh struct {
	client *redis.Client
}

func NewRedisMesh(client *redis.Client) *RedisMesh {
	return &RedisMesh{
		client: client,
	}
}

func (r *RedisMesh) Publish(ctx context.Context, channel, message string) error {
	return r.client.Publish(ctx, channel, message).Err()
}

func (r *RedisMesh) Subscribe(ctx context.Context, channel string) (<-chan string, error) {
	pubsub := r.client.Subscribe(ctx, channel)

	// Wait for confirmation that subscription is created before returning
	_, err := pubsub.Receive(ctx)
	if err != nil {
		pubsub.Close()
		return nil, err
	}

	redisCh := pubsub.Channel()
	ch := make(chan string, 100)

	go func() {
		defer pubsub.Close()
		defer close(ch)
		for {
			select {
			case <-ctx.Done():
				return
			case msg, ok := <-redisCh:
				if !ok {
					return
				}
				select {
				case ch <- msg.Payload:
				case <-ctx.Done():
					return
				}
			}
		}
	}()

	return ch, nil
}

var ErrLockNotAcquired = errors.New("failed to acquire lock")

func (r *RedisMesh) AcquireLock(ctx context.Context, key string) (func(), error) {
	lockKey := "mesh:lock:" + key
	// Generate a unique value for this lock instance to safely release it later.
	lockValue := "locked_" + time.Now().String()
	lockTimeout := 10 * time.Second

	ticker := time.NewTicker(50 * time.Millisecond)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return nil, ctx.Err()
		case <-ticker.C:
			acquired, err := r.client.SetNX(ctx, lockKey, lockValue, lockTimeout).Result()
			if err != nil {
				return nil, err
			}
			if acquired {
				// We acquired the lock
				releaseFunc := func() {
					// Lua script to safely release lock only if the value matches
					script := `
					if redis.call("get", KEYS[1]) == ARGV[1] then
						return redis.call("del", KEYS[1])
					else
						return 0
					end
					`
					r.client.Eval(context.Background(), script, []string{lockKey}, lockValue)
				}
				return releaseFunc, nil
			}
		}
	}
}
