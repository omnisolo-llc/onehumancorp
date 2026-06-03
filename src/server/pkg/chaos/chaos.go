package chaos

import (
	"math/rand"
	"sync/atomic"
	"time"
)

// SimulateNetworkDelay injects a delay to simulate network latency.
func SimulateNetworkDelay(ms uint64) {
	if ms > 0 {
		time.Sleep(time.Duration(ms) * time.Millisecond)
	}
}

// ShouldDropPacket determines if a packet should be dropped based on a given probability.
func ShouldDropPacket(probability float64) bool {
	return probability > 0.0 && rand.Float64() < probability
}

// CircuitBreaker represents a simple circuit breaker for mesh resilience.
type CircuitBreaker struct {
	failures  atomic.Uint64
	threshold uint64
}

// NewCircuitBreaker creates a new CircuitBreaker.
func NewCircuitBreaker(threshold uint64) *CircuitBreaker {
	return &CircuitBreaker{
		threshold: threshold,
	}
}

// RecordFailure records a failure.
func (cb *CircuitBreaker) RecordFailure() {
	cb.failures.Add(1)
}

// IsOpen returns true if the circuit breaker is open.
func (cb *CircuitBreaker) IsOpen() bool {
	return cb.failures.Load() >= cb.threshold
}

// Reset resets the circuit breaker.
func (cb *CircuitBreaker) Reset() {
	cb.failures.Store(0)
}
