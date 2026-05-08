package rate_limiter

import (
	"context"
	"os"
	"testing"
)

func TestRateLimiterStandalone(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")

	rl := NewRateLimiterManager("")
	ctx := context.Background()
	bucket := "test_bucket"

	// Request 1 token
	allowed, err := rl.RequestTokens(ctx, bucket, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !allowed {
		t.Errorf("expected to be allowed")
	}

	// Drain bucket
	allowed, err = rl.RequestTokens(ctx, bucket, 99)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !allowed {
		t.Errorf("expected to be allowed")
	}

	// Should still be allowed (soft limit)
	allowed, err = rl.RequestTokens(ctx, bucket, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !allowed {
		t.Errorf("expected to be allowed")
	}

	// Check status
	status, err := rl.GetRateLimitStatus(ctx, bucket)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !status.IsAllowed {
		t.Errorf("expected isAllowed=true")
	}
	if !status.SoftLimitReached {
		t.Errorf("expected SoftLimitReached=true")
	}
}

func TestRateLimiterCallTool(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "false")
	defer os.Unsetenv("OHC_MULTITENANT")
	rl := NewRateLimiterManager("")
	ctx := context.Background()
	bucket := "test_bucket_tool"

	res, err := rl.CallTool(ctx, "RequestTokens", map[string]interface{}{
		"bucket": bucket,
		"amount": 10.0,
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	allowed, ok := res.(bool)
	if !ok || !allowed {
		t.Errorf("expected true")
	}

	resStatus, err := rl.CallTool(ctx, "GetRateLimitStatus", map[string]interface{}{
		"bucket": bucket,
	})
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	status, ok := resStatus.(RateLimitInfo)
	if !ok {
		t.Errorf("expected RateLimitInfo")
	}
	if !status.IsAllowed {
		t.Errorf("expected true")
	}
}

func TestRateLimiterCloudSoftFailWithoutRedis(t *testing.T) {
	os.Setenv("OHC_MULTITENANT", "true")
	defer os.Unsetenv("OHC_MULTITENANT")

	rl := NewRateLimiterManager("")
	ctx := context.Background()
	bucket := "test_bucket_cloud"

	// Should succeed even without redis
	allowed, err := rl.RequestTokens(ctx, bucket, 1)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !allowed {
		t.Errorf("expected true")
	}

	status, err := rl.GetRateLimitStatus(ctx, bucket)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}
	if !status.IsAllowed {
		t.Errorf("expected true")
	}
}

func TestRateLimiterCallToolUnknown(t *testing.T) {
	rl := NewRateLimiterManager("")
	_, err := rl.CallTool(context.Background(), "UnknownTool", nil)
	if err == nil {
		t.Errorf("expected error")
	}
}
