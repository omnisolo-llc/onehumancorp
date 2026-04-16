package resilience

import (
	"context"
	"errors"
	"testing"
	"time"
)

func TestCircuitBreaker_Success(t *testing.T) {
	cb := NewCircuitBreaker(3, 100*time.Millisecond)
	err := cb.Execute(context.Background(), func(ctx context.Context) error {
		return nil
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if cb.state != StateClosed {
		t.Fatalf("expected state StateClosed, got %v", cb.state)
	}
}

func TestCircuitBreaker_OpenAndReset(t *testing.T) {
	cb := NewCircuitBreaker(2, 50*time.Millisecond)

	failingFn := func(ctx context.Context) error {
		return errors.New("failure")
	}

	// 1st failure -> Closed
	_ = cb.Execute(context.Background(), failingFn)
	if cb.state != StateClosed {
		t.Fatalf("expected StateClosed, got %v", cb.state)
	}

	// 2nd failure -> Open
	_ = cb.Execute(context.Background(), failingFn)
	if cb.state != StateOpen {
		t.Fatalf("expected StateOpen, got %v", cb.state)
	}

	// 3rd failure -> fast fail
	err := cb.Execute(context.Background(), func(ctx context.Context) error {
		return nil
	})
	if !errors.Is(err, ErrCircuitOpen) {
		t.Fatalf("expected ErrCircuitOpen, got %v", err)
	}

	// Wait for reset timeout
	time.Sleep(60 * time.Millisecond)

	// Next execution should be HalfOpen and succeed -> Closed
	err = cb.Execute(context.Background(), func(ctx context.Context) error {
		return nil
	})
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if cb.state != StateClosed {
		t.Fatalf("expected StateClosed, got %v", cb.state)
	}
}

func TestCircuitBreaker_HalfOpenToOpen(t *testing.T) {
	cb := NewCircuitBreaker(1, 50*time.Millisecond)
	failingFn := func(ctx context.Context) error {
		return errors.New("failure")
	}

	// 1st failure -> Open
	_ = cb.Execute(context.Background(), failingFn)

	// Wait for reset
	time.Sleep(60 * time.Millisecond)

	// Execute fails again -> back to Open
	_ = cb.Execute(context.Background(), failingFn)
	if cb.state != StateOpen {
		t.Fatalf("expected StateOpen, got %v", cb.state)
	}
}
