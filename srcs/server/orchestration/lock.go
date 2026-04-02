package orchestration

import (
	"context"
	"errors"
	"fmt"
	"sync"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// DistributedLockManager handles distributed locking across Redis, Postgres, and SQLite
type DistributedLockManager struct {
	db          db.Provider
	redisClient rueidis.Client
	localLocks  sync.Map
}

// NewDistributedLockManager creates a new DistributedLockManager
func NewDistributedLockManager(provider db.Provider, redisClient rueidis.Client) *DistributedLockManager {
	return &DistributedLockManager{
		db:          provider,
		redisClient: redisClient,
	}
}

// AcquireLock attempts to acquire a distributed lock.
func (m *DistributedLockManager) AcquireLock(ctx context.Context, key, owner string, ttl time.Duration) (bool, error) {
	if m.redisClient != nil {
		// Acquire Redis-backed distributed lock with NX and PX parameters
		cmd := m.redisClient.B().Set().Key(key).Value(owner).Nx().Px(ttl).Build()
		err := m.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			if rueidis.IsRedisNil(err) {
				return false, nil // Lock already acquired
			}
			return false, fmt.Errorf("failed to acquire redis lock: %w", err)
		}
		return true, nil
	}

	if m.db != nil {
		if !m.db.IsSQLite() {
			// Postgres: use row-level locks on a dedicated distributed_locks table.
			// Try to insert a lock record. If it conflicts, check if it's expired.
			tx, err := m.db.Begin(ctx)
			if err != nil {
				return false, err
			}
			defer tx.Rollback(ctx)

			// Clean up expired lock for this key if it exists
			_, err = tx.Exec(ctx, "DELETE FROM distributed_locks WHERE lock_key = $1 AND expires_at < CURRENT_TIMESTAMP", key)
			if err != nil {
				return false, err
			}

			// Try to insert the new lock
			expiresAt := time.Now().Add(ttl).UTC()
			res, err := tx.Exec(ctx, "INSERT INTO distributed_locks (lock_key, owner, expires_at) VALUES ($1, $2, $3) ON CONFLICT DO NOTHING", key, owner, expiresAt)
			if err != nil {
				return false, err
			}

			if res == 0 {
				return false, nil // Lock already held
			}

			if err := tx.Commit(ctx); err != nil {
				return false, err
			}
			return true, nil
		}
	}

	// SQLite / Standalone fallback using sync.Map
	lockExpiry := time.Now().Add(ttl)
	actual, loaded := m.localLocks.LoadOrStore(key, struct {
		owner  string
		expiry time.Time
	}{owner, lockExpiry})

	if loaded {
		currentLock := actual.(struct {
			owner  string
			expiry time.Time
		})
		if time.Now().After(currentLock.expiry) {
			// Expired, we can overwrite it safely using CompareAndSwap
			swapped := m.localLocks.CompareAndSwap(key, actual, struct {
				owner  string
				expiry time.Time
			}{owner, lockExpiry})
			return swapped, nil
		}
		return false, nil
	}

	return true, nil
}

// ReleaseLock releases the distributed lock.
func (m *DistributedLockManager) ReleaseLock(ctx context.Context, key, owner string) error {
	if m.redisClient != nil {
		// Use Lua script to safely delete only if the owner matches
		script := `
		if redis.call("get", KEYS[1]) == ARGV[1] then
			return redis.call("del", KEYS[1])
		else
			return 0
		end`
		cmd := m.redisClient.B().Eval().Script(script).Numkeys(1).Key(key).Arg(owner).Build()
		err := m.redisClient.Do(ctx, cmd).Error()
		if err != nil {
			return fmt.Errorf("failed to release redis lock: %w", err)
		}
		return nil
	}

	if m.db != nil {
		if !m.db.IsSQLite() {
			// Postgres: delete the lock if we own it
			res, err := m.db.Exec(ctx, "DELETE FROM distributed_locks WHERE lock_key = $1 AND owner = $2", key, owner)
			if err != nil {
				return err
			}
			if res == 0 {
				return errors.New("lock not held by owner")
			}
			return nil
		}
	}

	// SQLite / Standalone fallback using sync.Map
	if actual, ok := m.localLocks.Load(key); ok {
		currentLock := actual.(struct {
			owner  string
			expiry time.Time
		})
		if currentLock.owner == owner {
			m.localLocks.Delete(key)
			return nil
		}
		return errors.New("lock not held by owner")
	}

	return nil
}
