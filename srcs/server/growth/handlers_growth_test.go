package growth

import (
	"bytes"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

	_ "github.com/mattn/go-sqlite3"
)

func setupTestDB(t *testing.T) *sql.DB {
	db, err := sql.Open("sqlite3", ":memory:")
	if err != nil {
		t.Fatalf("failed to open test db: %v", err)
	}
	return db
}

func TestHandleReferralClick(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewGrowthService(db)

	reqBody := `{"id": "ref-123"}`
	req, err := http.NewRequest("POST", "/api/growth/referrals/click", bytes.NewBufferString(reqBody))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(svc.HandleReferralClick)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp Referral
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if resp.ID != "ref-123" {
		t.Errorf("expected ID 'ref-123', got '%s'", resp.ID)
	}
	if resp.Clicks != 1 {
		t.Errorf("expected Clicks 1, got %d", resp.Clicks)
	}
}

func TestHandleReferralConvert(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewGrowthService(db)

	reqBody := `{"id": "ref-456"}`
	req, err := http.NewRequest("POST", "/api/growth/referrals/convert", bytes.NewBufferString(reqBody))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(svc.HandleReferralConvert)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp Referral
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if resp.ID != "ref-456" {
		t.Errorf("expected ID 'ref-456', got '%s'", resp.ID)
	}
	if resp.Conversions != 1 {
		t.Errorf("expected Conversions 1, got %d", resp.Conversions)
	}
}

func TestHandleTeamInviteAccept(t *testing.T) {
	db := setupTestDB(t)
	defer db.Close()
	svc := NewGrowthService(db)

	reqBody := `{"id": "inv-789"}`
	req, err := http.NewRequest("POST", "/api/growth/team-invites/accept", bytes.NewBufferString(reqBody))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(svc.HandleTeamInviteAccept)
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp TeamInvite
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if resp.ID != "inv-789" {
		t.Errorf("expected ID 'inv-789', got '%s'", resp.ID)
	}
	if resp.Status != "ACCEPTED" {
		t.Errorf("expected Status 'ACCEPTED', got '%s'", resp.Status)
	}
}
