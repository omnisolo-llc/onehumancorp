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

	hash := sha256.Sum256([]byte(key))
	hashedKey := hex.EncodeToString(hash[:])
	path := filepath.Join(os.TempDir(), "ohc_lock_"+hashedKey)

	err := os.Mkdir(path, 0700)
	if err != nil {
		if os.IsExist(err) {
			// Check if the lock has expired by reading the info file
			entries, readErr := os.ReadDir(path)
			if readErr == nil {
				for _, entry := range entries {
					if !entry.IsDir() && strings.HasPrefix(entry.Name(), "info_") && strings.HasSuffix(entry.Name(), ".json") {
						content, err := os.ReadFile(filepath.Join(path, entry.Name()))
						if err == nil {
							expiryTime, parseErr := time.Parse(time.RFC3339Nano, string(content))
							if parseErr == nil && time.Now().After(expiryTime) {
								// Expired lock found. To avoid TOCTOU, we attempt to rename the directory.
								tempPath := path + "_" + uuid.New().String() + ".tmp"
								renameErr := os.Rename(path, tempPath)
								if renameErr == nil {
									// We successfully "stole" the expired lock atomically.
									os.RemoveAll(tempPath)
									err = os.Mkdir(path, 0700)
									if err != nil {
										return false, nil
									}
									goto acquired
								}
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
		os.RemoveAll(path)
		return false, err
	}

	return true, nil
}

func (m *memoryLock) Unlock(ctx context.Context, key string) error {
	hash := sha256.Sum256([]byte(key))
	hashedKey := hex.EncodeToString(hash[:])
	path := filepath.Join(os.TempDir(), "ohc_lock_"+hashedKey)

	// TOCTOU mitigation: rename the directory to a temp directory, check if our info file exists, if valid, delete it. If invalid, rename it back.
	tempPath := path + "_" + uuid.New().String() + ".tmp"
	err := os.Rename(path, tempPath)
	if err != nil {
		return nil // Lock might already be gone
	}

	if _, err := os.Stat(filepath.Join(tempPath, "info_"+m.token+".json")); err == nil {
		return os.RemoveAll(tempPath)
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
