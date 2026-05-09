package harness

import (
	"errors"
	"sync"
	"time"
)

type CircuitState int

const (
	StateClosed CircuitState = iota
	StateOpen
	StateHalfOpen
)

var (
	ErrCircuitOpen = errors.New("circuit breaker is open")
)

type CircuitBreaker struct {
	mu           sync.RWMutex
	state        CircuitState
	failureCount int
	maxFailures  int
	timeout      time.Duration
	lastFailure  time.Time
}

func NewCircuitBreaker(maxFailures int, timeout time.Duration) *CircuitBreaker {
	return &CircuitBreaker{
		state:       StateClosed,
		maxFailures: maxFailures,
		timeout:     timeout,
	}
}

func (cb *CircuitBreaker) Execute(operation func() error, fallback func() error) error {
	cb.mu.RLock()
	state := cb.state
	lastFailure := cb.lastFailure
	cb.mu.RUnlock()

	if state == StateOpen {
		if time.Since(lastFailure) > cb.timeout {
			cb.mu.Lock()
			cb.state = StateHalfOpen
			cb.mu.Unlock()
		} else {
			if fallback != nil {
				return fallback()
			}
			return ErrCircuitOpen
		}
	}

	err := operation()

	cb.mu.Lock()
	defer cb.mu.Unlock()

	if err != nil {
		cb.failureCount++
		cb.lastFailure = time.Now()
		if cb.failureCount >= cb.maxFailures {
			cb.state = StateOpen
		}
		if fallback != nil {
			return fallback()
		}
		return err
	}

	cb.failureCount = 0
	cb.state = StateClosed
	return nil
}
