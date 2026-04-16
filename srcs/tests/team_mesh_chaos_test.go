package tests

import (
	"context"
	"errors"
	"sync/atomic"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/lib/resilience"
)

// TestTeamMeshChaos_CircuitBreaker tests the chaos resilience of a Mesh network
// ensuring the Circuit Breaker correctly trips and fails fast.
func TestTeamMeshChaos_CircuitBreaker(t *testing.T) {
	cb := resilience.NewCircuitBreaker(3, 500*time.Millisecond)

	var fastFailCount int32
	var realFailCount int32

	ctx, cancel := context.WithTimeout(context.Background(), 2*time.Second)
	defer cancel()

	// Simulate an entirely broken external dependency
	brokenDependency := func(c context.Context) error {
		return errors.New("simulated network partition")
	}

	// Sequential execution to reliably trip the circuit breaker
	// First 3 requests should fail with real errors and trip it.
	for i := 0; i < 3; i++ {
		err := resilience.WithCircuitBreakerRetry(ctx, cb, 0, 1*time.Millisecond, brokenDependency)
		if err == nil {
			t.Errorf("Expected an error, got nil")
		} else if errors.Is(err, resilience.ErrCircuitOpen) {
			t.Errorf("Expected real failure, got ErrCircuitOpen on attempt %d", i+1)
		} else {
			realFailCount++
		}
	}

	// Now the circuit breaker should be open. Subsequent requests should fast-fail.
	for i := 0; i < 10; i++ {
		err := resilience.WithCircuitBreakerRetry(ctx, cb, 0, 1*time.Millisecond, brokenDependency)
		if err == nil {
			t.Errorf("Expected an error, got nil")
		} else if errors.Is(err, resilience.ErrCircuitOpen) {
			atomic.AddInt32(&fastFailCount, 1)
		} else {
			t.Errorf("Expected ErrCircuitOpen, got real error")
		}
	}

	if fastFailCount != 10 {
		t.Errorf("Expected 10 fast-fails, got %d", fastFailCount)
	}

	if realFailCount != 3 {
		t.Errorf("Expected 3 real failures, got %d", realFailCount)
	}
}
