package hybrid_cache

import (
	"context"
	"os"
	"testing"
	"time"

	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestInMemoryCacheManager(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	cache := NewCacheManager(nil, "")

	ctx := context.Background()

	// Test Set and Get
	err := cache.SetCache(ctx, "test_key", []byte("test_value"), time.Minute)
	if err != nil {
		t.Fatalf("SetCache failed: %v", err)
	}

	val, err := cache.GetCache(ctx, "test_key")
	if err != nil {
		t.Fatalf("GetCache failed: %v", err)
	}
	if string(val) != "test_value" {
		t.Errorf("expected test_value, got %s", string(val))
	}

	// Test Delete
	err = cache.DeleteCache(ctx, "test_key")
	if err != nil {
		t.Fatalf("DeleteCache failed: %v", err)
	}

	_, err = cache.GetCache(ctx, "test_key")
	if err != ErrCacheMiss {
		t.Errorf("expected ErrCacheMiss, got %v", err)
	}

	// Test expiration
	err = cache.SetCache(ctx, "exp_key", []byte("exp_value"), 10*time.Millisecond)
	if err != nil {
		t.Fatalf("SetCache failed: %v", err)
	}

	time.Sleep(20 * time.Millisecond)

	_, err = cache.GetCache(ctx, "exp_key")
	if err != ErrCacheMiss {
		t.Errorf("expected ErrCacheMiss, got %v", err)
	}
}

func TestRedisCacheManager(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")

	s, err := miniredis.Run()
	if err != nil {
		panic(err)
	}
	defer s.Close()

	redisOpts := &redis.Options{
		Addr: s.Addr(),
	}

	orgID := "org123"
	cache := NewCacheManager(redisOpts, orgID)

	ctx := context.Background()

	// Test Set and Get
	err = cache.SetCache(ctx, "test_key", []byte("test_value"), time.Minute)
	if err != nil {
		t.Fatalf("SetCache failed: %v", err)
	}

	val, err := cache.GetCache(ctx, "test_key")
	if err != nil {
		t.Fatalf("GetCache failed: %v", err)
	}
	if string(val) != "test_value" {
		t.Errorf("expected test_value, got %s", string(val))
	}

	// Verify tenant isolation in underlying store
	expectedKey := "tenant_id:org123:test_key"
	if !s.Exists(expectedKey) {
		t.Errorf("expected key %s to exist in miniredis", expectedKey)
	}

	// Test Delete
	err = cache.DeleteCache(ctx, "test_key")
	if err != nil {
		t.Fatalf("DeleteCache failed: %v", err)
	}

	_, err = cache.GetCache(ctx, "test_key")
	if err != ErrCacheMiss {
		t.Errorf("expected ErrCacheMiss, got %v", err)
	}

	// Verify it's deleted from underlying store
	if s.Exists(expectedKey) {
		t.Errorf("expected key %s to be deleted from miniredis", expectedKey)
	}
}

func TestRedisCacheManager_NoOrgID(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")

	s, err := miniredis.Run()
	if err != nil {
		panic(err)
	}
	defer s.Close()

	redisOpts := &redis.Options{
		Addr: s.Addr(),
	}

	cache := NewCacheManager(redisOpts, "")

	ctx := context.Background()
	err = cache.SetCache(ctx, "test_key", []byte("test_value"), time.Minute)
	if err != nil {
		t.Fatalf("SetCache failed: %v", err)
	}

	if !s.Exists("test_key") {
		t.Errorf("expected key test_key to exist without tenant prefix")
	}
}
