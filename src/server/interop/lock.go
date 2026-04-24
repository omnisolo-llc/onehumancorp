package interop

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
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

	h := sha256.New()
	h.Write([]byte(key))
	safeKey := hex.EncodeToString(h.Sum(nil))
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	err := os.Mkdir(path, 0700)
	if err != nil {
		if os.IsExist(err) {
			// Lock directory exists, check if any token file inside is expired
			entries, err := os.ReadDir(path)
			if err == nil {
				for _, entry := range entries {
					if !entry.IsDir() && strings.HasPrefix(entry.Name(), "info_") && strings.HasSuffix(entry.Name(), ".json") {
						content, err := os.ReadFile(filepath.Join(path, entry.Name()))
						if err == nil {
							expiryTime, parseErr := time.Parse(time.RFC3339Nano, string(content))
							if parseErr == nil && time.Now().After(expiryTime) {
								// Expired lock found. Delete the token file.
								os.Remove(filepath.Join(path, entry.Name()))
								// Delete the directory (will fail if another token file was written, avoiding TOCTOU)
								os.Remove(path)

								// Try to acquire the lock again
								err = os.Mkdir(path, 0700)
								if err == nil {
									goto acquired
								}
								return false, nil
							}
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
	err = os.WriteFile(filepath.Join(path, "info_"+m.token+".json"), []byte(expiry), 0600)
	if err != nil {
		os.Remove(filepath.Join(path, "info_"+m.token+".json"))
		os.Remove(path)
		return false, err
	}

	return true, nil
}

func (m *memoryLock) Unlock(ctx context.Context, key string) error {
	h := sha256.New()
	h.Write([]byte(key))
	safeKey := hex.EncodeToString(h.Sum(nil))
	path := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	tokenPath := filepath.Join(path, "info_"+m.token+".json")

	// Delete our token file
	err := os.Remove(tokenPath)
	if err != nil {
		return nil // Lock might already be gone
	}

	// Safely attempt to delete the directory (will only succeed if empty)
	os.Remove(path)

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
