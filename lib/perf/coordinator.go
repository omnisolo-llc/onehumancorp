package perf

import (
	"context"
	"fmt"
	"sync"

	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

type Coordinator struct {
	db *orchestration.SIPDB
}

func NewCoordinator(db *orchestration.SIPDB) *Coordinator {
	return &Coordinator{
		db: db,
	}
}

func (c *Coordinator) ParallelUpdateMemory(ctx context.Context, updates map[string]string) error {
	var wg sync.WaitGroup
	errCh := make(chan error, len(updates))

	// Get max workers from context, fallback to default
	maxWorkers := GetTuneConfig(ctx).MaxWorkers
	if maxWorkers <= 0 {
		maxWorkers = 10
	}
	sem := make(chan struct{}, maxWorkers)

	for key, val := range updates {
		wg.Add(1)
		go func(k, v string) {
			defer wg.Done()
			sem <- struct{}{} // Acquire token
			if err := c.db.UpdateMemory(ctx, k, v); err != nil {
				errCh <- fmt.Errorf("failed to update memory for %s: %w", k, err)
			}
			<-sem // Release token
		}(key, val)
	}

	wg.Wait()
	close(errCh)

	for err := range errCh {
		if err != nil {
			return err
		}
	}
	return nil
}
