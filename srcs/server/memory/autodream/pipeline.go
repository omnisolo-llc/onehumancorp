package autodream

import (
	"context"
)

// MemoryConsolidator defines the pipeline for long-term memory consolidation.
type MemoryConsolidator interface {
	Consolidate(ctx context.Context, taskID string, logs []string) error
}
