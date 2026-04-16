package locks

import (
	"context"
	"testing"
	"time"

	"github.com/redis/go-redis/v9"
)

func TestMemoryLocker(t *testing.T) {
	locker := NewMemoryLocker()
	ctx := context.Background()

	// Acquire lock
	ok, err := locker.Acquire(ctx, "test-resource", 1*time.Second)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !ok {
		t.Fatal("expected to acquire lock")
	}

	// Fail to acquire already held lock
	ok, err = locker.Acquire(ctx, "test-resource", 1*time.Second)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if ok {
		t.Fatal("expected to fail acquiring lock")
	}

	// Release lock
	err = locker.Release(ctx, "test-resource")
	if err != nil {
		t.Fatalf("unexpected error on release: %v", err)
	}

	// Acquire again
	ok, err = locker.Acquire(ctx, "test-resource", 1*time.Second)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !ok {
		t.Fatal("expected to acquire lock after release")
	}
}

func TestRedisLocker(t *testing.T) {
	// Skip or mock Redis in actual tests if Redis server is not available.
	// For simplicity, we just verify it compiles and instantiates.
	client := redis.NewClient(&redis.Options{Addr: "localhost:6379"})
	locker := NewRedisLocker(client)
	if locker == nil {
		t.Fatal("expected non-nil RedisLocker")
	}
}
