package orchestration

import (
	"context"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/redis/rueidis"
)

// DistributedLock provides an interface for distributed locking.
type DistributedLock interface {
	Lock(ctx context.Context, ttl time.Duration) error
	Unlock(ctx context.Context) error
}

// DistributedLockProvider provides distributed locks.
type DistributedLockProvider interface {
	NewLock(key string) DistributedLock
}

// NewDistributedLockProvider creates a lock provider based on the environment.
// For Cloud mode, it delegates to Redis (via rueidis).
// For Standalone mode, it delegates to SQLite.
func NewDistributedLockProvider(ctx context.Context, provider db.Provider, redisClient rueidis.Client) (DistributedLockProvider, error) {
	mp, err := NewMutexProvider(ctx, provider, redisClient)
	if err != nil {
		return nil, err
	}
	return &distributedLockProviderImpl{mp: mp}, nil
}

type distributedLockProviderImpl struct {
	mp MutexProvider
}

func (p *distributedLockProviderImpl) NewLock(key string) DistributedLock {
	return p.mp.NewMutex(key)
}
