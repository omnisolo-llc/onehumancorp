package pricing

import (
	"testing"
	"time"
)

func TestLocalEmbeddingCache(t *testing.T) {
	cache := NewLocalEmbeddingCache(100 * time.Millisecond)

	cache.Set("prompt1", "response1")

	if val, ok := cache.Get("prompt1"); !ok || val != "response1" {
		t.Errorf("expected response1, got %v", val)
	}

	if _, ok := cache.Get("prompt2"); ok {
		t.Errorf("expected false, got true")
	}

	time.Sleep(150 * time.Millisecond)

	if _, ok := cache.Get("prompt1"); ok {
		t.Errorf("expected false after expiration, got true")
	}

	cache.Set("prompt3", "response3")

	if pruned := cache.Prune(); pruned != 1 {
		t.Errorf("expected to prune 1 entry, pruned %d", pruned)
	}
}
