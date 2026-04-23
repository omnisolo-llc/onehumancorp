package vector

import (
	"context"
)

// SearchResult represents a single retrieved vector document.
type SearchResult struct {
	ID       string                 `json:"id"`
	Vector   []float32              `json:"vector"`
	Metadata map[string]interface{} `json:"metadata"`
	Score    float32                `json:"score"`
}

// VectorStorageProvider defines the universal interface for vector operations.
type VectorStorageProvider interface {
	// Store stores an embedding vector.
	Store(ctx context.Context, namespace, id string, vector []float32, metadata map[string]interface{}) error

	// Search retrieves similar vectors based on a query vector.
	Search(ctx context.Context, namespace string, queryVector []float32, topK int) ([]SearchResult, error)
}
