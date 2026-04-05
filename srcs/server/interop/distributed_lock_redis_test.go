package interop

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/rueidis"
)

func TestRedisLock_TryLock(t *testing.T) {
	s := miniredis.RunT(t)

	os.Setenv("REDIS_URL", "redis://"+s.Addr())
	defer os.Unsetenv("REDIS_URL")

	opts, err := rueidis.ParseURL("redis://" + s.Addr())
	if err != nil {
		t.Fatalf("failed to parse: %v", err)
	}
	opts.DisableCache = true // Miniredis doesn't fully support client-side caching

	c, err := rueidis.NewClient(opts)
	if err != nil {
		t.Fatalf("failed to connect: %v", err)
	}
	defer c.Close()

	rl := &redisLock{client: c}
	ctx := context.Background()
	key := "test_redis_lock"
	token := "token_1"
	otherToken := "token_2"

	// Acquire lock
	err = rl.TryLock(ctx, key, token, 1*time.Minute)
	if err != nil {
		t.Fatalf("expected to acquire lock, got %v", err)
	}

	// Fail to acquire again
	err = rl.TryLock(ctx, key, otherToken, 1*time.Minute)
	if err != ErrLockNotAcquired {
		t.Fatalf("expected ErrLockNotAcquired, got %v", err)
	}

	// Try to unlock with wrong token
	err = rl.Unlock(ctx, key, otherToken)
	if err != ErrLockNotHeld {
		t.Fatalf("expected ErrLockNotHeld, got %v", err)
	}

	// Unlock
	err = rl.Unlock(ctx, key, token)
	if err != nil {
		t.Fatalf("expected to unlock, got %v", err)
	}

	// Acquire again
	err = rl.TryLock(ctx, key, otherToken, 1*time.Minute)
	if err != nil {
		t.Fatalf("expected to acquire lock after unlock, got %v", err)
	}
}
