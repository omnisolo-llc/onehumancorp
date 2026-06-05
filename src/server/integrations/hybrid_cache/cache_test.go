package hybrid_cache

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestLocalCacheManager(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	cache := NewCacheManager(nil, "")
	ctx := context.Background()

	// Test Set and Get
	err := cache.SetCache(ctx, "key1", []byte("value1"), time.Minute)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	val, err := cache.GetCache(ctx, "key1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(val) != "value1" {
		t.Errorf("expected value1, got %s", string(val))
	}

	// Test Get non-existent
	_, err = cache.GetCache(ctx, "key2")
	if err != ErrKeyNotFound {
		t.Errorf("expected ErrKeyNotFound, got %v", err)
	}

	// Test Delete
	err = cache.DeleteCache(ctx, "key1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	_, err = cache.GetCache(ctx, "key1")
	if err != ErrKeyNotFound {
		t.Errorf("expected ErrKeyNotFound, got %v", err)
	}
}

func TestLocalCacheManager_Expiration(t *testing.T) {
	cache := NewLocalCacheManager()
	ctx := context.Background()

	err := cache.SetCache(ctx, "key1", []byte("value1"), 10*time.Millisecond)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	time.Sleep(20 * time.Millisecond)

	_, err = cache.GetCache(ctx, "key1")
	if err != ErrKeyNotFound {
		t.Errorf("expected ErrKeyNotFound after expiration, got %v", err)
	}
}

func TestRedisCacheManager(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("miniredis error: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	orgID := "org123"
	cache := NewCacheManager(client, orgID)
	ctx := context.Background()

	// Test Set and Get
	err = cache.SetCache(ctx, "key1", []byte("value1"), time.Minute)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}

	val, err := cache.GetCache(ctx, "key1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if string(val) != "value1" {
		t.Errorf("expected value1, got %s", string(val))
	}

	// Verify key has org prefix in redis
	mr.CheckGet(t, "org123:key1", "value1")

	// Test Get non-existent
	_, err = cache.GetCache(ctx, "key2")
	if err != ErrKeyNotFound {
		t.Errorf("expected ErrKeyNotFound, got %v", err)
	}

	// Test Delete
	err = cache.DeleteCache(ctx, "key1")
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	_, err = cache.GetCache(ctx, "key1")
	if err != ErrKeyNotFound {
		t.Errorf("expected ErrKeyNotFound, got %v", err)
	}
}

func TestRedisCacheManager_Error(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("miniredis error: %v", err)
	}
	defer mr.Close()

	client := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})
	orgID := "org123"
	cache := NewCacheManager(client, orgID)
	ctx := context.Background()

	mr.Close()

	_, err = cache.GetCache(ctx, "key1")
	if err == nil || err == ErrKeyNotFound {
		t.Errorf("expected connection error, got %v", err)
	}
}
