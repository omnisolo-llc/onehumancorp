package pricing

import (
	"strings"
	"sync"
)

// SemanticDeduplicator reduces redundant LLM queries by matching semantically similar prompts.
// It uses a bounded history to prevent memory leaks and maintain performance.
type SemanticDeduplicator struct {
	history      []string
	similarityTh float64
	maxSize      int
	mu           sync.RWMutex
}

// NewSemanticDeduplicator creates a new deduplicator with a given similarity threshold (0.0 to 1.0) and maximum history size.
func NewSemanticDeduplicator(threshold float64) *SemanticDeduplicator {
	return &SemanticDeduplicator{
		history:      make([]string, 0),
		similarityTh: threshold,
		maxSize:      1000, // Default bounded size
	}
}

// NewBoundedSemanticDeduplicator creates a new deduplicator with custom size limit.
func NewBoundedSemanticDeduplicator(threshold float64, maxSize int) *SemanticDeduplicator {
	return &SemanticDeduplicator{
		history:      make([]string, 0),
		similarityTh: threshold,
		maxSize:      maxSize,
	}
}

func jaccardSimilarity(s1, s2 string) float64 {
	w1 := strings.Fields(s1)
	w2 := strings.Fields(s2)

	set1 := make(map[string]bool)
	for _, w := range w1 {
		set1[w] = true
	}

	set2 := make(map[string]bool)
	for _, w := range w2 {
		set2[w] = true
	}

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

// IsDuplicate checks if a similar prompt has been seen before.
func (d *SemanticDeduplicator) IsDuplicate(prompt string) bool {
	reduced := ReduceTokens(prompt)

	d.mu.RLock()
	for _, h := range d.history {
		sim := jaccardSimilarity(reduced, h)
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
	d.history = append(d.history, reduced)
	d.mu.Unlock()

	return false
}
