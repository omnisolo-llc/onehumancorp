package interop

import (
	"context"
	"crypto/sha256"
	"encoding/hex"
	"encoding/json"
	"log/slog"
	"os"
	"path/filepath"
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
			slog.Warn("failed to parse REDIS_URL, falling back to file lock", "error", err)
			return &fileLock{token: uuid.New().String()}, nil
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			slog.Warn("failed to connect to redis, falling back to file lock", "error", err)
			return &fileLock{token: uuid.New().String()}, nil
		}
		slog.Info("DistributedLock initialized in Cloud mode (Redis)")
		return &cloudLock{client: c, token: uuid.New().String()}, nil
	}

	slog.Info("DistributedLock initialized in Standalone mode (File-based)")
	return &fileLock{token: uuid.New().String()}, nil
}

type lockInfo struct {
	Expiry time.Time `json:"expiry"`
	Token  string    `json:"token"`
}

// fileLock provides a local file-based lock implementation.
type fileLock struct {
	token string
}

func (f *fileLock) Lock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	h := sha256.New()
	h.Write([]byte(key))
	safeKey := hex.EncodeToString(h.Sum(nil))

	lockDir := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)

	err := os.Mkdir(lockDir, 0700)
	if err != nil {
		if !os.IsExist(err) {
			return false, err
		}

		// Directory exists, check if expired. Find the info file.
		entries, readErr := os.ReadDir(lockDir)
		var infoFile string
		for _, entry := range entries {
			if !entry.IsDir() {
				infoFile = filepath.Join(lockDir, entry.Name())
				break
			}
		}

		if infoFile == "" || readErr != nil {
			// If we can't find an info file (e.g. it doesn't exist yet because the
			// lock creator is still writing it), assume locked,
			// unless the directory itself is too old (e.g., creator crashed).
			if stat, statErr := os.Stat(lockDir); statErr == nil {
				if time.Since(stat.ModTime()) > 5*time.Second {
					// Directory is older than 5 seconds but info file is missing. Abandoned/crashed.
					// Fall through to reclaim.
				} else {
					return false, nil
				}
			} else {
				return false, nil
			}
		} else {
			data, err := os.ReadFile(infoFile)
			if err != nil {
				return false, nil
			}
			var info lockInfo
			if unmarshalErr := json.Unmarshal(data, &info); unmarshalErr != nil {
				// Invalid info file, assume locked to be safe
				return false, nil
			}

			if time.Now().Before(info.Expiry) {
				return false, nil // Still locked
			}
		}

		// Expired or abandoned, try to reclaim
		reclaimDir := filepath.Join(os.TempDir(), "ohc_reclaim_"+uuid.New().String())
		renameErr := os.Rename(lockDir, reclaimDir)
		if renameErr != nil {
			// Someone else might have reclaimed it, or we don't have permission
			return false, nil
		}

		// We successfully reclaimed it, clean up the old one
		os.RemoveAll(reclaimDir)

		// Try to create again
		err = os.Mkdir(lockDir, 0700)
		if err != nil {
			if os.IsExist(err) {
				return false, nil // Someone beat us to it
			}
			return false, err
		}
	}

	// Successfully created lockDir, write info
	info := lockInfo{
		Expiry: time.Now().Add(ttl),
		Token:  f.token,
	}
	data, _ := json.Marshal(info)
	infoFile := filepath.Join(lockDir, "info_"+f.token+".json")
	if writeErr := os.WriteFile(infoFile, data, 0600); writeErr != nil {
		os.Remove(lockDir)
		return false, writeErr
	}
	return true, nil
}

func (f *fileLock) Unlock(ctx context.Context, key string) error {
	h := sha256.New()
	h.Write([]byte(key))
	safeKey := hex.EncodeToString(h.Sum(nil))

	lockDir := filepath.Join(os.TempDir(), "ohc_lock_"+safeKey)
	infoFile := filepath.Join(lockDir, "info_"+f.token+".json")

	// Atomically remove our specific info file. If it fails, we either didn't own the lock
	// or another process already reclaimed it (so it's gone).
	err := os.Remove(infoFile)
	if err == nil {
		// Only remove the directory if it's empty. If another process reclaimed it
		// and created a new info file, os.Remove will safely fail.
		_ = os.Remove(lockDir)
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
