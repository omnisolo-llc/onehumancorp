package lock

import (
	"context"
	"database/sql"
	"errors"
	"fmt"
	"time"

	"github.com/jackc/pgx/v5"
	"github.com/google/uuid"
	"github.com/onehumancorp/mono/src/server/db"
)

// ErrLockNotAcquired is returned when a lock cannot be acquired
var ErrLockNotAcquired = errors.New("lock not acquired")

// Provider defines the interface for a distributed lock provider
type Provider interface {
	// TryLock attempts to acquire a lock for the given key.
	// If successful, returns true and an unlock function.
	// If unsuccessful, returns false and nil unlock function.
	TryLock(ctx context.Context, key string, ttl time.Duration) (bool, func(context.Context) error, error)
}

// DatabaseLockProvider implements a distributed lock using the database
type DatabaseLockProvider struct {
	db db.Provider
}

// NewDatabaseLockProvider creates a new DatabaseLockProvider
func NewDatabaseLockProvider(db db.Provider) *DatabaseLockProvider {
	return &DatabaseLockProvider{
		db: db,
	}
}

// TryLock attempts to acquire a lock using the database.
func (p *DatabaseLockProvider) TryLock(ctx context.Context, key string, ttl time.Duration) (bool, func(context.Context) error, error) {
	if p.db.IsSQLite() {
		return p.trySQLiteLock(ctx, key, ttl)
	}
	return p.tryPostgresLock(ctx, key, ttl)
}

func (p *DatabaseLockProvider) trySQLiteLock(ctx context.Context, key string, ttl time.Duration) (bool, func(context.Context) error, error) {
	token := uuid.New().String()

	// For SQLite, we can simulate an advisory lock using a dedicated locks table.
	// The Go database/sql driver with go-sqlite3 saves time.Time values directly, which are returned as strings in our previous test,
	// but using pure SQLite `datetime` functions might cause type mismatch or formatting issues depending on how the table was created
	// Let's use Go time to avoid SQLite `datetime` function mismatch if we're not sure how time is stored.

	now := time.Now().UTC()
	expiresAt := now.Add(ttl)

	query := `
		INSERT INTO distributed_locks (key, token, expires_at)
		VALUES (?, ?, ?)
		ON CONFLICT(key) DO UPDATE SET
			token = excluded.token,
			expires_at = excluded.expires_at
		WHERE distributed_locks.expires_at < ?
		RETURNING token
	`

	var returnedToken string
	row := p.db.QueryRow(ctx, query, key, token, expiresAt, now)
	err := row.Scan(&returnedToken)
	if err != nil {
		if errors.Is(err, sql.ErrNoRows) || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return false, nil, nil
		}
		return false, nil, fmt.Errorf("failed to acquire sqlite lock: %w", err)
	}

	unlock := func(unlockCtx context.Context) error {
		_, err := p.db.Exec(unlockCtx, "DELETE FROM distributed_locks WHERE key = ? AND token = ?", key, token)
		return err
	}

	return true, unlock, nil
}

func (p *DatabaseLockProvider) tryPostgresLock(ctx context.Context, key string, ttl time.Duration) (bool, func(context.Context) error, error) {
	token := uuid.New().String()

	// PostgreSQL ON CONFLICT syntax with database-native timestamps
	ttlSeconds := ttl.Seconds()

	query := fmt.Sprintf(`
		INSERT INTO distributed_locks (key, token, expires_at)
		VALUES ($1, $2, CURRENT_TIMESTAMP + interval '%f seconds')
		ON CONFLICT (key) DO UPDATE SET
			token = EXCLUDED.token,
			expires_at = EXCLUDED.expires_at
		WHERE distributed_locks.expires_at < CURRENT_TIMESTAMP
		RETURNING token
	`, ttlSeconds)

	var returnedToken string
	row := p.db.QueryRow(ctx, query, key, token)
	err := row.Scan(&returnedToken)
	if err != nil {
		if errors.Is(err, pgx.ErrNoRows) || err.Error() == "no rows in result set" || err.Error() == "sql: no rows in result set" {
			return false, nil, nil
		}
		return false, nil, fmt.Errorf("failed to acquire postgres lock: %w", err)
	}

	unlock := func(unlockCtx context.Context) error {
		_, err := p.db.Exec(unlockCtx, "DELETE FROM distributed_locks WHERE key = $1 AND token = $2", key, token)
		return err
	}

	return true, unlock, nil
}
