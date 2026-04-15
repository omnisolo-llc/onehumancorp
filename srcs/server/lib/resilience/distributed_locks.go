package resilience

import (
	"context"
	"crypto/sha256"
	"fmt"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	"github.com/redis/rueidis"
)

// DistributedLock provides a standard interface for locking across the swarm.
type DistributedLock interface {
	Lock(ctx context.Context, ttl time.Duration) error
	Unlock(ctx context.Context) error
}

// Ensure interface compatibility
var _ DistributedLock = (*DummyLock)(nil)

type DummyLock struct{}

func (d *DummyLock) Lock(ctx context.Context, ttl time.Duration) error { return nil }
func (d *DummyLock) Unlock(ctx context.Context) error                  { return nil }

// RedisLock implements DistributedLock using rueidis.
type RedisLock struct {
	client rueidis.Client
	key    string
	token  string
}

func NewRedisLock(client rueidis.Client, key string, token string) *RedisLock {
	return &RedisLock{
		client: client,
		key:    key,
		token:  token,
	}
}

func (r *RedisLock) Lock(ctx context.Context, ttl time.Duration) error {
	cmd := r.client.B().Set().Key(r.key).Value(r.token).Nx().Px(ttl).Build()
	err := r.client.Do(ctx, cmd).Error()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return fmt.Errorf("lock already acquired")
		}
		return err
	}
	return nil
}

var unlockScript = rueidis.NewLuaScript(`
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`)

func (r *RedisLock) Unlock(ctx context.Context) error {
	return unlockScript.Exec(ctx, r.client, []string{r.key}, []string{r.token}).Error()
}

// PostgresLock implements DistributedLock using PostgreSQL advisory locks.
// It uses pg_try_advisory_lock based on a 64-bit integer derived from the key string.
type PostgresLock struct {
	provider db.Provider
	key      string
	lockID   int64
	tx       db.Tx
}

func NewPostgresLock(provider db.Provider, key string) *PostgresLock {
	// Generate a stable 64-bit ID from the key using SHA256.
	hash := sha256.Sum256([]byte(key))
	var lockID int64
	for i := 0; i < 8; i++ {
		lockID |= int64(hash[i]) << (8 * i)
	}

	return &PostgresLock{
		provider: provider,
		key:      key,
		lockID:   lockID,
	}
}

func (p *PostgresLock) Lock(ctx context.Context, ttl time.Duration) error {
	if p.provider.IsSQLite() {
		// Advisory locks are a Postgres concept. For SQLite fallback, do nothing.
		return nil
	}

	tx, err := p.provider.Begin(ctx)
	if err != nil {
		return fmt.Errorf("failed to begin tx for lock: %w", err)
	}
	p.tx = tx

	var acquired bool
	// xact level advisory locks automatically release when tx commits/rolls back.
	row := tx.QueryRow(ctx, "SELECT pg_try_advisory_xact_lock($1)", p.lockID)
	err = row.Scan(&acquired)
	if err != nil {
		tx.Rollback(ctx)
		p.tx = nil
		return err
	}
	if !acquired {
		tx.Rollback(ctx)
		p.tx = nil
		return fmt.Errorf("lock already acquired")
	}

	// We must also respect TTL. Since pg_try_advisory_xact_lock does not have TTL,
	// we spawn a goroutine to roll back the tx after TTL.
	go func() {
		select {
		case <-time.After(ttl):
			if p.tx != nil {
				// We don't care about the error, we just want to ensure it unlocks.
				p.tx.Rollback(context.Background())
			}
		case <-ctx.Done():
			if p.tx != nil {
				p.tx.Rollback(context.Background())
			}
		}
	}()

	return nil
}

func (p *PostgresLock) Unlock(ctx context.Context) error {
	if p.provider.IsSQLite() {
		return nil
	}

	if p.tx != nil {
		err := p.tx.Commit(ctx) // commit or rollback releases xact lock
		p.tx = nil
		return err
	}
	return nil
}
