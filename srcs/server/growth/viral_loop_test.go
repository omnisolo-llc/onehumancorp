package growth

import (
	"context"
	"database/sql"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestProcessReferral(t *testing.T) {
	sqliteDb, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test sqlite db: %v", err)
	}
	defer sqliteDb.Close()

	// create table manually for the test
	_, err = sqliteDb.Exec(`
		CREATE TABLE IF NOT EXISTS growth_referrals (
			id TEXT PRIMARY KEY,
			organization_id TEXT NOT NULL,
			inviter_id TEXT NOT NULL,
			invitee_email TEXT NOT NULL,
			status TEXT NOT NULL,
			created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
		);
	`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	provider := db.NewSqliteProvider(sqliteDb)
	service := NewViralLoopService(provider)
	ctx := context.Background()

	ref, err := service.ProcessReferral(ctx, "org123", "user123", "friend@example.com")
	if err != nil {
		t.Fatalf("expected no error, got: %v", err)
	}

	if ref == nil {
		t.Fatal("expected referral to be returned")
	}
	if ref.OrganizationID != "org123" {
		t.Errorf("expected org123, got %s", ref.OrganizationID)
	}
	if ref.InviterID != "user123" {
		t.Errorf("expected user123, got %s", ref.InviterID)
	}
	if ref.InviteeEmail != "friend@example.com" {
		t.Errorf("expected friend@example.com, got %s", ref.InviteeEmail)
	}
	if ref.Status != "PENDING" {
		t.Errorf("expected PENDING, got %s", ref.Status)
	}
}
