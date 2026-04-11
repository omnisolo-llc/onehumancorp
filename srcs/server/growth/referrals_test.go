package growth

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func setupTestDB(t *testing.T) db.Provider {
	sqliteDB, err := sql.Open("sqlite", "file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to open sqlite: %v", err)
	}

	// Create table for tests
	_, err = sqliteDB.Exec(`
		DROP TABLE IF EXISTS referral_links;
		CREATE TABLE referral_links (
			id TEXT PRIMARY KEY,
			user_id TEXT NOT NULL,
			code TEXT UNIQUE NOT NULL,
			uses_count INTEGER NOT NULL DEFAULT 0,
			created_at TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return &db.DB{Provider: db.NewSqliteProvider(sqliteDB)}
}

func TestReferralService_CreateAndUse(t *testing.T) {
	provider := setupTestDB(t)
	svc := NewReferralService(provider)
	ctx := context.Background()

	// 1. Create a referral code
	userID := "user-123"
	code, err := svc.CreateReferralCode(ctx, userID)
	if err != nil {
		t.Fatalf("expected no error creating code, got %v", err)
	}
	if code == "" {
		t.Fatalf("expected non-empty code")
	}

	// 2. Try to use an invalid code
	err = svc.RecordReferralUsage(ctx, "invalid-code")
	if err == nil {
		t.Fatalf("expected error for invalid code")
	}

	// 3. Use the valid code
	err = svc.RecordReferralUsage(ctx, code)
	if err != nil {
		t.Fatalf("expected no error recording usage, got %v", err)
	}

	// 4. Verify usage count
	var usesCount int
	err = provider.QueryRow(ctx, "SELECT uses_count FROM referral_links WHERE code = $1", code).Scan(&usesCount)
	if err != nil {
		t.Fatalf("expected no error querying usage count, got %v", err)
	}
	if usesCount != 1 {
		t.Fatalf("expected usage count to be 1, got %d", usesCount)
	}
}
