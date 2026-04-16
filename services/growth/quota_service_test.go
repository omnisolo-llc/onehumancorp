package growth

import (
	"context"
	"testing"
	"github.com/onehumancorp/mono/lib/analytics"
	"github.com/alicebob/miniredis/v2"
	"github.com/redis/go-redis/v9"
)

func TestQuotaServiceCloud(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	tracker := analytics.NewTracker()
	service := NewQuotaService(tracker, rdb, 2)
	ctx := context.Background()
	tenantID := "tenant-123"

	// Test empty tenant
	_, err = service.CheckQuota(ctx, "")
	if err == nil {
		t.Errorf("Expected error for empty tenant ID")
	}

	err = service.IncrementUsage(ctx, "")
	if err == nil {
		t.Errorf("Expected error for empty tenant ID")
	}

	// Test normal usage
	allowed, err := service.CheckQuota(ctx, tenantID)
	if err != nil || !allowed {
		t.Errorf("Expected allowed for initial check")
	}

	err = service.IncrementUsage(ctx, tenantID)
	if err != nil {
		t.Errorf("IncrementUsage failed: %v", err)
	}

	err = service.IncrementUsage(ctx, tenantID)
	if err != nil {
		t.Errorf("IncrementUsage failed: %v", err)
	}

	// Test limit exceeded
	allowed, err = service.CheckQuota(ctx, tenantID)
	if err != nil || allowed {
		t.Errorf("Expected false after exceeding limit")
	}
}

func TestQuotaServiceStandalone(t *testing.T) {
	tracker := analytics.NewTracker()
	service := NewQuotaService(tracker, nil, 2)
	ctx := context.Background()
	tenantID := "tenant-standalone"

	// Test empty tenant
	_, err := service.CheckQuota(ctx, "")
	if err == nil {
		t.Errorf("Expected error for empty tenant ID")
	}

	err = service.IncrementUsage(ctx, "")
	if err == nil {
		t.Errorf("Expected error for empty tenant ID")
	}

	// Test normal usage (graceful degradation)
	allowed, err := service.CheckQuota(ctx, tenantID)
	if err != nil || !allowed {
		t.Errorf("Expected allowed for initial check")
	}

	err = service.IncrementUsage(ctx, tenantID)
	if err != nil {
		t.Errorf("IncrementUsage failed: %v", err)
	}

	// Will always allow in standalone mode without rdb
	allowed, err = service.CheckQuota(ctx, tenantID)
	if err != nil || !allowed {
		t.Errorf("Expected allowed for standalone check")
	}
}
