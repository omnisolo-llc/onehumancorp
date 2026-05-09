package orchestration

import (
	"context"
	"encoding/json"
	"fmt"
	"log"
	"os"
	"sync"
	"time"

	"github.com/google/uuid"
)

type WorkerPool struct {
	queue           TaskQueue
	harnessResolver *HarnessResolver
	workerCount     int
	wg              sync.WaitGroup
	cancelFunc      context.CancelFunc
}

func NewWorkerPool(queue TaskQueue, resolver *HarnessResolver, workerCount int) *WorkerPool {
	return &WorkerPool{
		queue:           queue,
		harnessResolver: resolver,
		workerCount:     workerCount,
	}
}

func (wp *WorkerPool) Start(ctx context.Context) {
	ctx, cancel := context.WithCancel(ctx)
	wp.cancelFunc = cancel

	hostname, _ := os.Hostname()
	if hostname == "" {
		hostname = "unknown"
	}

	for i := 0; i < wp.workerCount; i++ {
		workerID := fmt.Sprintf("%s-%s-worker-%d", hostname, uuid.New().String(), i)
		wp.wg.Add(1)
		go wp.workerLoop(ctx, workerID)
	}
}

func (wp *WorkerPool) Stop() {
	if wp.cancelFunc != nil {
		wp.cancelFunc()
	}
	wp.wg.Wait()
}

func (wp *WorkerPool) workerLoop(ctx context.Context, workerID string) {
	defer wp.wg.Done()

	for {
		select {
		case <-ctx.Done():
			return
		default:
		}

		task, err := wp.queue.Dequeue(ctx, workerID)
		if err != nil {
			log.Printf("[%s] Error dequeuing task: %v", workerID, err)
			select {
			case <-ctx.Done():
				return
			case <-time.After(1 * time.Second): // Backoff on error
			}
			continue
		}

		if task == nil {
			// No task available, backoff and poll again
			select {
			case <-ctx.Done():
				return
			case <-time.After(1 * time.Second):
			}
			continue
		}

		err = wp.processTask(ctx, task)
		status := "COMPLETED"
		if err != nil {
			status = "FAILED"
			log.Printf("[%s] Task %s failed: %v", workerID, task.ID, err)
		}

		if ackErr := wp.queue.Acknowledge(ctx, task.ID, status); ackErr != nil {
			log.Printf("[%s] Error acknowledging task %s: %v", workerID, task.ID, ackErr)
		}
	}
}

type SubAgentPayload struct {
	AgentID string `json:"agent_id"`
	Command string `json:"command"`
}

func (wp *WorkerPool) processTask(ctx context.Context, task *SubAgentTask) error {
	var payload SubAgentPayload
	if err := json.Unmarshal(task.Payload, &payload); err != nil {
		return fmt.Errorf("failed to unmarshal payload: %w", err)
	}

	agentHarness, err := wp.harnessResolver.Resolve(payload.AgentID)
	if err != nil {
		return fmt.Errorf("failed to resolve harness for agent %s: %w", payload.AgentID, err)
	}

	// This is a blocking operation, bounded by harness timeout/context
	// Note: AgentHarness interface doesn't take context in RunAttempt currently,
	// assuming it's handled synchronously or we wrap it.
	result, err := agentHarness.RunAttempt(payload.Command)
	if err != nil {
		return fmt.Errorf("harness execution error: %w", err)
	}

	if result.ExitCode != 0 {
		return fmt.Errorf("harness execution failed with exit code %d: %s", result.ExitCode, result.Stderr)
	}

	return nil
}
