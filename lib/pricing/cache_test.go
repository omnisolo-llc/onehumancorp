package pricing

import (
	"testing"
	"time"
)

func TestLocalEmbeddingCache_GetSet(t *testing.T) {
	cache := NewLocalEmbeddingCache(5 * time.Minute)

	prompt := "What is the capital of France?"
	response := "Paris"

	// Should not exist initially
	_, exists := cache.Get(prompt)
	if exists {
		t.Fatalf("Expected prompt to not be in cache")
	}

	// Set and retrieve
	cache.Set(prompt, response)

	cachedResponse, exists := cache.Get(prompt)
	if !exists {
		t.Fatalf("Expected prompt to be in cache")
	}

	if cachedResponse != response {
		t.Fatalf("Expected cached response %q, got %q", response, cachedResponse)
	}
}

func TestLocalEmbeddingCache_Expiration(t *testing.T) {
	// Very short TTL for testing
	cache := NewLocalEmbeddingCache(10 * time.Millisecond)

	prompt := "What is the meaning of life?"
	response := "42"

	cache.Set(prompt, response)

	// Should exist immediately
	_, exists := cache.Get(prompt)
	if !exists {
		t.Fatalf("Expected prompt to be in cache immediately after set")
	}

	// Wait for expiration
	time.Sleep(20 * time.Millisecond)

	// Should not exist after expiration
	_, exists = cache.Get(prompt)
	if exists {
		t.Fatalf("Expected prompt to be expired from cache")
	}
}

func TestLocalEmbeddingCache_Prune(t *testing.T) {
	cache := NewLocalEmbeddingCache(10 * time.Millisecond)

	cache.Set("prompt1", "response1")
	cache.Set("prompt2", "response2")

	// Wait for expiration
	time.Sleep(20 * time.Millisecond)

	// Add a non-expired entry
	cache.Set("prompt3", "response3")

	pruned := cache.Prune()
	if pruned != 2 {
		t.Fatalf("Expected 2 entries to be pruned, got %d", pruned)
	}

	// Prompt 3 should still be there
	_, exists := cache.Get("prompt3")
	if !exists {
		t.Fatalf("Expected prompt3 to still be in cache")
	}
}
