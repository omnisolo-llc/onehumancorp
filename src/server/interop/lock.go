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

	// Step 1: Ensure directory exists.
	// os.MkdirAll is generally safe, but we only need os.Mkdir here.
	// Wait, we can't use os.Mkdir for mutual exclusion if we don't delete the dir.
	// Instead, let's use directory atomic creation with os.Mkdir for mutual exclusion.
	// We'll create the directory, if it fails because it exists, we check the info file.
	// If the info file is expired, we don't try to delete the directory (which is TOCTOU).
	// Instead, we just acquire by atomically renaming our temporary info file over the existing one.
	// Wait, rename is not atomic if we only want to overwrite *expired* ones.
	//
	// A better way: Let's use `os.OpenFile(..., O_CREATE|O_EXCL)` but securely! Wait, we know `O_EXCL` over NFS might be buggy but locally it's fine.
	// But the issue said "exclusively utilizing atomic file and directory creation operations (e.g., os.OpenFile with os.O_EXCL and os.Mkdir) instead of relying on syscall.Flock or os.Remove."
	// Let's use directories but with unique names, and a symlink? No, Windows doesn't support symlinks well.
	//
	// How to avoid TOCTOU with Mkdir:
	// To acquire, Mkdir(path/token).
	// To release, Remove(path/token).
	// But how to check expiry?
	// Lock dir: `ohc_lock_KEY`. Inside, we `Mkdir("holder")`. That still needs `Remove`.
	// The problem is: if it's expired, two processes see it's expired.
	// They both try to `os.Rename(path, tempPath)`.
	// One succeeds, one fails! The one that succeeds cleans up and retries.
	// The one that fails just retries or returns false.
	// Wait, `os.Rename` of a directory fails if the source doesn't exist.
	// This *is* atomic and prevents the steal!
	// Let's check why the reviewer said it was a race:
	// "Process A renames the directory out of the way... Process B then executes its pending os.Rename(path, tempPath)... Process B blindly moves whatever is currently at path, stealing Process A's valid lock!"
	// Ah! Process A renames expired dir to TempA. Process A creates new valid dir.
	// Then Process B (who checked and saw the old expired dir) calls `os.Rename(path, TempB)`.
	// Since Process A just created the new valid dir, Process B ends up renaming Process A's *valid* dir to TempB, thinking it was the expired one!
	// That is the TOCTOU.

	// How to fix:
	// The directory name itself should contain the token and expiry!
	// We can read the directory contents. Find a directory starting with `lock_`.
	// If it's valid, return false.
	// If it's expired, we rename that *exact* directory (e.g. `path/lock_TOKEN_EXPIRY`) to `path/expired_...`.
	// If the rename succeeds, we successfully evicted the expired lock.
	// Then we create our own `path/lock_MYTOKEN_MYEXPIRY`.
	// Since the directory name includes the token and expiry, Process B's `os.Rename(path/lock_OLDTOKEN_OLDEXPIRY, ...)` will FAIL because Process A renamed it and it's no longer there. Process B will NOT accidentally rename Process A's new lock, because Process A's lock has a different name!

	basePath := filepath.Join(os.TempDir(), "ohc_lock_dir_"+safeKey)
	_ = os.Mkdir(basePath, 0777) // Ensure base dir exists

	// Scan base directory for existing locks
	entries, err := os.ReadDir(basePath)
	if err != nil {
		return false, err
	}

	for _, entry := range entries {
		if entry.IsDir() && strings.HasPrefix(entry.Name(), "lock_") {
			parts := strings.SplitN(entry.Name(), "_", 3)
			if len(parts) == 3 {
				expiryTime, parseErr := time.Parse(time.RFC3339Nano, parts[2])
				if parseErr == nil {
					if time.Now().After(expiryTime) {
						// It's expired. Try to atomically remove it by renaming it to a temp name.
						oldPath := filepath.Join(basePath, entry.Name())
						tempPath := filepath.Join(basePath, "expired_"+uuid.New().String())
						renameErr := os.Rename(oldPath, tempPath)
						if renameErr == nil {
							os.RemoveAll(tempPath)
						}
						// If rename failed, someone else evicted it. That's fine, continue checking.
					} else {
						// Found a valid lock that hasn't expired.
						return false, nil
					}
				}
			}
		}
	}

	// No valid locks found (or all expired ones were evicted or are being evicted).
	// Create our lock directory.
	expiry := time.Now().Add(ttl).Format(time.RFC3339Nano)
	lockDirName := "lock_" + m.token + "_" + expiry
	lockPath := filepath.Join(basePath, lockDirName)

	err = os.Mkdir(lockPath, 0777)
	if err != nil {
		// If it fails, maybe someone else just created a lock.
		return false, nil
	}

	// We created the directory, but to be absolutely sure we won the race,
	// we check again to make sure no other lock was created at the exact same time
	// and sorted before us, or similar. Actually, Mkdir is not atomic relative to the base dir,
	// but multiple Mkdirs with DIFFERENT names can succeed.
	// If two processes create their dirs simultaneously, they both exist.
	// To resolve this split-brain, we must check if there is any OTHER lock dir
	// that was created. If so, we only keep ours if ours is the "winner" (e.g., lexicographically first token).

	entries, err = os.ReadDir(basePath)
	if err != nil {
		os.RemoveAll(lockPath)
		return false, err
	}

	var validLocks []string
	for _, entry := range entries {
		if entry.IsDir() && strings.HasPrefix(entry.Name(), "lock_") {
			parts := strings.SplitN(entry.Name(), "_", 3)
			if len(parts) == 3 {
				expiryTime, parseErr := time.Parse(time.RFC3339Nano, parts[2])
				if parseErr == nil && !time.Now().After(expiryTime) {
					validLocks = append(validLocks, entry.Name())
				}
			}
		}
	}

	if len(validLocks) > 1 {
		// There are multiple valid locks. The one with the lexicographically smallest token wins.
		winner := validLocks[0]
		for _, name := range validLocks {
			if name < winner {
				winner = name
			}
		}

		if winner != lockDirName {
			// We lost the race.
			os.RemoveAll(lockPath)
			return false, nil
		}
	}

	return true, nil
}

func (m *memoryLock) Unlock(ctx context.Context, key string) error {
	safeKey := strings.ReplaceAll(key, "/", "_")
	basePath := filepath.Join(os.TempDir(), "ohc_lock_dir_"+safeKey)

	entries, err := os.ReadDir(basePath)
	if err != nil {
		return nil
	}

	for _, entry := range entries {
		if entry.IsDir() && strings.HasPrefix(entry.Name(), "lock_"+m.token+"_") {
			oldPath := filepath.Join(basePath, entry.Name())
			tempPath := filepath.Join(basePath, "released_"+uuid.New().String())
			renameErr := os.Rename(oldPath, tempPath)
			if renameErr == nil {
				os.RemoveAll(tempPath)
			}
			return nil
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
