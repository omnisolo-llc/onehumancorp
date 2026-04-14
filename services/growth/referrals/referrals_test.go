package referrals

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

func TestReferralSystem(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec("CREATE TABLE referrals (tenant_id TEXT, code TEXT, user_id TEXT, usages INTEGER, PRIMARY KEY(tenant_id, code))")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	rs, err := NewReferralSystem(db)
	if err != nil {
		t.Fatalf("failed to create referral system: %v", err)
	}

	ctx := context.Background()
	tenantID := "tenant_1"

	userID := "user_nova_456"
	code, err := rs.GenerateCode(ctx, tenantID, userID)
	if err != nil {
		t.Fatalf("failed to generate code: %v", err)
	}
	if code == "" {
		t.Fatal("expected non-empty referral code")
	}

	referredUser, err := rs.UseCode(ctx, tenantID, code)
	if err != nil {
		t.Fatalf("failed to use code: %v", err)
	}
	if referredUser != userID {
		t.Errorf("expected referred user %s, got %s", userID, referredUser)
	}

	stats, err := rs.GetStats(ctx, tenantID, userID)
	if err != nil {
		t.Fatalf("failed to get stats: %v", err)
	}
	if stats != 1 {
		t.Errorf("expected 1 referral stat, got %d", stats)
	}

	_, err = rs.UseCode(ctx, tenantID, "invalid_code")
	if err == nil {
		t.Error("expected error for invalid code, got nil")
	}

	_, err = rs.UseCode(ctx, "wrong_tenant", code)
	if err == nil {
		t.Error("expected error for valid code but wrong tenant, got nil")
	}
}

func TestGetViralCoefficient(t *testing.T) {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}
	defer db.Close()

	_, err = db.Exec("CREATE TABLE referrals (tenant_id TEXT, code TEXT, user_id TEXT, usages INTEGER, PRIMARY KEY(tenant_id, code))")
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	rs, err := NewReferralSystem(db)
	if err != nil {
		t.Fatalf("failed to create referral system: %v", err)
	}

	ctx := context.Background()
	tenantID := "tenant_1"

	// 0 codes generated
	k, err := rs.GetViralCoefficient(ctx, tenantID)
	if err != nil {
		t.Fatalf("expected no error, got %v", err)
	}
	if k != 0.0 {
		t.Errorf("expected 0.0, got %f", k)
	}

	code1, _ := rs.GenerateCode(ctx, tenantID, "user_1")
	code2, _ := rs.GenerateCode(ctx, tenantID, "user_2")
	_, _ = rs.GenerateCode(ctx, tenantID, "user_2") // user_2 generated 2 codes

	k, _ = rs.GetViralCoefficient(ctx, tenantID)
	if k != 0.0 {
		t.Errorf("expected 0.0, got %f", k)
	}

	_, _ = rs.UseCode(ctx, tenantID, code1)
	_, _ = rs.UseCode(ctx, tenantID, code2)
	_, _ = rs.UseCode(ctx, tenantID, code2)

	// total usages = 3, distinct users = 2 (user_1, user_2)
	k, _ = rs.GetViralCoefficient(ctx, tenantID)
	if k != 1.5 {
		t.Errorf("expected 1.5, got %f", k)
	}
}
