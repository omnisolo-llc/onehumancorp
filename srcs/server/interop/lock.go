package interop

import (
	"context"
	"sync"
	"time"

	"github.com/redis/go-redis/v9"
)

type DistributedLock interface {
	AcquireLock(ctx context.Context, resource string, owner string, ttlSeconds uint64) (bool, error)
	ReleaseLock(ctx context.Context, resource string, owner string) error
}

type RedisLock struct {
	client interface {
		SetNX(ctx context.Context, key string, value interface{}, expiration time.Duration) *redis.BoolCmd
		Eval(ctx context.Context, script string, keys []string, args ...interface{}) *redis.Cmd
	}
}

func NewRedisLock(client interface {
	SetNX(ctx context.Context, key string, value interface{}, expiration time.Duration) *redis.BoolCmd
	Eval(ctx context.Context, script string, keys []string, args ...interface{}) *redis.Cmd
}) *RedisLock {
	return &RedisLock{
		client: client,
	}
}

func (l *RedisLock) AcquireLock(ctx context.Context, resource string, owner string, ttlSeconds uint64) (bool, error) {
	expirationMs := ttlSeconds * 1000

	// Either SET NX (acquire new lock) or if we already own it, extend it.
	script := `
		local current_owner = redis.call("get", KEYS[1])
		if not current_owner or current_owner == ARGV[1] then
			redis.call("set", KEYS[1], ARGV[1], "PX", ARGV[2])
			return 1
		else
			return 0
		end
	`
	cmd := l.client.Eval(ctx, script, []string{resource}, owner, expirationMs)
	res, err := cmd.Result()
	if err != nil {
		return false, err
	}

	val, ok := res.(int64)
	return ok && val == 1, nil
}

func (l *RedisLock) ReleaseLock(ctx context.Context, resource string, owner string) error {
	script := `
		if redis.call("get", KEYS[1]) == ARGV[1] then
			return redis.call("del", KEYS[1])
		else
			return 0
		end
	`
	cmd := l.client.Eval(context.WithoutCancel(ctx), script, []string{resource}, owner)
	_, err := cmd.Result()
	return err
}

type MemoryLock struct {
	mu    sync.Mutex
	locks map[string]lockEntry
}

type lockEntry struct {
	owner     string
	expiresAt time.Time
}

func NewMemoryLock() *MemoryLock {
	return &MemoryLock{
		locks: make(map[string]lockEntry),
	}
}

func (l *MemoryLock) AcquireLock(ctx context.Context, resource string, owner string, ttlSeconds uint64) (bool, error) {
	l.mu.Lock()
	defer l.mu.Unlock()

	now := time.Now()
	if entry, ok := l.locks[resource]; ok {
		if now.Before(entry.expiresAt) && entry.owner != owner {
			return false, nil
		}
	}

	l.locks[resource] = lockEntry{
		owner:     owner,
		expiresAt: now.Add(time.Duration(ttlSeconds) * time.Second),
	}

	return true, nil
}

func (l *MemoryLock) ReleaseLock(ctx context.Context, resource string, owner string) error {
	l.mu.Lock()
	defer l.mu.Unlock()

	if entry, ok := l.locks[resource]; ok {
		if entry.owner == owner {
			delete(l.locks, resource)
		}
	}

	return nil
}
