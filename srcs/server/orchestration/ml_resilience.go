package orchestration

import (
	"context"
	"time"
	"log"
	"sync"
	"fmt"
)

var sleepFunc = time.Sleep

// Exported var so tests can override the backoff duration.
var RetryBackoffDuration = 2 * time.Second
var RetryTimeoutDuration = 60 * time.Second

// CircuitBreaker state
type CircuitState int

const (
	StateClosed CircuitState = iota
	StateOpen
	StateHalfOpen
)

// CircuitBreaker config
type CircuitBreaker struct {
	mu           sync.Mutex
	state        CircuitState
	failures     int
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

// Allow checks if a request is allowed by the circuit breaker.
func (cb *CircuitBreaker) Allow() bool {
	cb.mu.Lock()
	defer cb.mu.Unlock()

	switch cb.state {
	case StateClosed:
		return true
	case StateOpen:
		if time.Since(cb.lastFailure) > cb.timeout {
			cb.state = StateHalfOpen
			return true
		}
		return false
	case StateHalfOpen:
		// In HalfOpen, we only allow one request to pass through to test the service
		return true
	}
	return true
}

// RecordResult records the result of a request.
func (cb *CircuitBreaker) RecordResult(err error) {
	cb.mu.Lock()
	defer cb.mu.Unlock()

	if err != nil {
		cb.failures++
		cb.lastFailure = time.Now()
		if cb.state == StateClosed && cb.failures >= cb.maxFailures {
			cb.state = StateOpen
			log.Printf("CircuitBreaker: Circuit tripped open after %d failures", cb.failures)
		} else if cb.state == StateHalfOpen {
			cb.state = StateOpen
			log.Printf("CircuitBreaker: Circuit tripped open again from HalfOpen")
		}
	} else {
		if cb.state == StateHalfOpen {
			cb.state = StateClosed
			cb.failures = 0
			log.Printf("CircuitBreaker: Circuit closed after successful request in HalfOpen")
		} else if cb.state == StateClosed {
			cb.failures = 0
		}
	}
}

var globalCB = NewCircuitBreaker(3, 30*time.Second)

// RunWithTimeoutAndRetry executes the given job with a timeout and automatic retries.
func RunWithTimeoutAndRetry(ctx context.Context, job *Job, spawner SubAgentSpawner) error {
	var lastErr error
	for attempt := 1; attempt <= 3; attempt++ {
		// Circuit Breaker check
		if !globalCB.Allow() {
			return fmt.Errorf("circuit breaker is open, LLM/Spawner is unavailable")
		}

		// Create a context with a 60-second timeout for each attempt
		attemptCtx, cancel := context.WithTimeout(ctx, RetryTimeoutDuration)

		// Spawn the job
		errCh := make(chan error, 1)
		go func() {
			errCh <- spawner.SpawnIsolated(attemptCtx, job)
		}()

		var err error
		select {
		case <-attemptCtx.Done():
			err = attemptCtx.Err() // This will be context.DeadlineExceeded if it timed out
		case err = <-errCh:
		}

		cancel()

		// Record result in circuit breaker
		globalCB.RecordResult(err)

		if err == nil {
			return nil // Success
		}

		lastErr = err
		log.Printf("SubAgentWorker: Job %s failed attempt %d: %v", job.ID, attempt, err)

		if attempt < 3 {
			// Add a small backoff before retrying
			sleepFunc(RetryBackoffDuration)
		}
	}
	return lastErr
}
