package llm

import (
	"context"
)

// EmbeddingClient defines the interface for generating vector embeddings.
type EmbeddingClient interface {
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// DefaultEmbeddingClient is a basic implementation of EmbeddingClient
// that could later integrate with Minimax, OpenAI, or a local provider.
type DefaultEmbeddingClient struct{}

// NewDefaultEmbeddingClient creates a new DefaultEmbeddingClient.
func NewDefaultEmbeddingClient() *DefaultEmbeddingClient {
	return &DefaultEmbeddingClient{}
}

// GenerateEmbedding generates a dummy 1536-dimensional embedding.
func (c *DefaultEmbeddingClient) GenerateEmbedding(ctx context.Context, text string) ([]float32, error) {
	// For now, return a dummy embedding.
	emb := make([]float32, 1536)
	if len(text) > 0 {
		emb[0] = float32(len(text)) / 1000.0 // Just a deterministic dummy value
	}
	return emb, nil
}
