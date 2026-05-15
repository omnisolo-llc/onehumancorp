package growth

import (
	"bytes"
	"context"
	"database/sql"
	"net/http"
	"net/http/httptest"
	"onehumancorp/srcs/server/onboarding"
	"testing"
	"time"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("Failed to open DB: %v", err)
	}
	db.Exec("CREATE TABLE IF NOT EXISTS organizations (id TEXT PRIMARY KEY, plan_tier TEXT, pro_until DATETIME)")
	db.Exec("CREATE TABLE IF NOT EXISTS referrals (id TEXT PRIMARY KEY, inviter_id TEXT, invitee_id TEXT, clicks INT, conversions INT)")
	return db
}

func TestGrowthService_ReferralFlow(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewGrowthService(db)

	db.Exec("INSERT INTO organizations (id, plan_tier) VALUES ('inviter1', 'Free')")
	db.Exec("INSERT INTO organizations (id, plan_tier) VALUES ('invitee1', 'Free')")
	db.Exec("INSERT INTO referrals (id, inviter_id, clicks, conversions) VALUES ('ref1', 'inviter1', 0, 0)")

	t.Run("HandleReferralClick", func(t *testing.T) {
		body := `{"id": "ref1"}`
		req := httptest.NewRequest(http.MethodPost, "/click", bytes.NewBufferString(body))
		req = req.WithContext(context.WithValue(req.Context(), onboarding.TenantContextKey, "invitee1"))
		w := httptest.NewRecorder()
		svc.HandleReferralClick(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("Expected 200, got %d", w.Code)
		}
	})

	t.Run("HandleReferralConvert", func(t *testing.T) {
		body := `{"id": "ref1", "invitee_id": "invitee1"}`
		req := httptest.NewRequest(http.MethodPost, "/convert", bytes.NewBufferString(body))
		req = req.WithContext(context.WithValue(req.Context(), onboarding.TenantContextKey, "invitee1"))
		w := httptest.NewRecorder()
		svc.HandleReferralConvert(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("Expected 200, got %d", w.Code)
		}

		var inviterPlan, inviteePlan string
		var inviterProUntil, inviteeProUntil sql.NullTime
		db.QueryRow("SELECT plan_tier, pro_until FROM organizations WHERE id = 'inviter1'").Scan(&inviterPlan, &inviterProUntil)
		db.QueryRow("SELECT plan_tier, pro_until FROM organizations WHERE id = 'invitee1'").Scan(&inviteePlan, &inviteeProUntil)

		if inviterPlan != "Pro" || inviteePlan != "Pro" {
			t.Errorf("Expected both to be Pro, got %s and %s", inviterPlan, inviteePlan)
		}

		expectedDate := time.Now().AddDate(0, 1, 0)
		if !inviterProUntil.Valid || inviterProUntil.Time.Before(expectedDate.Add(-1*time.Minute)) || inviterProUntil.Time.After(expectedDate.Add(1*time.Minute)) {
			t.Errorf("Inviter pro_until not valid or not set to 1 month from now: %v", inviterProUntil)
		}
		if !inviteeProUntil.Valid || inviteeProUntil.Time.Before(expectedDate.Add(-1*time.Minute)) || inviteeProUntil.Time.After(expectedDate.Add(1*time.Minute)) {
			t.Errorf("Invitee pro_until not valid or not set to 1 month from now: %v", inviteeProUntil)
		}
	})

	t.Run("HandleTeamInviteAccept", func(t *testing.T) {
		body := `{"id": "inv1"}`
		req := httptest.NewRequest(http.MethodPost, "/accept", bytes.NewBufferString(body))
		req = req.WithContext(context.WithValue(req.Context(), onboarding.TenantContextKey, "invitee1"))
		w := httptest.NewRecorder()
		svc.HandleTeamInviteAccept(w, req)
		if w.Code != http.StatusOK {
			t.Errorf("Expected 200, got %d", w.Code)
		}
	})
}
