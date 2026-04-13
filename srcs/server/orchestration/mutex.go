package orchestration

import (
	"context"
	"errors"
	"fmt"
	"strings"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

var (
	ErrLockAcquisitionFailed = errors.New("failed to acquire lock")
	ErrLockNotOwned          = errors.New("lock is not owned or has expired")
)

// Mutex defines the interface for a distributed lock.
type Mutex interface {
	// Lock attempts to acquire the lock. It should return ErrLockAcquisitionFailed if it cannot be acquired.
	Lock(ctx context.Context, ttl time.Duration) error
	// Unlock releases the lock.
	Unlock(ctx context.Context) error
}

// MutexProvider creates mutexes for given keys.
type MutexProvider interface {
	NewMutex(key string) Mutex
}

// NewMutexProvider creates the appropriate MutexProvider based on the environment.
func NewMutexProvider(ctx context.Context, provider db.Provider, redisClient rueidis.Client) (MutexProvider, error) {
	if redisClient != nil {
		return &RedisMutexProvider{client: redisClient}, nil
	}

	// For SQLite/DB, ensure the table exists when the provider is created
	query := `
		CREATE TABLE IF NOT EXISTS distributed_locks (
			lock_key TEXT PRIMARY KEY,
			owner_id TEXT NOT NULL,
			expires_at DATETIME NOT NULL
		);
	`
	if _, err := provider.Exec(ctx, query); err != nil {
		return nil, fmt.Errorf("failed to initialize distributed_locks table: %w", err)
	}

	return &SQLiteMutexProvider{db: provider}, nil
}

// RedisMutexProvider uses Redis for distributed locking.
type RedisMutexProvider struct {
	client rueidis.Client
}

func (p *RedisMutexProvider) NewMutex(key string) Mutex {
	return &RedisMutex{
		client:  p.client,
		key:     fmt.Sprintf("ohc:lock:%s", key),
		ownerID: generateID(),
	}
}

type RedisMutex struct {
	client  rueidis.Client
	key     string
	ownerID string
}

func (m *RedisMutex) Lock(ctx context.Context, ttl time.Duration) error {
	cmd := m.client.B().Set().Key(m.key).Value(m.ownerID).Nx().Px(ttl).Build()
	err := m.client.Do(ctx, cmd).Error()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return ErrLockAcquisitionFailed
		}
		return fmt.Errorf("redis set error: %w", err)
	}
	return nil
}

func (m *RedisMutex) Unlock(ctx context.Context) error {
	script := `
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`
	cmd := m.client.B().Eval().Script(script).Numkeys(1).Key(m.key).Arg(m.ownerID).Build()
	val, err := m.client.Do(ctx, cmd).AsInt64()
	if err != nil {
		return fmt.Errorf("redis eval error: %w", err)
	}
	if val == 0 {
		return ErrLockNotOwned
	}
	return nil
}

// SQLiteMutexProvider uses a database table for locking.
type SQLiteMutexProvider struct {
	db db.Provider
}

func (p *SQLiteMutexProvider) NewMutex(key string) Mutex {
	return &SQLiteMutex{
		provider: p,
		key:      key,
		ownerID:  generateID(),
	}
}

type SQLiteMutex struct {
	provider *SQLiteMutexProvider
	key      string
	ownerID  string
}

func (m *SQLiteMutex) Lock(ctx context.Context, ttl time.Duration) error {
	tx, err := m.provider.db.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx: %w", err)
	}
	defer tx.Rollback(ctx)

	nowStr := time.Now().Format(time.RFC3339Nano)
	_, _ = tx.Exec(ctx, "DELETE FROM distributed_locks WHERE lock_key = $1 AND expires_at < $2", m.key, nowStr)

	expiresAt := time.Now().Add(ttl)

	query := `
		INSERT INTO distributed_locks (lock_key, owner_id, expires_at)
		VALUES ($1, $2, $3)
	`
	_, execErr := tx.Exec(ctx, query, m.key, m.ownerID, expiresAt.Format(time.RFC3339Nano))

	if execErr != nil {
		if strings.Contains(execErr.Error(), "UNIQUE constraint failed") || strings.Contains(execErr.Error(), "duplicate key value") {
			return ErrLockAcquisitionFailed
		}
		return fmt.Errorf("database insert error: %w", execErr)
	}

	if err := tx.Commit(ctx); err != nil {
		return fmt.Errorf("failed to commit tx: %w", err)
	}
	return nil
}

func (m *SQLiteMutex) Unlock(ctx context.Context) error {
	query := `DELETE FROM distributed_locks WHERE lock_key = $1 AND owner_id = $2`
	res, err := m.provider.db.Exec(ctx, query, m.key, m.ownerID)
	if err != nil {
		return err
	}

	rowsAffected := res
	if rowsAffected == 0 {
		return ErrLockNotOwned
	}
	return nil
}
