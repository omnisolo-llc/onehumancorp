package growth

import (
	"context"
	"database/sql"
	"testing"

	_ "modernc.org/sqlite"
)

func setupDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open db: %v", err)
	}

	_, err = db.Exec(`CREATE TABLE invites (
		code TEXT PRIMARY KEY,
		team_id TEXT NOT NULL,
		inviter_id TEXT NOT NULL,
		created_at DATETIME NOT NULL,
		redeemed BOOLEAN NOT NULL DEFAULT false
	)`)
	if err != nil {
		t.Fatalf("failed to create table: %v", err)
	}

	return db
}

func TestDBReferralManager(t *testing.T) {
	db := setupDB(t)
	defer db.Close()

	ctx := context.Background()
	rm := NewDBReferralManager(db)

	// Test generation
	invite, err := rm.GenerateInvite(ctx, "team-123", "user-456")
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if invite.Code == "" {
		t.Error("expected code to be generated")
	}

	// Test redemption
	redeemed, err := rm.RedeemInvite(ctx, invite.Code)
	if err != nil {
		t.Fatalf("unexpected error: %v", err)
	}

	if !redeemed.Redeemed {
		t.Error("expected invite to be redeemed")
	}

	// Test double redemption
	_, err = rm.RedeemInvite(ctx, invite.Code)
	if err == nil {
		t.Error("expected error for double redemption")
	}

	// Test not found
	_, err = rm.RedeemInvite(ctx, "invalid-code")
	if err == nil {
		t.Error("expected error for not found invite")
	}
}
