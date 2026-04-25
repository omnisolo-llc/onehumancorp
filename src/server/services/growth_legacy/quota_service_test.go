package growth

import (
	"context"
	"github.com/alicebob/miniredis/v2"
	"github.com/onehumancorp/mono/src/server/lib/analytics"
	"github.com/redis/go-redis/v9"
	"testing"
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
	repo := NewReferralRepository(rdb)
	service := NewQuotaService(tracker, rdb, repo, 2)
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
	repo := NewReferralRepository(nil)
	service := NewQuotaService(tracker, nil, repo, 2)
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

func TestQuotaServiceDynamicLimit(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	tracker := analytics.NewTracker()
	repo := NewReferralRepository(rdb)
	service := NewQuotaService(tracker, rdb, repo, 2)
	ctx := context.Background()
	tenantID := "tenant-dynamic"

	// Baseline limit is 2. Let's use 2.
	service.IncrementUsage(ctx, tenantID)
	service.IncrementUsage(ctx, tenantID)

	// Now usage is 2, limit is 2 -> exceeded.
	allowed, _ := service.CheckQuota(ctx, tenantID)
	if allowed {
		t.Errorf("Expected quota to be exceeded")
	}

	// Add a SIGNED_UP referral for this tenant
	referral := &GrowthReferral{
		ID:           "ref-dyn-1",
		InviterID:    tenantID,
		InviteeEmail: "dyn@example.com",
		Status:       "SIGNED_UP",
	}
	err = repo.SaveReferral(ctx, referral)
	if err != nil {
		t.Fatalf("Failed to save referral: %v", err)
	}

	// Now dynamic limit should be 2 + 50 = 52.
	allowed, err = service.CheckQuota(ctx, tenantID)
	if err != nil || !allowed {
		t.Errorf("Expected quota to be allowed dynamically, got err: %v", err)
	}

	// Let's use up to 51
	for i := 0; i < 49; i++ {
		service.IncrementUsage(ctx, tenantID)
	}

	allowed, _ = service.CheckQuota(ctx, tenantID)
	if !allowed {
		t.Errorf("Expected quota to be allowed at usage 51")
	}

	// Usage 52 -> exceeded
	service.IncrementUsage(ctx, tenantID)
	allowed, _ = service.CheckQuota(ctx, tenantID)
	if allowed {
		t.Errorf("Expected quota to be exceeded at usage 52")
	}
}
