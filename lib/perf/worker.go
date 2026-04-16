package perf

import (
	"context"
	"runtime"
	"sync"
)

// Coordinator manages parallel execution of tasks across worker threads.
// It implements Claude-inspired parallelization for the OHC Team Mesh.
type Coordinator struct {
	numWorkers int
	taskQueue  chan Task
	wg         sync.WaitGroup
}

// Task represents a unit of work for a worker thread.
type Task func(ctx context.Context) error

// NewCoordinator creates a new Coordinator with the specified number of workers.
// If numWorkers is <= 0, it defaults to the number of logical CPUs.
func NewCoordinator(numWorkers int) *Coordinator {
	if numWorkers <= 0 {
		numWorkers = runtime.NumCPU()
	}
	return &Coordinator{
		numWorkers: numWorkers,
		taskQueue:  make(chan Task, numWorkers*2),
	}
}

// Start begins processing tasks in the background.
func (c *Coordinator) Start(ctx context.Context) {
	for i := 0; i < c.numWorkers; i++ {
		c.wg.Add(1)
		go func() {
			defer c.wg.Done()
			for {
				select {
				case <-ctx.Done():
					return
				case task, ok := <-c.taskQueue:
					if !ok {
						return
					}
					// Execute task and ignore error for now, could add error channel
					_ = task(ctx)
				}
			}
		}()
	}
}

// Submit adds a task to the queue for parallel execution.
func (c *Coordinator) Submit(task Task) {
	c.taskQueue <- task
}

// Stop gracefully stops the workers after completing queued tasks.
func (c *Coordinator) Stop() {
	close(c.taskQueue)
	c.wg.Wait()
}
