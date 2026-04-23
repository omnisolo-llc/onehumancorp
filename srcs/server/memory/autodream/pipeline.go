package autodream

import (
	"context"
	"time"
)

// MemoryConsolidator defines the pipeline for long-term memory consolidation.
type MemoryConsolidator interface {
	Consolidate(ctx context.Context, taskID string, logs []string) error
	ResolveConflicts(ctx context.Context, organizationID string, topic string) error
	PruneStaleContext(ctx context.Context, organizationID string, threshold time.Duration) (int64, error)
	GetSharedContext(ctx context.Context, query string) (string, error)
}
