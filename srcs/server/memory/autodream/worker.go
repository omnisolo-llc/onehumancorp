package autodream

import (
	"context"
	"fmt"
	"sync"
	"time"
)

type PruneWorker struct {
	service        MemoryConsolidator
	organizationID string
	threshold      time.Duration
	ticker         *time.Ticker
	quit           chan struct{}
	stopOnce       sync.Once
}

func NewPruneWorker(service MemoryConsolidator, organizationID string, threshold time.Duration, interval time.Duration) *PruneWorker {
	return &PruneWorker{
		service:        service,
		organizationID: organizationID,
		threshold:      threshold,
		ticker:         time.NewTicker(interval),
		quit:           make(chan struct{}),
	}
}

func (w *PruneWorker) Start(ctx context.Context) {
	go func() {
		for {
			select {
			case <-w.ticker.C:
				w.prune(ctx)
			case <-w.quit:
				w.ticker.Stop()
				return
			}
		}
	}()
}

func (w *PruneWorker) Stop() {
	w.stopOnce.Do(func() {
		close(w.quit)
	})
}

func (w *PruneWorker) prune(ctx context.Context) {
	deleted, err := w.service.PruneStaleContext(ctx, w.organizationID, w.threshold)
	if err != nil {
		fmt.Printf("PruneWorker: failed to prune stale context for org %s: %v\n", w.organizationID, err)
		return
	}
	if deleted > 0 {
		fmt.Printf("PruneWorker: successfully pruned %d stale context records for org %s.\n", deleted, w.organizationID)
	}
}
