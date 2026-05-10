package tiers

import (
	"context"
	"database/sql"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func TestTierService(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer db.Close()

	// Create tables for testing
	_, err = db.Exec(`
		CREATE TABLE tenants (
			id TEXT PRIMARY KEY,
			tier TEXT NOT NULL
		);
		CREATE TABLE tier_usage (
			tenant_id TEXT PRIMARY KEY,
			product_count INT DEFAULT 0,
			ai_actions_month INT DEFAULT 0,
			storage_bytes BIGINT DEFAULT 0,
			last_reset_date TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	tenantID := "test-tenant-1"
	_, err = db.Exec("INSERT INTO tenants (id, tier) VALUES (?, ?)", tenantID, "free")
	if err != nil {
		t.Fatalf("failed to insert tenant: %v", err)
	}

	// Missing tenant test
	tenantID2 := "missing-tenant"

	svc := NewTierService(db)
	ctx := context.Background()

	// Test Initial state (should allow 1 product)
	allowed, err := svc.CheckLimit(ctx, tenantID, "products", 1)
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if !allowed {
		t.Error("expected initial product creation to be allowed")
	}

	// Test missing tenant uses free tier
	allowed, err = svc.CheckLimit(ctx, tenantID2, "products", 1)
	if err != nil {
		t.Errorf("CheckLimit error for missing tenant: %v", err)
	}
	if !allowed {
		t.Error("expected initial product creation to be allowed for missing tenant")
	}

	// Test UpdateUsage products
	err = svc.UpdateUsage(ctx, tenantID, "products", 1)
	if err != nil {
		t.Errorf("UpdateUsage products error: %v", err)
	}

	// Test UpdateUsage ai_actions
	err = svc.UpdateUsage(ctx, tenantID, "ai_actions", 1)
	if err != nil {
		t.Errorf("UpdateUsage ai_actions error: %v", err)
	}

	// Test UpdateUsage storage
	err = svc.UpdateUsage(ctx, tenantID, "storage", 1024)
	if err != nil {
		t.Errorf("UpdateUsage storage error: %v", err)
	}

	// Test UpdateUsage unknown
	err = svc.UpdateUsage(ctx, tenantID, "unknown", 1)
	if err == nil {
		t.Errorf("UpdateUsage unknown error should not be nil")
	}

	// Test exceeding limit (free tier allows 10 products)
	allowed, err = svc.CheckLimit(ctx, tenantID, "products", 10) // 1 + 10 = 11 > 10
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if allowed {
		t.Error("expected product creation to be denied (limit exceeded)")
	}

	// Test exceeding limit (free tier allows 100 ai actions)
	allowed, err = svc.CheckLimit(ctx, tenantID, "ai_actions", 100) // 1 + 100 = 101 > 100
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if allowed {
		t.Error("expected ai_actions to be denied (limit exceeded)")
	}

	// Test exceeding limit (free tier allows 500MB storage)
	allowed, err = svc.CheckLimit(ctx, tenantID, "storage", 500*1024*1024)
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if allowed {
		t.Error("expected storage to be denied (limit exceeded)")
	}

	// Test exceeding limit ai_departments
	allowed, err = svc.CheckLimit(ctx, tenantID, "ai_departments", 1) // 1 > 1 (wait, increment is 1)
	// actually for free tier max ai deps is 1. If we increment by 2 it should fail
	allowed, err = svc.CheckLimit(ctx, tenantID, "ai_departments", 2)
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if allowed {
		t.Error("expected ai_departments to be denied (limit exceeded)")
	}

	// Test unknown metric
	_, err = svc.CheckLimit(ctx, tenantID, "unknown", 1)
	if err == nil {
		t.Errorf("CheckLimit unknown error should not be nil")
	}

	// Test Pro tier (unlimited)
	tenantID3 := "pro-tenant"
	_, err = db.Exec("INSERT INTO tenants (id, tier) VALUES (?, ?)", tenantID3, "pro")
	if err != nil {
		t.Fatalf("failed to insert tenant: %v", err)
	}

	allowed, err = svc.CheckLimit(ctx, tenantID3, "products", 1000)
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if !allowed {
		t.Error("expected unlimited products to be allowed")
	}

	// Test invalid tier strings fallback to free
	tenantID4 := "invalid-tier"
	_, err = db.Exec("INSERT INTO tenants (id, tier) VALUES (?, ?)", tenantID4, "random")
	if err != nil {
		t.Fatalf("failed to insert tenant: %v", err)
	}
	allowed, err = svc.CheckLimit(ctx, tenantID4, "products", 11)
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if allowed {
		t.Error("expected invalid tier to fallback to free")
	}

	tenantID5 := "empty-tier"
	_, err = db.Exec("INSERT INTO tenants (id, tier) VALUES (?, ?)", tenantID5, "")
	if err != nil {
		t.Fatalf("failed to insert tenant: %v", err)
	}
	allowed, err = svc.CheckLimit(ctx, tenantID5, "products", 11)
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if allowed {
		t.Error("expected empty tier to fallback to free")
	}

	// Make last reset date old
	_, err = db.Exec("UPDATE tier_usage SET last_reset_date = ? WHERE tenant_id = ?", time.Now().Add(-31*24*time.Hour), tenantID)
	if err != nil {
		t.Fatalf("failed to update last reset date: %v", err)
	}
	// Check ai actions again to trigger reset path
	allowed, err = svc.CheckLimit(ctx, tenantID, "ai_actions", 1)
	if err != nil {
		t.Errorf("CheckLimit error: %v", err)
	}
	if !allowed {
		t.Error("expected ai actions to be allowed after reset")
	}

	// Make sql query fail for tenant checking
	db.Close()
	_, err = svc.CheckLimit(ctx, tenantID, "products", 1)
	if err == nil {
		t.Errorf("CheckLimit should fail when db is closed")
	}
}

func TestTierLimits(t *testing.T) {
	// Simple test to ensure limits are defined properly
	if LimitsByTier[TierFree].MaxProducts != 10 {
		t.Errorf("expected free tier to have max 10 products")
	}
	if LimitsByTier[TierPro].MaxProducts != -1 {
		t.Errorf("expected pro tier to have unlimited products")
	}
}

func TestTierService_FailingUsageQuery(t *testing.T) {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open memory db: %v", err)
	}
	defer db.Close()

	// Create tables for testing but with missing column to trigger query failure
	_, err = db.Exec(`
		CREATE TABLE tenants (
			id TEXT PRIMARY KEY,
			tier TEXT NOT NULL
		);
		CREATE TABLE tier_usage (
			tenant_id TEXT PRIMARY KEY
		);
	`)
	if err != nil {
		t.Fatalf("failed to create tables: %v", err)
	}

	tenantID := "test-tenant-1"
	_, err = db.Exec("INSERT INTO tenants (id, tier) VALUES (?, ?)", tenantID, "free")
	if err != nil {
		t.Fatalf("failed to insert tenant: %v", err)
	}

	svc := NewTierService(db)
	ctx := context.Background()

	// Check limit should fail on usage query
	_, err = svc.CheckLimit(ctx, tenantID, "products", 1)
	if err == nil {
		t.Errorf("CheckLimit should fail on bad usage query")
	}
}
