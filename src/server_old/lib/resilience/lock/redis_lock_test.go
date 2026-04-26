package lock

import (
	"context"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/rueidis"
)

func TestRedisLockProvider(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("Failed to start miniredis: %v", err)
	}
	defer s.Close()

	client, err := rueidis.NewClient(rueidis.ClientOption{
		InitAddress: []string{s.Addr()},
		DisableCache: true, // required for miniredis since client-side caching is not fully supported
	})
	if err != nil {
		t.Fatalf("Failed to connect to miniredis: %v", err)
	}
	defer client.Close()

	provider := NewRedisLockProvider(client)
	ctx := context.Background()

	t.Run("acquire new lock", func(t *testing.T) {
		// Test acquiring lock
		locked, unlock, err := provider.TryLock(ctx, "test-key", 5*time.Second)
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}
		if !locked {
			t.Fatalf("Expected to acquire lock")
		}

		// Test trying to acquire same lock
		locked2, _, err2 := provider.TryLock(ctx, "test-key", 5*time.Second)
		if err2 != nil {
			t.Fatalf("Expected no error, got %v", err2)
		}
		if locked2 {
			t.Fatalf("Expected NOT to acquire lock")
		}

		// Unlock
		err = unlock(ctx)
		if err != nil {
			t.Fatalf("Expected no error on unlock, got %v", err)
		}

		// Try acquiring again after unlock
		locked3, unlock3, err3 := provider.TryLock(ctx, "test-key", 5*time.Second)
		if err3 != nil {
			t.Fatalf("Expected no error, got %v", err3)
		}
		if !locked3 {
			t.Fatalf("Expected to acquire lock")
		}
		unlock3(ctx)
	})

	t.Run("acquire expired lock", func(t *testing.T) {
		s.FastForward(0) // Ensure time works correctly

		// Acquire lock with short TTL
		locked, unlock, err := provider.TryLock(ctx, "test-key-expired", 1*time.Second)
		if err != nil {
			t.Fatalf("Expected no error, got %v", err)
		}
		if !locked {
			t.Fatalf("Expected to acquire lock")
		}

		// Fast forward miniredis time by more than TTL
		s.FastForward(2 * time.Second)

		// Acquire should succeed because previous lock expired
		locked2, unlock2, err2 := provider.TryLock(ctx, "test-key-expired", 5*time.Second)
		if err2 != nil {
			t.Fatalf("Expected no error, got %v", err2)
		}
		if !locked2 {
			t.Fatalf("Expected to acquire expired lock")
		}

		// Unlocking first lock should not affect second lock
		err = unlock(ctx)
		if err != nil {
			t.Fatalf("Expected no error on old unlock, got %v", err)
		}

		// Trying to acquire again should fail since it's held by locked2
		locked3, _, err3 := provider.TryLock(ctx, "test-key-expired", 5*time.Second)
		if err3 != nil {
			t.Fatalf("Expected no error, got %v", err3)
		}
		if locked3 {
			t.Fatalf("Expected NOT to acquire lock")
		}

		unlock2(ctx)
	})
}
