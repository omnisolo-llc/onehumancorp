package resilience

import (
	"errors"
	"context"
	"fmt"
	"math/rand"
	"time"
)

// WithRetry executes a callback function repeatedly until it succeeds or the context is cancelled.
// It implements an exponential backoff strategy with a maximum number of retries.
func WithRetry(ctx context.Context, maxRetries int, initialBackoff time.Duration, callback func(ctx context.Context) error) error {
	var err error
	backoff := initialBackoff
	if backoff <= 0 {
		backoff = 1 * time.Millisecond // minimum safe backoff
	}

	for attempt := 0; attempt <= maxRetries; attempt++ {
		err = callback(ctx)
		if err == nil {
			return nil
		}
		if errors.Is(err, ErrCircuitOpen) {
			return err
		}

		if attempt == maxRetries {
			break
		}

		// Add jitter to avoid thundering herd
		jitterVal := int64(backoff) / 2
		if jitterVal <= 0 {
			jitterVal = 1
		}
		jitter := time.Duration(rand.Int63n(jitterVal))

		timer := time.NewTimer(backoff + jitter)

		select {
		case <-ctx.Done():
			timer.Stop()
			return fmt.Errorf("context cancelled during retry: %w (last error: %v)", ctx.Err(), err)
		case <-timer.C:
			// Exponential backoff
			backoff *= 2
		}
	}

	return fmt.Errorf("operation failed after %d retries: %w", maxRetries, err)
}


// WithCircuitBreakerRetry executes a callback function repeatedly using a Circuit Breaker.
// If the circuit breaker is open, it fails fast without retrying.
func WithCircuitBreakerRetry(ctx context.Context, cb *CircuitBreaker, maxRetries int, initialBackoff time.Duration, callback func(ctx context.Context) error) error {
	return WithRetry(ctx, maxRetries, initialBackoff, func(c context.Context) error {
		return cb.Execute(c, callback)
	})
}
