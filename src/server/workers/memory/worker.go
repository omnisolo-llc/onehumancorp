package memory

import (
	"context"
	"fmt"
	"log/slog"
	"time"

	"github.com/onehumancorp/mono/src/server/db"
	"github.com/onehumancorp/mono/src/server/memory/autodream"
)

// Worker runs background tasks for memory consolidation, conflict resolution, and pruning.
type Worker struct {
	provider db.Provider
	service  *autodream.Service
}

func NewWorker(provider db.Provider, service *autodream.Service) *Worker {
	return &Worker{
		provider: provider,
		service:  service,
	}
}

// ProcessOrganization background job for a specific tenant
func (w *Worker) ProcessOrganization(ctx context.Context, organizationID string) error {
	slog.Info("MemoryWorker: Starting memory maintenance for organization", "organizationID", organizationID)

	// Resolve conflicts
	if err := w.service.ResolveConflicts(ctx, organizationID); err != nil {
		slog.Error("MemoryWorker: Failed to resolve conflicts", "organizationID", organizationID, "error", err)
		return fmt.Errorf("failed to resolve conflicts: %w", err)
	}

	// Prune stale context (older than 180 days for instance, conservative pruning)
	if err := w.service.PruneStaleContext(ctx, organizationID, 180*24*time.Hour); err != nil {
		slog.Error("MemoryWorker: Failed to prune stale context", "organizationID", organizationID, "error", err)
		return fmt.Errorf("failed to prune stale context: %w", err)
	}

	slog.Info("MemoryWorker: Finished memory maintenance for organization", "organizationID", organizationID)
	return nil
}
