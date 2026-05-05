package pricing

import (
	"crypto/sha256"
	"encoding/hex"
	"sync"
	"time"
)

type CacheEntry struct {
	Response  string
	CreatedAt time.Time
	ExpiresAt time.Time
}

type LocalEmbeddingCache struct {
	entries map[string]CacheEntry
	ttl     time.Duration
	mu      sync.RWMutex
}

func NewLocalEmbeddingCache(ttl time.Duration) *LocalEmbeddingCache {
	cache := &LocalEmbeddingCache{
		entries: make(map[string]CacheEntry),
		ttl:     ttl,
	}

	go func() {
		ticker := time.NewTicker(ttl)
		defer ticker.Stop()
		for range ticker.C {
			cache.Prune()
		}
	}()

	return cache
}

func (c *LocalEmbeddingCache) hashPrompt(prompt string) string {
	h := sha256.New()
	h.Write([]byte(prompt))
	return hex.EncodeToString(h.Sum(nil))
}

func (c *LocalEmbeddingCache) Get(prompt string) (string, bool) {
	c.mu.RLock()
	defer c.mu.RUnlock()

	key := c.hashPrompt(prompt)
	entry, exists := c.entries[key]
	if !exists {
		return "", false
	}

	if time.Now().After(entry.ExpiresAt) {
		return "", false
	}

	return entry.Response, true
}

func (c *LocalEmbeddingCache) Set(prompt, response string) {
	c.mu.Lock()
	defer c.mu.Unlock()

	key := c.hashPrompt(prompt)
	now := time.Now()
	c.entries[key] = CacheEntry{
		Response:  response,
		CreatedAt: now,
		ExpiresAt: now.Add(c.ttl),
	}
}

func (c *LocalEmbeddingCache) Prune() int {
	c.mu.Lock()
	defer c.mu.Unlock()

	now := time.Now()
	pruned := 0
	for k, v := range c.entries {
		if now.After(v.ExpiresAt) {
			delete(c.entries, k)
			pruned++
		}
	}
	return pruned
}
