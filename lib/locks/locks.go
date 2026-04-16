package locks

import (
	"context"
	"errors"
	"sync"
	"time"

	"github.com/redis/go-redis/v9"
)

var (
	ErrLockFailed = errors.New("failed to acquire lock")
)

// Locker defines the interface for distributed coordination locks.
type Locker interface {
	Acquire(ctx context.Context, resource string, ttl time.Duration) (bool, error)
	Release(ctx context.Context, resource string) error
}

// MemoryLocker implements a standalone in-memory mutex lock for standalone mode.
type MemoryLocker struct {
	mu    sync.Mutex
	locks map[string]time.Time
}

func NewMemoryLocker() *MemoryLocker {
	return &MemoryLocker{
		locks: make(map[string]time.Time),
	}
}

func (l *MemoryLocker) Acquire(ctx context.Context, resource string, ttl time.Duration) (bool, error) {
	l.mu.Lock()
	defer l.mu.Unlock()

	if expiresAt, exists := l.locks[resource]; exists {
		if time.Now().Before(expiresAt) {
			return false, nil // Lock is held
		}
	}
	l.locks[resource] = time.Now().Add(ttl)
	return true, nil
}

func (l *MemoryLocker) Release(ctx context.Context, resource string) error {
	l.mu.Lock()
	defer l.mu.Unlock()
	delete(l.locks, resource)
	return nil
}

// RedisLocker implements a distributed lock using Redis SETNX.
type RedisLocker struct {
	client *redis.Client
}

func NewRedisLocker(client *redis.Client) *RedisLocker {
	return &RedisLocker{
		client: client,
	}
}

func (l *RedisLocker) Acquire(ctx context.Context, resource string, ttl time.Duration) (bool, error) {
	ok, err := l.client.SetNX(ctx, "lock:"+resource, "1", ttl).Result()
	if err != nil {
		return false, err
	}
	return ok, nil
}

func (l *RedisLocker) Release(ctx context.Context, resource string) error {
	_, err := l.client.Del(ctx, "lock:"+resource).Result()
	return err
}
