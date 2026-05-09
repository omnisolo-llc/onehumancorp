package harness

import (
	"context"
	"errors"
	"time"
)

type AgentHarness struct {
	timeout    time.Duration
	maxRetries int
}

func NewAgentHarness() *AgentHarness {
	return &AgentHarness{
		timeout:    60 * time.Second,
		maxRetries: 3,
	}
}

func (h *AgentHarness) ExecuteJob(ctx context.Context, jobFunc func(context.Context) error) error {
	var lastErr error
	for attempt := 0; attempt < h.maxRetries; attempt++ {
		// Respect parent context
		if ctx.Err() != nil {
			return ctx.Err()
		}

		jobCtx, cancel := context.WithTimeout(ctx, h.timeout)

		errChan := make(chan error, 1)
		go func() {
			errChan <- jobFunc(jobCtx)
		}()

		select {
		case <-jobCtx.Done():
			err := jobCtx.Err()
			if errors.Is(err, context.DeadlineExceeded) {
				lastErr = errors.New("job timed out")
			} else {
				lastErr = err
			}
			cancel()
		case err := <-errChan:
			cancel()
			if err == nil {
				return nil
			}
			lastErr = err
		}

		// Don't retry if parent context is done
		if ctx.Err() != nil {
			return ctx.Err()
		}

		// Backoff before retry
		select {
		case <-ctx.Done():
			return ctx.Err()
		case <-time.After(time.Duration(1<<attempt) * time.Second):
		}
	}
	return lastErr
}
