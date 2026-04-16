package pricing

import (
	"strings"
	"sync"
)

type historyItem struct {
	reduced string
	wordSet map[string]bool
}

// SemanticDeduplicator reduces redundant LLM queries by matching semantically similar prompts.
// It uses a bounded history to prevent memory leaks and maintain performance.
type SemanticDeduplicator struct {
	history      []historyItem
	similarityTh float64
	maxSize      int
	mu           sync.RWMutex
}

// NewSemanticDeduplicator creates a new deduplicator with a given similarity threshold (0.0 to 1.0) and maximum history size.
func NewSemanticDeduplicator(threshold float64) *SemanticDeduplicator {
	return &SemanticDeduplicator{
		history:      make([]historyItem, 0),
		similarityTh: threshold,
		maxSize:      1000, // Default bounded size
	}
}

// NewBoundedSemanticDeduplicator creates a new deduplicator with custom size limit.
func NewBoundedSemanticDeduplicator(threshold float64, maxSize int) *SemanticDeduplicator {
	return &SemanticDeduplicator{
		history:      make([]historyItem, 0),
		similarityTh: threshold,
		maxSize:      maxSize,
	}
}

func jaccardSimilarity(set1, set2 map[string]bool) float64 {
	intersection := 0
	for w := range set1 {
		if set2[w] {
			intersection++
		}
	}

	union := len(set1) + len(set2) - intersection
	if union == 0 {
		return 1.0 // Both empty
	}

	return float64(intersection) / float64(union)
}

func getWordSet(s string) map[string]bool {
	words := strings.Fields(s)
	set := make(map[string]bool, len(words))
	for _, w := range words {
		set[w] = true
	}
	return set
}

// IsDuplicate checks if a similar prompt has been seen before.
func (d *SemanticDeduplicator) IsDuplicate(prompt string) bool {
	reduced := ReduceTokens(prompt)
	wordSet := getWordSet(reduced)

	d.mu.RLock()
	for _, h := range d.history {
		sim := jaccardSimilarity(wordSet, h.wordSet)
		if sim >= d.similarityTh {
			d.mu.RUnlock()
			return true
		}
	}
	d.mu.RUnlock()

	d.mu.Lock()
	if len(d.history) >= d.maxSize && d.maxSize > 0 {
		// Remove oldest item (simple queue behavior)
		// We could also use list.List, but slices are fine for small/moderate max sizes
		d.history = d.history[1:]
	}
	d.history = append(d.history, historyItem{reduced: reduced, wordSet: wordSet})
	d.mu.Unlock()

	return false
}
