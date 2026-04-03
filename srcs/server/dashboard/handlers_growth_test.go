package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"

)

func TestHandleLandingPageExperiments(t *testing.T) {
	s := &Server{}

	// Test POST
	payload := `{"title": "Test Experiment", "trafficSplit": 0.5}`
	req := httptest.NewRequest(http.MethodPost, "/api/growth/experiments", bytes.NewBufferString(payload))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	s.handleLandingPageExperiments(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var created LandingPageExperiment
	if err := json.NewDecoder(w.Body).Decode(&created); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if created.Title != "Test Experiment" {
		t.Errorf("expected title 'Test Experiment', got '%s'", created.Title)
	}
	if created.TrafficSplit != 0.5 {
		t.Errorf("expected trafficSplit 0.5, got %f", created.TrafficSplit)
	}
	if created.Status != "ACTIVE" {
		t.Errorf("expected status 'ACTIVE', got '%s'", created.Status)
	}

	// Test GET
	reqGet := httptest.NewRequest(http.MethodGet, "/api/growth/experiments", nil)
	wGet := httptest.NewRecorder()

	s.handleLandingPageExperiments(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wGet.Code)
	}

	var list []LandingPageExperiment
	if err := json.NewDecoder(wGet.Body).Decode(&list); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(list) != 1 {
		t.Fatalf("expected 1 experiment, got %d", len(list))
	}
	if list[0].ID != created.ID {
		t.Errorf("expected ID %s, got %s", created.ID, list[0].ID)
	}
}

func TestHandleReferrals(t *testing.T) {
	s := &Server{}

	// Test POST
	payload := `{"userId": "user-123", "referralCode": "GROWTH2026"}`
	req := httptest.NewRequest(http.MethodPost, "/api/growth/referrals", bytes.NewBufferString(payload))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	s.handleReferrals(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var created Referral
	if err := json.NewDecoder(w.Body).Decode(&created); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if created.UserID != "user-123" {
		t.Errorf("expected userId 'user-123', got '%s'", created.UserID)
	}
	if created.ReferralCode != "GROWTH2026" {
		t.Errorf("expected referralCode 'GROWTH2026', got '%s'", created.ReferralCode)
	}
	if created.Clicks != 0 {
		t.Errorf("expected 0 clicks, got %d", created.Clicks)
	}
	if created.Conversions != 0 {
		t.Errorf("expected 0 conversions, got %d", created.Conversions)
	}

	// Test GET
	reqGet := httptest.NewRequest(http.MethodGet, "/api/growth/referrals", nil)
	wGet := httptest.NewRecorder()

	s.handleReferrals(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wGet.Code)
	}

	var list []Referral
	if err := json.NewDecoder(wGet.Body).Decode(&list); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(list) != 1 {
		t.Fatalf("expected 1 referral, got %d", len(list))
	}
	if list[0].ID != created.ID {
		t.Errorf("expected ID %s, got %s", created.ID, list[0].ID)
	}
}
