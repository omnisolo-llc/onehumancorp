package harness

import (
	"errors"
	"testing"
	"time"

	"github.com/stretchr/testify/assert"
)

func TestCircuitBreaker_Success(t *testing.T) {
	cb := NewCircuitBreaker(3, 10*time.Millisecond)

	err := cb.Execute(func() error {
		return nil
	}, nil)

	assert.NoError(t, err)
	assert.Equal(t, StateClosed, cb.state)
}

func TestCircuitBreaker_Trips(t *testing.T) {
	cb := NewCircuitBreaker(2, 10*time.Millisecond)

	// First failure
	_ = cb.Execute(func() error { return errors.New("fail") }, nil)
	assert.Equal(t, StateClosed, cb.state)

	// Second failure trips it
	_ = cb.Execute(func() error { return errors.New("fail") }, nil)
	assert.Equal(t, StateOpen, cb.state)

	// Third attempt immediately returns ErrCircuitOpen
	err := cb.Execute(func() error { return nil }, nil)
	assert.ErrorIs(t, err, ErrCircuitOpen)
}

func TestCircuitBreaker_Fallback(t *testing.T) {
	cb := NewCircuitBreaker(1, 10*time.Millisecond)

	// Trip it
	_ = cb.Execute(func() error { return errors.New("fail") }, nil)

	// Now it's open, should use fallback
	err := cb.Execute(func() error { return nil }, func() error {
		return errors.New("fallback executed")
	})

	assert.Equal(t, "fallback executed", err.Error())
}

func TestCircuitBreaker_Recovers(t *testing.T) {
	cb := NewCircuitBreaker(1, 10*time.Millisecond)

	// Trip it
	_ = cb.Execute(func() error { return errors.New("fail") }, nil)
	assert.Equal(t, StateOpen, cb.state)

	// Wait for timeout
	time.Sleep(15 * time.Millisecond)

	// Should transition to half-open and try, then succeed and close
	err := cb.Execute(func() error { return nil }, nil)
	assert.NoError(t, err)
	assert.Equal(t, StateClosed, cb.state)
}
