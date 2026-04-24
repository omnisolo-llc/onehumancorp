package interop

import (
	"context"
	"log/slog"
	"os"
	"path/filepath"
	"strings"
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
		if os.Getenv("DISABLE_REDIS_CACHE") == "true" {
			opts.DisableCache = true
		}
		if err != nil {
			slog.Warn("failed to parse REDIS_URL, falling back to memory lock", "error", err)
			return &memoryLock{}, nil
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			slog.Warn("failed to connect to redis, falling back to memory lock", "error", err)
			return &memoryLock{}, nil
		}
		slog.Info("DistributedLock initialized in Cloud mode (Redis)")
		return &cloudLock{client: c, token: uuid.New().String()}, nil
	}

	slog.Info("DistributedLock initialized in Standalone mode (In-Memory)")
	return &memoryLock{}, nil
}

// memoryLock provides a local file-based lock implementation.
type memoryLock struct {
	token string
}

func (m *memoryLock) Lock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	if m.token == "" {
		m.token = uuid.New().String()
	}

	safeKey := strings.ReplaceAll(key, "/", "_")
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	err := os.Mkdir(path, 0777)
	if err != nil {
		if os.IsExist(err) {
			// Check if it's actually a file from a previous version and remove it
			info, err := os.Stat(path)
			if err == nil && !info.IsDir() {
				os.Remove(path)
				return m.Lock(ctx, key, ttl)
			}

			metaPath := filepath.Join(path, "meta.txt")
			content, err := os.ReadFile(metaPath)
			if err == nil {
				parts := strings.SplitN(string(content), ",", 2)
				if len(parts) == 2 {
					expiryTime, parseErr := time.Parse(time.RFC3339Nano, parts[0])
					if parseErr == nil && time.Now().After(expiryTime) {
						// Expired lock found. To avoid TOCTOU, we steal it using rename.
						stealPath := path + "_steal_" + uuid.New().String()
						err = os.Rename(path, stealPath)
						if err == nil {
							os.RemoveAll(stealPath)
							err = os.Mkdir(path, 0777)
							if err != nil {
								return false, nil
							}
							goto acquired
						}
					}
				}
			}
			return false, nil
		}
		return false, err
	}

acquired:
	expiry := time.Now().Add(ttl).Format(time.RFC3339Nano)
	metaPath := filepath.Join(path, "meta.txt")
	err = os.WriteFile(metaPath, []byte(expiry+","+m.token), 0666)
	if err != nil {
		os.RemoveAll(path)
		return false, err
	}

	return true, nil
}

func (m *memoryLock) Unlock(ctx context.Context, key string) error {
	safeKey := strings.ReplaceAll(key, "/", "_")
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	// We must check if the lock belongs to us BEFORE attempting to steal/rename it.
	// Otherwise, a late Unlock call from an expired owner could temporarily hide the real owner's lock directory.
	metaPath := filepath.Join(path, "meta.txt")
	content, err := os.ReadFile(metaPath)
	if err != nil {
		return nil // File not readable or doesn't exist, ignore
	}

	parts := strings.SplitN(string(content), ",", 2)
	if len(parts) != 2 || parts[1] != m.token {
		return nil // Not our lock
	}

	// It's our lock, so we can safely try to rename it away.
	tempPath := path + "_" + uuid.New().String() + ".tmp"
	err = os.Rename(path, tempPath)
	if err != nil {
		return nil // File might already be gone
	}

	// Optional safety check: ensure the token didn't change right as we renamed it
	content, err = os.ReadFile(filepath.Join(tempPath, "meta.txt"))
	if err == nil {
		parts = strings.SplitN(string(content), ",", 2)
		if len(parts) == 2 && parts[1] == m.token {
			return os.RemoveAll(tempPath)
		}
	}

	// If not our lock after all, put it back
	os.Rename(tempPath, path)
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
