package mesh

import (
	"context"
	"time"

	"github.com/google/uuid"
	"github.com/redis/go-redis/v9"
)

// DistributedLock provides a robust Redis-based distributed lock.
type DistributedLock struct {
	client *redis.Client
	key    string
	value  string
}

// NewDistributedLock creates a new DistributedLock instance.
func NewDistributedLock(client *redis.Client, key string) *DistributedLock {
	return &DistributedLock{
		client: client,
		key:    "lock:" + key,
		value:  uuid.New().String(), // Unique identifier for this lock instance
	}
}

// Acquire attempts to acquire the lock. It blocks until the lock is acquired
// or the context is cancelled.
func (l *DistributedLock) Acquire(ctx context.Context, expiration time.Duration) error {
	for {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			// Try to acquire the lock
			ok, err := l.client.SetNX(ctx, l.key, l.value, expiration).Result()
			if err != nil {
				return err
			}
			if ok {
				return nil // Lock acquired
			}
			// Backoff before retrying
			time.Sleep(100 * time.Millisecond)
		}
	}
}

// Release releases the lock using a Lua script to ensure only the owner can release it.
func (l *DistributedLock) Release(ctx context.Context) error {
	script := `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`
	err := l.client.Eval(ctx, script, []string{l.key}, l.value).Err()
	if err == redis.Nil {
		return nil
	}
	return err
}
