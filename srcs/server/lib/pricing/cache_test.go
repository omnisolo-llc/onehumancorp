package pricing

import (
	"testing"
	"time"
)

func TestLocalEmbeddingCache(t *testing.T) {
	// Use a large TTL to prevent the background ticker from pruning before our explicit call
	cache := NewLocalEmbeddingCache(1 * time.Hour)

	cache.Set("prompt1", "response1")

	if val, ok := cache.Get("prompt1"); !ok || val != "response1" {
		t.Errorf("expected response1, got %v", val)
	}

	if _, ok := cache.Get("prompt2"); ok {
		t.Errorf("expected false, got true")
	}

	// Manually backdate the entry to simulate expiration
	cache.mu.Lock()
	entry := cache.entries[cache.hashPrompt("prompt1")]
	entry.ExpiresAt = time.Now().Add(-1 * time.Second)
	cache.entries[cache.hashPrompt("prompt1")] = entry
	cache.mu.Unlock()

	if _, ok := cache.Get("prompt1"); ok {
		t.Errorf("expected false after expiration, got true")
	}

	cache.Set("prompt3", "response3")

	if pruned := cache.Prune(); pruned != 1 {
		t.Errorf("expected to prune 1 entry, pruned %d", pruned)
	}
}
