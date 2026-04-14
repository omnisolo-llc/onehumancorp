package builtin

import (
	"context"
	"fmt"
	"sync"
)

// SubagentTask is one scheduled unit of work.
type SubagentTask func(ctx context.Context, index int) error

// ScheduleSubagents runs total subagent tasks with a bounded worker pool.
func ScheduleSubagents(ctx context.Context, total, workers int, task SubagentTask) error {
	if total <= 0 {
		return nil
	}
	if workers <= 0 {
		workers = 1
	}
	if task == nil {
		return fmt.Errorf("task function is required")
	}

	ctx, cancel := context.WithCancel(ctx)
	defer cancel()

	jobs := make(chan int)
	var wg sync.WaitGroup
	var once sync.Once
	var runErr error

	workerFn := func() {
		defer wg.Done()
		for idx := range jobs {
			if err := task(ctx, idx); err != nil {
				once.Do(func() {
					runErr = err
					cancel()
				})
				return
			}
		}
	}

	wg.Add(workers)
	for i := 0; i < workers; i++ {
		go workerFn()
	}

	for i := 0; i < total; i++ {
		select {
		case <-ctx.Done():
			break
		case jobs <- i:
		}
	}
	close(jobs)
	wg.Wait()

	return runErr
}
