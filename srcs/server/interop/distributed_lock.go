package interop

import (
	"context"
	"errors"
	"fmt"
	"os"
	"sync"
	"time"

	"github.com/redis/rueidis"
)

var (
	ErrLockNotAcquired = errors.New("lock could not be acquired")
	ErrLockNotHeld     = errors.New("lock is not held")
)

// DistributedLock defines the interface for acquiring and releasing a lock
// to prevent agent conflicts across the swarm.
type DistributedLock interface {
	// TryLock attempts to acquire the lock using a unique token. Returns nil if successful.
	// Returns ErrLockNotAcquired if the lock is already held.
	TryLock(ctx context.Context, key string, token string, ttl time.Duration) error

	// Unlock releases the lock only if the token matches.
	Unlock(ctx context.Context, key string, token string) error
}

// NewDistributedLock returns a distributed lock depending on the execution mode.
// If REDIS_URL is present and OHC_STANDALONE is not true, it returns a Redis lock.
// Otherwise, it returns an in-memory lock (for Standalone Desktop Mode).
func NewDistributedLock() (DistributedLock, error) {
	redisURL := os.Getenv("REDIS_URL")
	if redisURL != "" && os.Getenv("OHC_STANDALONE") != "true" {
		opts, err := rueidis.ParseURL(redisURL)
		if err != nil {
			return nil, fmt.Errorf("failed to parse REDIS_URL: %w", err)
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			return nil, fmt.Errorf("failed to connect to redis: %w", err)
		}
		return &redisLock{client: c}, nil
	}
	return &memoryLock{locks: make(map[string]memoryLockItem)}, nil
}

// memoryLock provides a local in-memory lock for Standalone mode.
type memoryLock struct {
	mu    sync.Mutex
	locks map[string]memoryLockItem
}

type memoryLockItem struct {
	token      string
	expiration time.Time
}

func (m *memoryLock) TryLock(ctx context.Context, key string, token string, ttl time.Duration) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	now := time.Now()
	if item, exists := m.locks[key]; exists {
		if now.Before(item.expiration) {
			if item.token == token {
				// Re-acquiring or extending lock
				m.locks[key] = memoryLockItem{token: token, expiration: now.Add(ttl)}
				return nil
			}
			return ErrLockNotAcquired
		}
	}
	m.locks[key] = memoryLockItem{token: token, expiration: now.Add(ttl)}
	return nil
}

func (m *memoryLock) Unlock(ctx context.Context, key string, token string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

	if item, exists := m.locks[key]; exists {
		if item.token != token {
			return ErrLockNotHeld // Or a specific error for "not owner"
		}
		delete(m.locks, key)
		return nil
	}
	return ErrLockNotHeld
}

// redisLock provides a distributed lock using Redis via SET NX EX.
type redisLock struct {
	client rueidis.Client
}

func (r *redisLock) TryLock(ctx context.Context, key string, token string, ttl time.Duration) error {
	cmd := r.client.B().Set().Key("lock:" + key).Value(token).Nx().Ex(ttl).Build()
	err := r.client.Do(ctx, cmd).Error()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return ErrLockNotAcquired
		}
		return fmt.Errorf("redis set nx error: %w", err)
	}
	return nil
}

const unlockScript = `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`

func (r *redisLock) Unlock(ctx context.Context, key string, token string) error {
	script := rueidis.NewLuaScript(unlockScript)
	cmd := script.Exec(ctx, r.client, []string{"lock:" + key}, []string{token})

	val, err := cmd.AsInt64()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return ErrLockNotHeld
		}
		return fmt.Errorf("redis unlock error: %w", err)
	}
	if val == 0 {
		return ErrLockNotHeld
	}
	return nil
}
