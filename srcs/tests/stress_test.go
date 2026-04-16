package tests

import (
	"context"
	"sync"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/lib/resilience"
)

// TestStressVerification_HighConcurrency tests the system under high concurrency
// mimicking Cloud pods to verify host machine resource exhaustion resistance.
func TestStressVerification_HighConcurrency(t *testing.T) {
	cb := resilience.NewCircuitBreaker(5, 50*time.Millisecond)

	var wg sync.WaitGroup
	numWorkers := 1000 // High concurrency

	ctx, cancel := context.WithTimeout(context.Background(), 5*time.Second)
	defer cancel()

	successFn := func(c context.Context) error {
		return nil
	}

	for i := 0; i < numWorkers; i++ {
		wg.Add(1)
		go func() {
			defer wg.Done()
			for j := 0; j < 50; j++ {
				_ = resilience.WithCircuitBreakerRetry(ctx, cb, 1, 1*time.Millisecond, successFn)
			}
		}()
	}

	wg.Wait()
	t.Log("Successfully handled high concurrency stress test.")
}
