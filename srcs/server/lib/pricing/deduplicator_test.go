package pricing

import (
	"testing"
)

func TestSemanticDeduplicator(t *testing.T) {
	deduper := NewSemanticDeduplicator(0.8)

	prompt1 := "What is the capital of France?"
	prompt2 := "What is the capital city of France?"
	prompt3 := "Tell me about quantum computing."

	if deduper.IsDuplicate(prompt1) {
		t.Fatalf("prompt1 should not be a duplicate")
	}

	// Jaccard similarity is 0.75, so setting threshold to 0.7 will make it match
	deduper70 := NewSemanticDeduplicator(0.7)
	deduper70.IsDuplicate(prompt1)
	if !deduper70.IsDuplicate(prompt2) {
		t.Fatalf("prompt2 should be a duplicate of prompt1 with threshold 0.7")
	}

	if deduper.IsDuplicate(prompt3) {
		t.Fatalf("prompt3 should not be a duplicate")
	}
}

func TestSemanticDeduplicatorBounds(t *testing.T) {
	deduper := NewBoundedSemanticDeduplicator(0.8, 2)

	deduper.IsDuplicate("prompt A")
	deduper.IsDuplicate("prompt B")
	deduper.IsDuplicate("prompt C")

	if len(deduper.history) != 2 {
		t.Fatalf("Expected history length 2, got %d", len(deduper.history))
	}
}
