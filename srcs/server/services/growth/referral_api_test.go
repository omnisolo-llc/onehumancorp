package growth

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"strings"
	"testing"

	"github.com/onehumancorp/mono/srcs/server/auth"
	"github.com/onehumancorp/mono/srcs/server/db"
)

func TestGenerateReferralLink(t *testing.T) {
	userID := "user123"
	link, err := GenerateReferralLink(userID)

	if err != nil {
		t.Fatalf("Expected no error, got %v", err)
	}

	if !strings.HasPrefix(link, "ohc://join?ref=") {
		t.Errorf("Link %s does not have expected prefix", link)
	}

	if !strings.Contains(link, "utm_source=standalone_desktop") {
		t.Errorf("Link %s missing utm_source", link)
	}

	if !strings.Contains(link, "inviter=user123") {
		t.Errorf("Link %s missing inviter", link)
	}
}

func TestGenerateReferralLink_EmptyUser(t *testing.T) {
	_, err := GenerateReferralLink("")
	if err == nil {
		t.Fatalf("Expected error for empty user ID, got nil")
	}
}


func TestApplyReferralHandler(t *testing.T) {
	dbProvider := db.NewTestProvider(t)
	dbProvider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS users (
			id              TEXT PRIMARY KEY,
			username        TEXT UNIQUE NOT NULL,
			email           TEXT UNIQUE NOT NULL,
			password_hash   TEXT NOT NULL DEFAULT '',
			roles           TEXT NOT NULL DEFAULT '{}',
			active          BOOLEAN NOT NULL DEFAULT TRUE,
			organization_id TEXT NOT NULL DEFAULT '',
			oidc_subject    TEXT UNIQUE DEFAULT NULL,
			created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
			updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
			referral_code   TEXT UNIQUE,
			referred_by     TEXT REFERENCES users(id)
		);
	`)
	defer dbProvider.Close()

	ctx := context.Background()

	_, err := dbProvider.Exec(ctx, "INSERT INTO users (id, username, email, referral_code) VALUES ('user1', 'inviter', 'inviter@test.com', 'CODE123')")
	if err != nil {
		t.Fatalf("failed to insert inviter: %v", err)
	}

	_, err = dbProvider.Exec(ctx, "INSERT INTO users (id, username, email, referral_code) VALUES ('user2', 'invitee', 'invitee@test.com', 'CODE456')")
	if err != nil {
		t.Fatalf("failed to insert invitee: %v", err)
	}

	handler := ApplyReferralHandler(dbProvider)

	claims := &auth.Claims{Subject: "user2"}
	reqCtx := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	reqBody := ApplyReferralRequest{
		UserID:       "user2",
		ReferralCode: "CODE123",
	}
	bodyBytes, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/api/referrals/apply", bytes.NewReader(bodyBytes))
	req = req.WithContext(reqCtx)
	rr := httptest.NewRecorder()

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp ApplyReferralResponse
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	if !resp.Success {
		t.Errorf("expected success to be true")
	}

	var referredBy string
	err = dbProvider.QueryRow(ctx, "SELECT referred_by FROM users WHERE id = 'user2'").Scan(&referredBy)
	if err != nil {
		t.Fatalf("failed to query referred_by: %v", err)
	}
	if referredBy != "user1" {
		t.Errorf("expected referred_by to be 'user1', got %s", referredBy)
	}
}

func TestApplyReferralHandler_SelfReferral(t *testing.T) {
	dbProvider := db.NewTestProvider(t)
	dbProvider.Exec(context.Background(), `
		CREATE TABLE IF NOT EXISTS users (
			id              TEXT PRIMARY KEY,
			username        TEXT UNIQUE NOT NULL,
			email           TEXT UNIQUE NOT NULL,
			password_hash   TEXT NOT NULL DEFAULT '',
			roles           TEXT NOT NULL DEFAULT '{}',
			active          BOOLEAN NOT NULL DEFAULT TRUE,
			organization_id TEXT NOT NULL DEFAULT '',
			oidc_subject    TEXT UNIQUE DEFAULT NULL,
			created_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
			updated_at      TIMESTAMPTZ NOT NULL DEFAULT CURRENT_TIMESTAMP,
			referral_code   TEXT UNIQUE,
			referred_by     TEXT REFERENCES users(id)
		);
	`)
	defer dbProvider.Close()

	ctx := context.Background()

	_, err := dbProvider.Exec(ctx, "INSERT INTO users (id, username, email, referral_code) VALUES ('user1', 'inviter', 'inviter@test.com', 'CODE123')")
	if err != nil {
		t.Fatalf("failed to insert user: %v", err)
	}

	handler := ApplyReferralHandler(dbProvider)

	claims := &auth.Claims{Subject: "user1"}
	reqCtx := context.WithValue(ctx, auth.ClaimsContextKeyForTest, claims)

	reqBody := ApplyReferralRequest{
		UserID:       "user1",
		ReferralCode: "CODE123",
	}
	bodyBytes, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/api/referrals/apply", bytes.NewReader(bodyBytes))
	req = req.WithContext(reqCtx)
	rr := httptest.NewRecorder()

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusBadRequest)
	}
}
