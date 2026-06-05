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
	ErrCacheMiss = errors.New("cache miss")
)

type CacheManager interface {
	GetCache(ctx context.Context, key string) ([]byte, error)
	SetCache(ctx context.Context, key string, value []byte, ttl time.Duration) error
	DeleteCache(ctx context.Context, key string) error
}

func NewCacheManager(redisOpts *redis.Options, orgID string) CacheManager {
	if os.Getenv("OHC_MULTITENANT") == "true" {
		client := redis.NewClient(redisOpts)
		return &RedisCacheManager{
			client: client,
			orgID:  orgID,
		}
	}
	return NewInMemoryCacheManager()
}

type InMemoryCacheManager struct {
	mu    sync.RWMutex
	store map[string]cacheItem
}

type cacheItem struct {
	value     []byte
	expiresAt time.Time
}

func NewInMemoryCacheManager() *InMemoryCacheManager {
	return &InMemoryCacheManager{
		store: make(map[string]cacheItem),
	}
}

func (c *InMemoryCacheManager) GetCache(ctx context.Context, key string) ([]byte, error) {
	c.mu.RLock()
	defer c.mu.RUnlock()

	item, ok := c.store[key]
	if !ok {
		return nil, ErrCacheMiss
	}

	if !item.expiresAt.IsZero() && time.Now().After(item.expiresAt) {
		return nil, ErrCacheMiss
	}

	return item.value, nil
}

func (c *InMemoryCacheManager) SetCache(ctx context.Context, key string, value []byte, ttl time.Duration) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	var expiresAt time.Time
	if ttl > 0 {
		expiresAt = time.Now().Add(ttl)
	}

	c.store[key] = cacheItem{
		value:     value,
		expiresAt: expiresAt,
	}

	return nil
}

func (c *InMemoryCacheManager) DeleteCache(ctx context.Context, key string) error {
	c.mu.Lock()
	defer c.mu.Unlock()

	delete(c.store, key)
	return nil
}

type RedisCacheManager struct {
	client *redis.Client
	orgID  string
}

func (c *RedisCacheManager) formatKey(key string) string {
	if c.orgID != "" {
		return fmt.Sprintf("tenant_id:%s:%s", c.orgID, key)
	}
	return key
}

func (c *RedisCacheManager) GetCache(ctx context.Context, key string) ([]byte, error) {
	formattedKey := c.formatKey(key)
	val, err := c.client.Get(ctx, formattedKey).Bytes()
	if err == redis.Nil {
		return nil, ErrCacheMiss
	}
	return val, err
}

func (c *RedisCacheManager) SetCache(ctx context.Context, key string, value []byte, ttl time.Duration) error {
	formattedKey := c.formatKey(key)
	return c.client.Set(ctx, formattedKey, value, ttl).Err()
}

func (c *RedisCacheManager) DeleteCache(ctx context.Context, key string) error {
	formattedKey := c.formatKey(key)
	return c.client.Del(ctx, formattedKey).Err()
}
