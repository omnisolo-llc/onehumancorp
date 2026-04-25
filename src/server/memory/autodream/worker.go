package autodream

import (
	"context"
	"log"
	"time"

	"github.com/onehumancorp/mono/src/server/auth"
	"github.com/onehumancorp/mono/src/server/lib/resilience/lock"
	"github.com/onehumancorp/mono/src/server/memory"
)

// BackgroundWorker runs the consolidation logic in the background.
type BackgroundWorker struct {
	service    *Service
	vectorRepo *memory.VectorRepository
	interval   time.Duration
	staleAfter time.Duration
	lockProv   *lock.DatabaseLockProvider
}

// NewBackgroundWorker creates a new BackgroundWorker.
func NewBackgroundWorker(service *Service, vectorRepo *memory.VectorRepository, interval time.Duration, staleAfter time.Duration, lockProv *lock.DatabaseLockProvider) *BackgroundWorker {
	return &BackgroundWorker{
		service:    service,
		vectorRepo: vectorRepo,
		interval:   interval,
		staleAfter: staleAfter,
		lockProv:   lockProv,
	}
}

// Start begins the background worker loop. It should be run in a goroutine.
func (w *BackgroundWorker) Start(ctx context.Context) {
	ticker := time.NewTicker(w.interval)
	defer ticker.Stop()

	for {
		select {
		case <-ctx.Done():
			return
		case <-ticker.C:
			w.runCycle(ctx)
		}
	}
}

func (w *BackgroundWorker) runCycle(ctx context.Context) {
	// Attempt to acquire global lock for AutoDream
	locked, unlock, err := w.lockProv.TryLock(ctx, "autodream_background_worker", 10*time.Minute)
	if err != nil {
		log.Printf("Failed to attempt lock for AutoDream: %v", err)
		return
	}
	if !locked {
		// Another worker is running this cycle, so we skip
		return
	}
	defer func() {
		if err := unlock(ctx); err != nil {
			log.Printf("Failed to release lock for AutoDream: %v", err)
		}
	}()

	orgIDs, err := w.vectorRepo.GetOrganizationIDs(ctx)
	if err != nil {
		log.Printf("Failed to get organization IDs: %v", err)
		return
	}

	for _, orgID := range orgIDs {
		// Create a tenant-scoped context for each organization
		tenantCtx := auth.ContextWithClaims(ctx, &auth.Claims{OrganizationID: orgID})

		// Resolve conflicts
		// Prune stale context
		if err := w.service.PruneStaleContext(tenantCtx, time.Now().Add(-w.staleAfter)); err != nil {
			log.Printf("Failed to prune stale context for org %s: %v", orgID, err)
		}
	}
}
