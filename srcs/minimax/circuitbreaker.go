package minimax

import (
	"context"
	"errors"
	"sync"
	"time"
)

type State int

const (
	StateClosed State = iota
	StateOpen
	StateHalfOpen
)

type CircuitBreaker struct {
	mu             sync.Mutex
	state          State
	failures       int
	maxFailures    int
	lastFailureAt  time.Time
	resetTimeout   time.Duration
	client         *Client
}

func NewCircuitBreaker(client *Client, maxFailures int, resetTimeout time.Duration) *CircuitBreaker {
	return &CircuitBreaker{
		state:        StateClosed,
		maxFailures:  maxFailures,
		resetTimeout: resetTimeout,
		client:       client,
	}
}

func (cb *CircuitBreaker) Reason(ctx context.Context, prompt string) (string, error) {
	cb.mu.Lock()
	switch cb.state {
	case StateOpen:
		if time.Since(cb.lastFailureAt) > cb.resetTimeout {
			cb.state = StateHalfOpen
		} else {
			cb.mu.Unlock()
			return "", errors.New("circuit breaker is open")
		}
	case StateHalfOpen, StateClosed:
		// proceed
	}
	cb.mu.Unlock()

	result, err := cb.client.Reason(ctx, prompt)

	cb.mu.Lock()
	defer cb.mu.Unlock()

	if err != nil {
		cb.failures++
		cb.lastFailureAt = time.Now()
		if cb.failures >= cb.maxFailures || cb.state == StateHalfOpen {
			cb.state = StateOpen
		}
		return "", err
	}

	cb.state = StateClosed
	cb.failures = 0
	return result, nil
}

func (cb *CircuitBreaker) Reset() {
	cb.mu.Lock()
	defer cb.mu.Unlock()
	cb.state = StateClosed
	cb.failures = 0
	cb.lastFailureAt = time.Time{}
}
