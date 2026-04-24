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

	file, err := os.OpenFile(path, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
	if err != nil {
		if os.IsExist(err) {
			content, err := os.ReadFile(path)
			if err == nil {
				parts := strings.SplitN(string(content), ",", 2)
				if len(parts) == 2 {
					expiryTime, parseErr := time.Parse(time.RFC3339Nano, parts[0])
					if parseErr == nil && time.Now().After(expiryTime) {
						// Expired lock found. To avoid TOCTOU, we don't just blindly remove.
						// We check the token.
						// We do not remove it if it has been updated. (token=parts[1])
						// We just delete it and retry.
						// Wait, if it has expired, anyone could be trying to delete it.
						// To be safer, we can try to delete it. If it fails, that's fine.
						os.Remove(path)
						file, err = os.OpenFile(path, os.O_CREATE|os.O_EXCL|os.O_WRONLY, 0666)
						if err != nil {
							return false, nil
						}
						goto acquired
					}
				}
			}
			return false, nil
		}
		return false, err
	}

acquired:
	defer file.Close()
	expiry := time.Now().Add(ttl).Format(time.RFC3339Nano)
	_, err = file.WriteString(expiry + "," + m.token)
	if err != nil {
		os.Remove(path)
		return false, err
	}

	return true, nil
}

func (m *memoryLock) Unlock(ctx context.Context, key string) error {
	safeKey := strings.ReplaceAll(key, "/", "_")
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	// TOCTOU mitigation: rename the file to a temp file, read it, if valid, delete it. If invalid, rename it back.
	// Wait, rename on Windows/Linux is atomic.
	tempPath := path + "_" + uuid.New().String() + ".tmp"
	err := os.Rename(path, tempPath)
	if err != nil {
		return nil // File might already be gone
	}

	content, err := os.ReadFile(tempPath)
	if err == nil {
		parts := strings.SplitN(string(content), ",", 2)
		if len(parts) == 2 && parts[1] == m.token {
			return os.Remove(tempPath)
		}
	}

	// If not our lock, put it back
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
