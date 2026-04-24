package autodream

import (
	"context"
)

type LLMClient interface {
	Reason(ctx context.Context, prompt string) (string, error)
	GenerateEmbedding(ctx context.Context, text string) ([]float32, error)
}

// MemoryConsolidator defines the pipeline for long-term memory consolidation.
type MemoryConsolidator interface {
	Consolidate(ctx context.Context, taskID string, logs []string) error
}
