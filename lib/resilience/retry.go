package resilience

import (
	"context"
	"fmt"
	"time"
)

// Retry executes the given function up to attempts times with exponential backoff.
func Retry(ctx context.Context, attempts int, backoff time.Duration, fn func(context.Context) error) error {
	var err error
	for i := 0; i < attempts; i++ {
		err = fn(ctx)
		if err == nil {
			return nil
		}

		if i == attempts-1 {
			break
		}

		timer := time.NewTimer(backoff)
		select {
		case <-ctx.Done():
			timer.Stop()
			return ctx.Err()
		case <-timer.C:
			backoff *= 2
		}
	}
	return fmt.Errorf("retry failed after %d attempts: %w", attempts, err)
}
