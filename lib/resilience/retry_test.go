package resilience

import (
	"context"
	"errors"
	"testing"
	"time"
)

func TestRetry_Success(t *testing.T) {
	ctx := context.Background()
	attempts := 0
	err := Retry(ctx, 3, 10*time.Millisecond, func(ctx context.Context) error {
		attempts++
		if attempts < 2 {
			return errors.New("temporary error")
		}
		return nil
	})

	if err != nil {
		t.Fatalf("expected success, got %v", err)
	}
	if attempts != 2 {
		t.Fatalf("expected 2 attempts, got %d", attempts)
	}
}

func TestRetry_Failure(t *testing.T) {
	ctx := context.Background()
	err := Retry(ctx, 3, 10*time.Millisecond, func(ctx context.Context) error {
		return errors.New("permanent error")
	})

	if err == nil {
		t.Fatalf("expected failure")
	}
}
