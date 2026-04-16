package resilience

import (
	"context"
	"errors"
	"github.com/prometheus/client_golang/prometheus"
	"sync"
	"time"
)

// State represents the state of the circuit breaker.
type State int

const (
	StateClosed State = iota
	StateHalfOpen
	StateOpen
)

var (
	ErrCircuitOpen = errors.New("circuit breaker is open")
)

var (
	circuitBreakerStateChanges = prometheus.NewCounterVec(
		prometheus.CounterOpts{
			Name: "ohc_circuit_breaker_state_changes_total",
			Help: "Total number of state changes in the circuit breaker",
		},
		[]string{"state"},
	)
)

func init() {
	prometheus.MustRegister(circuitBreakerStateChanges)
}

// CircuitBreaker implements the fail-safe mechanism for degradation validation.
type CircuitBreaker struct {
	mu           sync.Mutex
	state        State
	probeInFlight bool
	failureCount int
	maxFailures  int
	resetTimeout time.Duration
	lastFailure  time.Time
}

// NewCircuitBreaker creates a new circuit breaker.
func NewCircuitBreaker(maxFailures int, resetTimeout time.Duration) *CircuitBreaker {
	return &CircuitBreaker{
		state:        StateClosed,
		maxFailures:  maxFailures,
		resetTimeout: resetTimeout,
	}
}

// Execute runs the given function if the circuit is closed or half-open.
func (cb *CircuitBreaker) Execute(ctx context.Context, fn func(context.Context) error) error {
	cb.mu.Lock()
	switch cb.state {
	case StateOpen:
		if time.Since(cb.lastFailure) > cb.resetTimeout {
			cb.state = StateHalfOpen
			circuitBreakerStateChanges.WithLabelValues("half_open").Inc()
			cb.probeInFlight = true
		} else {
			cb.mu.Unlock()
			return ErrCircuitOpen
		}
	case StateHalfOpen:
		if cb.probeInFlight {
			cb.mu.Unlock()
			return ErrCircuitOpen // Deny concurrent requests while probing
		}
		// In a real system, another probe might be allowed after a timeout, but for now we just deny.
	}
	cb.mu.Unlock()

	err := fn(ctx)

	cb.mu.Lock()
	defer cb.mu.Unlock()

	if err != nil {
		cb.failureCount++
		cb.lastFailure = time.Now()
		if cb.state == StateHalfOpen || cb.failureCount >= cb.maxFailures {
			if cb.state != StateOpen {
				cb.state = StateOpen
				circuitBreakerStateChanges.WithLabelValues("open").Inc()
			}
		}
		cb.probeInFlight = false
		return err
	}

	cb.failureCount = 0
	if cb.state != StateClosed {
		cb.state = StateClosed
		circuitBreakerStateChanges.WithLabelValues("closed").Inc()
	}
	cb.probeInFlight = false
	return nil
}
