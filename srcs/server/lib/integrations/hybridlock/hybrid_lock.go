package hybridlock

import (
    "context"
    "sync"
    "time"

    "github.com/redis/go-redis/v9"
)

// Lock represents an acquired lock
type Lock struct {
    key   string
    token string
}

type localLockEntry struct {
    token    string
    expireAt time.Time
}

// HybridLockManager manages distributed locks across Cloud (Redis) and Standalone (In-Memory) modes.
type HybridLockManager struct {
    redisClient *redis.Client
    mu          sync.Mutex
    localLocks  map[string]localLockEntry
}

// NewHybridLockManager creates a new HybridLockManager. If redisClient is nil, it runs in Standalone mode.
func NewHybridLockManager(redisClient *redis.Client) *HybridLockManager {
    return &HybridLockManager{
        redisClient: redisClient,
        localLocks:  make(map[string]localLockEntry),
    }
}

// Acquire attempts to acquire a lock for a given key with a TTL.
func (m *HybridLockManager) Acquire(ctx context.Context, key, token string, ttl time.Duration) (*Lock, error) {
    if m.redisClient != nil {
        success, err := m.redisClient.SetNX(ctx, "lock:"+key, token, ttl).Result()
        if err != nil {
            return nil, err
        }
        if !success {
            return nil, nil // Lock not acquired
        }
        return &Lock{key: key, token: token}, nil
    }

    // Standalone mode: In-memory lock
    m.mu.Lock()
    defer m.mu.Unlock()

    now := time.Now()
    entry, exists := m.localLocks[key]

    // If the lock exists and has not expired, we cannot acquire it
    if exists && now.Before(entry.expireAt) {
        return nil, nil
    }

    // Otherwise, it doesn't exist or has expired, so we can acquire it
    m.localLocks[key] = localLockEntry{
        token:    token,
        expireAt: now.Add(ttl),
    }
    return &Lock{key: key, token: token}, nil
}

// Release attempts to release a lock.
func (m *HybridLockManager) Release(ctx context.Context, lock *Lock) error {
    if m.redisClient != nil {
        // Use Lua script to release only if the token matches
        script := `
            if redis.call("get", KEYS[1]) == ARGV[1] then
                return redis.call("del", KEYS[1])
            else
                return 0
            end
        `
        _, err := m.redisClient.Eval(ctx, script, []string{"lock:" + lock.key}, lock.token).Result()
        return err
    }

    // Standalone mode: In-memory release
    m.mu.Lock()
    defer m.mu.Unlock()

    entry, exists := m.localLocks[lock.key]
    if exists && entry.token == lock.token {
        delete(m.localLocks, lock.key)
    }
    return nil
}
