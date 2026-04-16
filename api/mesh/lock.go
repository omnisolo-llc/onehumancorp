package mesh

import (
	"context"
	"fmt"
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
			select {
			case <-time.After(100 * time.Millisecond):
			case <-ctx.Done():
				return ctx.Err()
			}
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

// AcquireWithRedlock attempts to acquire a lock across multiple Redis instances
// to provide higher fault tolerance than standard Acquire. It requires a quorum of nodes.
func (l *DistributedLock) AcquireWithRedlock(ctx context.Context, expiration time.Duration, nodes []*redis.Client) error {
	if len(nodes) == 0 {
		return fmt.Errorf("no redis nodes provided for redlock")
	}

	quorum := (len(nodes) / 2) + 1
	acquired := 0
	driftFactor := 0.01 // drift factor based on Redlock spec

	// Loop to keep retrying until context ends
	for {
		select {
		case <-ctx.Done():
			return ctx.Err()
		default:
			acquired = 0
			startTime := time.Now()

			// Attempt to acquire lock in all instances
			for _, node := range nodes {
				// Use a shorter timeout for individual node operations
				nodeCtx, cancel := context.WithTimeout(ctx, 50*time.Millisecond)
				ok, err := node.SetNX(nodeCtx, l.key, l.value, expiration).Result()
				cancel()

				if err == nil && ok {
					acquired++
				}
			}

			elapsedTime := time.Since(startTime)
			drift := time.Duration(float64(expiration)*driftFactor) + 2*time.Millisecond
			validityTime := expiration - elapsedTime - drift

			if acquired >= quorum && validityTime > 0 {
				return nil // We have quorum and the lock is still valid!
			}

			// Failed to acquire quorum or validity time is negative. Must release from whatever we acquired it from.
			for _, node := range nodes {
				nodeCtx, cancel := context.WithTimeout(ctx, 50*time.Millisecond)
				script := `
				if redis.call("get", KEYS[1]) == ARGV[1] then
					return redis.call("del", KEYS[1])
				else
					return 0
				end`
				_ = node.Eval(nodeCtx, script, []string{l.key}, l.value).Err()
				cancel()
			}

			// Backoff before retrying
			select {
			case <-time.After(100 * time.Millisecond):
			case <-ctx.Done():
				return ctx.Err()
			}
		}
	}
}

// ReleaseRedlock releases the lock across all Redis instances.
func (l *DistributedLock) ReleaseRedlock(ctx context.Context, nodes []*redis.Client) error {
	script := `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`
	var lastErr error
	for _, node := range nodes {
		nodeCtx, cancel := context.WithTimeout(ctx, 100*time.Millisecond)
		err := node.Eval(nodeCtx, script, []string{l.key}, l.value).Err()
		cancel()
		if err != nil && err != redis.Nil {
			lastErr = err
		}
	}
	return lastErr
}
