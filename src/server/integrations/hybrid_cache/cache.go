package hybrid_cache

import (
	"context"
	"errors"
	"fmt"
	"os"
	"sync"
	"time"

	"github.com/redis/go-redis/v9"
)

var (
	ErrKeyNotFound = errors.New("key not found")
)

type CacheManager interface {
	GetCache(ctx context.Context, key string) ([]byte, error)
	SetCache(ctx context.Context, key string, value []byte, ttl time.Duration) error
	DeleteCache(ctx context.Context, key string) error
}

type LocalCacheManager struct {
	mu    sync.RWMutex
	store map[string]localCacheEntry
}

type localCacheEntry struct {
	value     []byte
	expiresAt time.Time
}

func NewLocalCacheManager() *LocalCacheManager {
	return &LocalCacheManager{
		store: make(map[string]localCacheEntry),
	}
}

func (l *LocalCacheManager) GetCache(ctx context.Context, key string) ([]byte, error) {
	l.mu.RLock()
	defer l.mu.RUnlock()

	entry, ok := l.store[key]
	if !ok {
		return nil, ErrKeyNotFound
	}
	if !entry.expiresAt.IsZero() && time.Now().After(entry.expiresAt) {
		return nil, ErrKeyNotFound
	}
	return entry.value, nil
}

func (l *LocalCacheManager) SetCache(ctx context.Context, key string, value []byte, ttl time.Duration) error {
	l.mu.Lock()
	defer l.mu.Unlock()

	var expiresAt time.Time
	if ttl > 0 {
		expiresAt = time.Now().Add(ttl)
	}

	l.store[key] = localCacheEntry{
		value:     value,
		expiresAt: expiresAt,
	}
	return nil
}

func (l *LocalCacheManager) DeleteCache(ctx context.Context, key string) error {
	l.mu.Lock()
	defer l.mu.Unlock()
	delete(l.store, key)
	return nil
}

type RedisCacheManager struct {
	client *redis.Client
	orgID  string
}

func NewRedisCacheManager(client *redis.Client, orgID string) *RedisCacheManager {
	return &RedisCacheManager{
		client: client,
		orgID:  orgID,
	}
}

func (r *RedisCacheManager) formatKey(key string) string {
	return fmt.Sprintf("%s:%s", r.orgID, key)
}

func (r *RedisCacheManager) GetCache(ctx context.Context, key string) ([]byte, error) {
	val, err := r.client.Get(ctx, r.formatKey(key)).Bytes()
	if err != nil {
		if errors.Is(err, redis.Nil) {
			return nil, ErrKeyNotFound
		}
		return nil, err
	}
	return val, nil
}

func (r *RedisCacheManager) SetCache(ctx context.Context, key string, value []byte, ttl time.Duration) error {
	return r.client.Set(ctx, r.formatKey(key), value, ttl).Err()
}

func (r *RedisCacheManager) DeleteCache(ctx context.Context, key string) error {
	return r.client.Del(ctx, r.formatKey(key)).Err()
}

// NewCacheManager creates the appropriate CacheManager based on environment.
// For Cloud mode, it requires a non-nil redis.Client and orgID.
func NewCacheManager(redisClient *redis.Client, orgID string) CacheManager {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		return NewRedisCacheManager(redisClient, orgID)
	}
	return NewLocalCacheManager()
}
