package resilience

import (
	"context"
	"testing"
	"time"
)

func TestDummyLock(t *testing.T) {
	lock := &DummyLock{}
	err := lock.Lock(context.Background(), time.Second)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	err = lock.Unlock(context.Background())
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
}
