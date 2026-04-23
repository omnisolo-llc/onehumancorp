package llm

import (
	"context"
)

// Embedder defines the interface for generating vector embeddings
type Embedder interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// DefaultEmbedder acts as a standard implementation.
type DefaultEmbedder struct{}

func NewDefaultEmbedder() *DefaultEmbedder {
	return &DefaultEmbedder{}
}

func (e *DefaultEmbedder) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	// Simple stub for tests. In reality, it would call an external API.
	return make([]float32, 1536), nil
}
