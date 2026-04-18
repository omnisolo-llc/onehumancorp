package services

import (
	"context"
	"testing"
	"time"
)

func TestDistributedLockService_Local(t *testing.T) {
	svc := NewDistributedLockService(nil, nil)
	ctx := context.Background()

	acquired, err := svc.AcquireLock(ctx, "test-lock", time.Minute, "token-1")
	if err != nil {
		t.Fatalf("failed to acquire lock: %v", err)
	}
	if !acquired {
		t.Fatalf("expected lock to be acquired")
	}

	if err := svc.ReleaseLock(ctx, "test-lock", "token-1"); err != nil {
		t.Fatalf("failed to release lock: %v", err)
	}
}
