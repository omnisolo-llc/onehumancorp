package pricing

import (
	"container/list"

	"crypto/sha256"
	"encoding/hex"
	"log"
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

// CompressedEmbeddingCache implements a cost-saving local cache that compresses responses
// to reduce memory bloat when caching large strings.
type CompressedEmbeddingCache struct {
	entries map[string]CacheEntry
	mu      sync.RWMutex
	ttl     time.Duration
}

// NewCompressedEmbeddingCache initializes a new compressed local cache with a given TTL.
func NewCompressedEmbeddingCache(ttl time.Duration) *CompressedEmbeddingCache {
	return &CompressedEmbeddingCache{
		entries: make(map[string]CacheEntry),
		ttl:     ttl,
	}
}

// hashPrompt generates a stable hash for a prompt string.
func (c *CompressedEmbeddingCache) hashPrompt(prompt string) string {
	h := sha256.New()
	h.Write([]byte(prompt))
	return hex.EncodeToString(h.Sum(nil))
}

// Get retrieves a response from the cache if it exists and hasn't expired.
func (c *CompressedEmbeddingCache) Get(prompt string) (string, bool) {
	key := c.hashPrompt(prompt)

	c.mu.RLock()
	entry, exists := c.entries[key]
	c.mu.RUnlock()

	if !exists {
		return "", false
	}

	if time.Now().After(entry.ExpiresAt) {
		// Clean up expired entry, let Prune handle deletion
		return "", false
	}

	decompressed, err := DecompressLossless(entry.Response)
	if err != nil {
		log.Printf("Failed to decompress cached response: %v", err)
		return "", false
	}

	return decompressed, true
}

// Set stores a prompt-response pair in the cache, compressing it first.
func (c *CompressedEmbeddingCache) Set(prompt, response string) {
	key := c.hashPrompt(prompt)
	now := time.Now()

	compressed, err := CompressLossless(response)
	if err != nil {
		log.Printf("Failed to compress cache response: %v", err)
		return
	}

	c.mu.Lock()
	defer c.mu.Unlock()

	c.entries[key] = CacheEntry{
		Response:  compressed,
		CreatedAt: now,
		ExpiresAt: now.Add(c.ttl),
	}
}

// Clear removes all entries from the cache.
func (c *CompressedEmbeddingCache) Clear() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.entries = make(map[string]CacheEntry)
}

// Prune removes all expired entries from the cache to free up memory.
func (c *CompressedEmbeddingCache) Prune() int {
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

// BoundedEmbeddingCache implements an LRU bounded cache
type BoundedEmbeddingCache struct {
	entries   map[string]*list.Element
	evictList *list.List
	mu        sync.RWMutex
	maxItems  int
	ttl       time.Duration
}

type cacheItem struct {
	key   string
	entry CacheEntry
}

// NewBoundedEmbeddingCache initializes a new bounded local cache with a given TTL and max capacity.
func NewBoundedEmbeddingCache(ttl time.Duration, maxItems int) *BoundedEmbeddingCache {
	return &BoundedEmbeddingCache{
		entries:   make(map[string]*list.Element),
		evictList: list.New(),
		maxItems:  maxItems,
		ttl:       ttl,
	}
}

// hashPrompt generates a stable hash for a prompt string.
func (c *BoundedEmbeddingCache) hashPrompt(prompt string) string {
	h := sha256.New()
	h.Write([]byte(prompt))
	return hex.EncodeToString(h.Sum(nil))
}

// Get retrieves a response from the cache if it exists and hasn't expired.
func (c *BoundedEmbeddingCache) Get(prompt string) (string, bool) {
	key := c.hashPrompt(prompt)

	c.mu.Lock()
	defer c.mu.Unlock()

	if ele, hit := c.entries[key]; hit {
		item := ele.Value.(*cacheItem)
		if time.Now().After(item.entry.ExpiresAt) {
			// Expired, let Prune or eviction handle it or we could remove it here
			return "", false
		}
		c.evictList.MoveToFront(ele)
		return item.entry.Response, true
	}
	return "", false
}

// Set stores a prompt-response pair in the cache.
func (c *BoundedEmbeddingCache) Set(prompt, response string) {
	if c.maxItems <= 0 {
		return
	}

	key := c.hashPrompt(prompt)
	now := time.Now()

	c.mu.Lock()
	defer c.mu.Unlock()

	// Check for existing item
	if ele, hit := c.entries[key]; hit {
		c.evictList.MoveToFront(ele)
		ele.Value.(*cacheItem).entry.Response = response
		ele.Value.(*cacheItem).entry.CreatedAt = now
		ele.Value.(*cacheItem).entry.ExpiresAt = now.Add(c.ttl)
		return
	}

	// Add new item
	entry := CacheEntry{
		Response:  response,
		CreatedAt: now,
		ExpiresAt: now.Add(c.ttl),
	}
	item := &cacheItem{key: key, entry: entry}
	ele := c.evictList.PushFront(item)
	c.entries[key] = ele

	// Evict if over capacity
	if c.evictList.Len() > c.maxItems {
		c.removeOldest()
	}
}

func (c *BoundedEmbeddingCache) removeOldest() {
	ele := c.evictList.Back()
	if ele != nil {
		c.removeElement(ele)
	}
}

func (c *BoundedEmbeddingCache) removeElement(e *list.Element) {
	c.evictList.Remove(e)
	item := e.Value.(*cacheItem)
	delete(c.entries, item.key)
}

// Clear removes all entries from the cache.
func (c *BoundedEmbeddingCache) Clear() {
	c.mu.Lock()
	defer c.mu.Unlock()
	c.evictList.Init()
	c.entries = make(map[string]*list.Element)
}

// Prune removes all expired entries from the cache to free up memory.
func (c *BoundedEmbeddingCache) Prune() int {
	c.mu.Lock()
	defer c.mu.Unlock()

	now := time.Now()
	prunedCount := 0

	for ele := c.evictList.Front(); ele != nil; {
		next := ele.Next()
		item := ele.Value.(*cacheItem)
		if now.After(item.entry.ExpiresAt) {
			c.removeElement(ele)
			prunedCount++
		}
		ele = next
	}

	return prunedCount
}
