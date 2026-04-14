package interop

import (
	"context"
	"log/slog"
	"os"
	"sync"
	"time"

	"github.com/google/uuid"
	"github.com/redis/rueidis"
)

// DistributedLock provides an interface for distributed locking.
type DistributedLock interface {
	Lock(ctx context.Context, key string, ttl time.Duration) (bool, error)
	Unlock(ctx context.Context, key string) error
}

// NewDistributedLock returns a new DistributedLock depending on the execution mode.
func NewDistributedLock() (DistributedLock, error) {
	redisURL := os.Getenv("REDIS_URL")
	if redisURL != "" && os.Getenv("OHC_STANDALONE") != "true" {
		opts, err := rueidis.ParseURL(redisURL)
		if err != nil {
			slog.Warn("failed to parse REDIS_URL, falling back to memory lock", "error", err)
			return &memoryLock{locks: make(map[string]lockEntry)}, nil
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			slog.Warn("failed to connect to redis, falling back to memory lock", "error", err)
			return &memoryLock{locks: make(map[string]lockEntry)}, nil
		}
		slog.Info("DistributedLock initialized in Cloud mode (Redis)")
		return &cloudLock{client: c, token: uuid.New().String()}, nil
	}

	slog.Info("DistributedLock initialized in Standalone mode (In-Memory)")
	return &memoryLock{
		locks: make(map[string]lockEntry),
	}, nil
}

type lockEntry struct {
    expiry time.Time
    token string
}

// memoryLock provides a local in-memory lock implementation.
type memoryLock struct {
	mu    sync.Mutex
	locks map[string]lockEntry
    token string
}

func (m *memoryLock) Lock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	m.mu.Lock()
	defer m.mu.Unlock()

    if m.token == "" {
        m.token = uuid.New().String()
    }

	// Check if already locked and not expired
	if entry, ok := m.locks[key]; ok {
		if time.Now().Before(entry.expiry) {
			return false, nil // Already locked
		}
	}

	// Grant lock
	m.locks[key] = lockEntry{expiry: time.Now().Add(ttl), token: m.token}
	return true, nil
}

func (m *memoryLock) Unlock(ctx context.Context, key string) error {
	m.mu.Lock()
	defer m.mu.Unlock()

    if entry, ok := m.locks[key]; ok {
        if entry.token == m.token {
            delete(m.locks, key)
        }
    }
	return nil
}

// cloudLock provides a Redis backed lock using rueidis.
type cloudLock struct {
	client rueidis.Client
    token  string
}

func (c *cloudLock) Lock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	// SET key value NX PX ttl
	cmd := c.client.B().Set().Key(key).Value(c.token).Nx().Px(ttl).Build()
	err := c.client.Do(ctx, cmd).Error()
	if err != nil {
		if rueidis.IsRedisNil(err) {
			return false, nil // Not locked
		}
		return false, err
	}
	return true, nil
}

var unlockScript = rueidis.NewLuaScript(`
if redis.call("get", KEYS[1]) == ARGV[1] then
    return redis.call("del", KEYS[1])
else
    return 0
end
`)

func (c *cloudLock) Unlock(ctx context.Context, key string) error {
	err := unlockScript.Exec(ctx, c.client, []string{key}, []string{c.token}).Error()
	return err
}
