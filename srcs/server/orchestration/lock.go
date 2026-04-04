package orchestration

import (
	"context"
	"errors"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

var ErrLockNotAcquired = errors.New("lock not acquired")

// DistributedLock provides a simple acquire/release interface for agent coordination.
type DistributedLock interface {
	// Acquire attempts to acquire a lock for a specific duration.
	// Returns ErrLockNotAcquired if it cannot be acquired.
	Acquire(ctx context.Context, key string, ownerID string, ttl time.Duration) error

	// Release releases the lock if the owner matches.
	Release(ctx context.Context, key string, ownerID string) error
}

// RedisLock implements DistributedLock using rueidis SET NX EX.
type RedisLock struct {
	client rueidis.Client
}

func NewRedisLock(redisURL string) (*RedisLock, error) {
	opt, err := rueidis.ParseURL(redisURL)
	if err != nil {
		return nil, err
	}
	c, err := rueidis.NewClient(opt)
	if err != nil {
		return nil, err
	}
	return &RedisLock{client: c}, nil
}

func (rl *RedisLock) Acquire(ctx context.Context, key string, ownerID string, ttl time.Duration) error {
	cmd := rl.client.B().Set().Key("lock:" + key).Value(ownerID).Nx().Px(ttl).Build()
	err := rl.client.Do(ctx, cmd).Error()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return ErrLockNotAcquired
		}
		return err
	}
	return nil
}

func (rl *RedisLock) Release(ctx context.Context, key string, ownerID string) error {
	// Use Lua script to ensure we only release if we are the owner
	script := rueidis.NewLuaScript(`
if redis.call("get",KEYS[1]) == ARGV[1] then
    return redis.call("del",KEYS[1])
else
    return 0
end`)
	res, err := script.Exec(ctx, rl.client, []string{"lock:" + key}, []string{ownerID}).AsInt64()
	if err != nil {
		return err
	}
	if res == 0 {
		return errors.New("lock not owned or already expired")
	}
	return nil
}

// DatabaseLock implements DistributedLock using the distributed_locks table.
type DatabaseLock struct {
	db db.Provider
}

func NewDatabaseLock(db db.Provider) *DatabaseLock {
	return &DatabaseLock{db: db}
}

func (dl *DatabaseLock) Acquire(ctx context.Context, key string, ownerID string, ttl time.Duration) error {
	expiresAt := time.Now().Add(ttl)

	// Try to insert or update if expired
	query := `
		INSERT INTO distributed_locks (lock_key, owner_id, expires_at)
		VALUES ($1, $2, $3)
		ON CONFLICT(lock_key) DO UPDATE SET
			owner_id = EXCLUDED.owner_id,
			expires_at = EXCLUDED.expires_at,
			created_at = CURRENT_TIMESTAMP
		WHERE distributed_locks.expires_at < CURRENT_TIMESTAMP
	`

	// In SQLite, the syntax is slightly different for ON CONFLICT with a condition
	if dl.db.IsSQLite() {
		query = `
			INSERT INTO distributed_locks (lock_key, owner_id, expires_at)
			VALUES ($1, $2, $3)
			ON CONFLICT(lock_key) DO UPDATE SET
				owner_id = excluded.owner_id,
				expires_at = excluded.expires_at,
				created_at = CURRENT_TIMESTAMP
			WHERE distributed_locks.expires_at < CURRENT_TIMESTAMP
		`
	}

	affected, err := dl.db.Exec(ctx, query, key, ownerID, expiresAt)
	if err != nil {
		return fmt.Errorf("database lock error: %w", err)
	}

	if affected == 0 {
		// Verify if we actually own it (maybe we acquired it earlier)
		var currentOwner string
		var currentExpires time.Time
		err := dl.db.QueryRow(ctx, "SELECT owner_id, expires_at FROM distributed_locks WHERE lock_key = $1", key).Scan(&currentOwner, &currentExpires)
		if err == nil && currentOwner == ownerID && currentExpires.After(time.Now()) {
			return nil // We already own it
		}
		return ErrLockNotAcquired
	}

	return nil
}

func (dl *DatabaseLock) Release(ctx context.Context, key string, ownerID string) error {
	query := `DELETE FROM distributed_locks WHERE lock_key = $1 AND owner_id = $2`
	affected, err := dl.db.Exec(ctx, query, key, ownerID)
	if err != nil {
		return err
	}
	if affected == 0 {
		return errors.New("lock not owned or already expired")
	}
	return nil
}

// LockManager chooses the best locking mechanism based on environment.
func NewDistributedLockManager(redisURL string, db db.Provider) DistributedLock {
	if redisURL != "" {
		if rl, err := NewRedisLock(redisURL); err == nil {
			return rl
		}
	}
	return NewDatabaseLock(db)
}
