package orchestration

import (
	"context"
	"fmt"
	"sync"
	"time"

	"github.com/go-redsync/redsync/v4"
	"github.com/go-redsync/redsync/v4/redis/goredis/v9"
	"github.com/redis/go-redis/v9"
)

type DistributedLock interface {
	Acquire(ctx context.Context, taskID string) (func() error, error)
}

type RedisLock struct {
	rs *redsync.Redsync
}

func NewRedisLock(client *redis.Client) *RedisLock {
	pool := goredis.NewPool(client)
	rs := redsync.New(pool)
	return &RedisLock{rs: rs}
}

func (l *RedisLock) Acquire(ctx context.Context, taskID string) (func() error, error) {
	mutex := l.rs.NewMutex(fmt.Sprintf("ohc:lock:task:%s", taskID), redsync.WithExpiry(5*time.Second))

	if err := mutex.LockContext(ctx); err != nil {
		return nil, fmt.Errorf("failed to acquire redis lock: %w", err)
	}

	return func() error {
		ok, err := mutex.UnlockContext(ctx)
		if err != nil {
			return err
		}
		if !ok {
			return fmt.Errorf("redis unlock failed")
		}
		return nil
	}, nil
}

type StandaloneLock struct {
	mu    sync.Mutex
	locks map[string]*sync.Mutex
}

func NewStandaloneLock() *StandaloneLock {
	return &StandaloneLock{
		locks: make(map[string]*sync.Mutex),
	}
}

func (l *StandaloneLock) Acquire(ctx context.Context, taskID string) (func() error, error) {
	l.mu.Lock()
	if _, ok := l.locks[taskID]; !ok {
		l.locks[taskID] = &sync.Mutex{}
	}
	taskMutex := l.locks[taskID]
	l.mu.Unlock()

	taskMutex.Lock()

	return func() error {
		taskMutex.Unlock()
		l.mu.Lock()
		delete(l.locks, taskID)
		l.mu.Unlock()
		return nil
	}, nil
}
