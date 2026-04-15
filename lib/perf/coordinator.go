package perf

import (
	"context"
	"sync"
	"sync/atomic"
)

type CoordinatorMode struct {
	concurrency int
}

func NewCoordinatorMode(concurrency int) *CoordinatorMode {
	if concurrency <= 0 {
		concurrency = 4
	}
	return &CoordinatorMode{
		concurrency: concurrency,
	}
}

func (c *CoordinatorMode) ExecuteParallel(ctx context.Context, tasks []func() error) error {
	if len(tasks) == 0 {
		return nil
	}

	workerCount := c.concurrency
	if len(tasks) < workerCount {
		workerCount = len(tasks)
	}

	var wg sync.WaitGroup
	var firstErr atomic.Value

	// Calculate batch size dynamically
	batchSize := len(tasks) / workerCount
	if batchSize == 0 {
		batchSize = 1
	}

	for i := 0; i < workerCount; i++ {
		wg.Add(1)

		startIdx := i * batchSize
		endIdx := startIdx + batchSize
		if i == workerCount-1 {
			// Last worker gets all remaining tasks
			endIdx = len(tasks)
		}

		go func(start, end int) {
			defer wg.Done()

			for curr := start; curr < end; curr++ {
				// Check for cancellation or existing errors occasionally to save time
				if curr%64 == 0 {
					if err := ctx.Err(); err != nil {
						firstErr.CompareAndSwap(nil, err)
						return
					}
					if firstErr.Load() != nil {
						return
					}
				}

				if err := tasks[curr](); err != nil {
					firstErr.CompareAndSwap(nil, err)
					return
				}
			}
		}(startIdx, endIdx)
	}

	wg.Wait()

	if err := firstErr.Load(); err != nil {
		return err.(error)
	}
	return nil
}
