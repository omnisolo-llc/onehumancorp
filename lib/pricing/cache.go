package pricing

import (
	"crypto/sha256"
	"encoding/hex"
	"sync"
	"time"
)

// CacheEntry stores a cached LLM response and its metadata.
type CacheEntry struct {
	Response  string
	CreatedAt time.Time
	ExpiresAt time.Time
}

// LocalEmbeddingCache implements a cost-saving local cache for LLM queries/embeddings.
// This reduces the need for expensive API calls when similar prompts are encountered.
type LocalEmbeddingCache struct {
	entries map[string]CacheEntry
	mu      sync.RWMutex
	ttl     time.Duration
}

// NewLocalEmbeddingCache initializes a new local cache with a given TTL.
func NewLocalEmbeddingCache(ttl time.Duration) *LocalEmbeddingCache {
	return &LocalEmbeddingCache{
		entries: make(map[string]CacheEntry),
		ttl:     ttl,
	}
}

// hashPrompt generates a stable hash for a prompt string.
func (c *LocalEmbeddingCache) hashPrompt(prompt string) string {
	h := sha256.New()
	h.Write([]byte(prompt))
	return hex.EncodeToString(h.Sum(nil))
}

// Get retrieves a response from the cache if it exists and hasn't expired.
func (c *LocalEmbeddingCache) Get(prompt string) (string, bool) {
	key := c.hashPrompt(prompt)

	c.mu.RLock()
	entry, exists := c.entries[key]
	c.mu.RUnlock()

	if !exists {
		return "", false
	}

	if time.Now().After(entry.ExpiresAt) {
		// Clean up expired entry, but we let Prune() handle deletion to avoid lock complexity
		return "", false
	}

	return entry.Response, true
}

// Set stores a prompt-response pair in the cache.
func (c *LocalEmbeddingCache) Set(prompt, response string) {
	key := c.hashPrompt(prompt)
	now := time.Now()

	c.mu.Lock()
	defer c.mu.Unlock()

	c.entries[key] = CacheEntry{
		Response:  response,
		CreatedAt: now,
		ExpiresAt: now.Add(c.ttl),
	}
}

// Clear removes all entries from the cache.
func (c *LocalEmbeddingCache) Clear() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.entries = make(map[string]CacheEntry)
}

// Prune removes all expired entries from the cache to free up memory.
func (c *LocalEmbeddingCache) Prune() int {
	c.mu.Lock()
	defer c.mu.Unlock()

	now := time.Now()
	prunedCount := 0

	for key, entry := range c.entries {
		if now.After(entry.ExpiresAt) {
			delete(c.entries, key)
			prunedCount++
		}
	}

	return prunedCount
}
