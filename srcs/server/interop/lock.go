package interop

import (
	"context"
	"io"
	"log/slog"
	"os"
	"path/filepath"
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
			slog.Warn("failed to parse REDIS_URL, falling back to file lock", "error", err)
			return newFileLock(), nil
		}
		c, err := rueidis.NewClient(opts)
		if err != nil {
			slog.Warn("failed to connect to redis, falling back to file lock", "error", err)
			return newFileLock(), nil
		}
		slog.Info("DistributedLock initialized in Cloud mode (Redis)")
		return &cloudLock{client: c, token: uuid.New().String()}, nil
	}

	slog.Info("DistributedLock initialized in Standalone mode (File)")
	return newFileLock(), nil
}

type fileLock struct {
	baseDir string
	token   string
	mu      sync.Mutex
	locks   map[string]bool
}

func newFileLock() *fileLock {
	homeDir := os.TempDir()
	baseDir := filepath.Join(homeDir, ".ohc-local-data", "locks")
	if err := os.MkdirAll(baseDir, 0755); err != nil {
		slog.Warn("Failed to create lock dir, using temp", "error", err)
		baseDir = filepath.Join(os.TempDir(), "ohc_locks")
		os.MkdirAll(baseDir, 0755)
	}

	return &fileLock{
		baseDir: baseDir,
		token:   uuid.New().String(),
		locks:   make(map[string]bool),
	}
}

func (f *fileLock) Lock(ctx context.Context, key string, ttl time.Duration) (bool, error) {
	f.mu.Lock()
	defer f.mu.Unlock()

	// Re-entrant lock check for same process
	if f.locks[key] {
		return false, nil
	}

	lockFile := filepath.Join(f.baseDir, key+".lock")

	// Attempt to acquire lock atomically
	file, err := os.OpenFile(lockFile, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0666)
	if err != nil {
		if os.IsExist(err) {
			// Lock file exists. Check if it's expired.
			stat, errStat := os.Stat(lockFile)
			if errStat == nil && time.Since(stat.ModTime()) > ttl {
				// File is expired. Attempt to forcefully remove it and re-acquire.
				// This has a slight TOCTOU risk if two processes see it expired at the same time,
				// but because we use O_EXCL next, only one will succeed in creating it.
				os.Remove(lockFile)
				file, err = os.OpenFile(lockFile, os.O_WRONLY|os.O_CREATE|os.O_EXCL, 0666)
				if err != nil {
					return false, nil // Another process grabbed it first
				}
			} else {
				return false, nil // Locked by someone else and not expired
			}
		} else {
			return false, err // Other I/O error
		}
	}
	defer file.Close()

	// We got the lock! Write token for identification if needed.
	_, _ = io.WriteString(file, f.token)
	f.locks[key] = true
	return true, nil
}

func (f *fileLock) Unlock(ctx context.Context, key string) error {
	f.mu.Lock()
	defer f.mu.Unlock()

	if !f.locks[key] {
		return nil
	}

	lockFile := filepath.Join(f.baseDir, key+".lock")

	// Ensure we only delete it if we are the ones who created it, verifying token
	file, err := os.Open(lockFile)
	if err == nil {
		content, _ := io.ReadAll(file)
		file.Close()
		if string(content) == f.token {
			os.Remove(lockFile)
		}
	}

	delete(f.locks, key)
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
