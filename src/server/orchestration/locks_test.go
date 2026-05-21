package orchestration

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestStandaloneLock_Acquire(t *testing.T) {
	lock := NewStandaloneLock()
	ctx := context.Background()

	unlock, err := lock.Acquire(ctx, "task1")
	if err != nil {
		t.Fatalf("failed to acquire lock: %v", err)
	}

	err = unlock()
	if err != nil {
		t.Fatalf("failed to unlock: %v", err)
	}
}

func TestRedisLock_Acquire(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	lock := NewRedisLock(client)
	ctx := context.Background()

	unlock, err := lock.Acquire(ctx, "task_redis_1")
	if err != nil {
		t.Fatalf("failed to acquire redis lock: %v", err)
	}

	// Try acquiring again should fail with timeout/locked
	ctxTimeout, cancel := context.WithTimeout(context.Background(), 100*time.Millisecond)
	defer cancel()

	_, err2 := lock.Acquire(ctxTimeout, "task_redis_1")
	if err2 == nil {
		t.Fatalf("expected error acquiring already locked redis lock")
	}

	err = unlock()
	if err != nil {
		t.Fatalf("failed to unlock redis lock: %v", err)
	}

	// Should be able to acquire again
	unlock2, err3 := lock.Acquire(ctx, "task_redis_1")
	if err3 != nil {
		t.Fatalf("failed to acquire lock after unlock: %v", err3)
	}
	_ = unlock2()
}
