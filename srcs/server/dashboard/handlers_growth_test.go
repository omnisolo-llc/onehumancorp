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

func TestHandleViralCoefficient(t *testing.T) {
	s := &Server{
		referrals: []Referral{
			{UserID: "user-1", Conversions: 2},
			{UserID: "user-1", Conversions: 1},
			{UserID: "user-2", Conversions: 0},
			{UserID: "user-3", Conversions: 3},
		},
	}

	reqGet := httptest.NewRequest(http.MethodGet, "/api/growth/viral-coefficient", nil)
	wGet := httptest.NewRecorder()

	s.handleViralCoefficient(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wGet.Code)
	}

	var res ViralCoefficientResponse
	if err := json.NewDecoder(wGet.Body).Decode(&res); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if res.TotalReferrals != 4 {
		t.Errorf("expected 4 referrals, got %d", res.TotalReferrals)
	}
	// user-1: 3, user-2: 0, user-3: 3 -> Total Conversions = 6
	if res.TotalConversions != 6 {
		t.Errorf("expected 6 conversions, got %d", res.TotalConversions)
	}
	// unique inviters: user-1, user-2, user-3 -> 3
	if res.UniqueInviters != 3 {
		t.Errorf("expected 3 unique inviters, got %d", res.UniqueInviters)
	}
	// KFactor = 6 / 3 = 2.0
	if res.KFactor != 2.0 {
		t.Errorf("expected kFactor 2.0, got %f", res.KFactor)
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

func TestHandleTeamInvites(t *testing.T) {
	s := &Server{}

	// Test POST
	payload := `{"inviterId": "user-123", "email": "test@example.com"}`
	req := httptest.NewRequest(http.MethodPost, "/api/growth/team-invites", bytes.NewBufferString(payload))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	s.handleTeamInvites(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var created TeamInvite
	if err := json.NewDecoder(w.Body).Decode(&created); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if created.InviterID != "user-123" {
		t.Errorf("expected inviterId 'user-123', got '%s'", created.InviterID)
	}
	if created.Email != "test@example.com" {
		t.Errorf("expected email 'test@example.com', got '%s'", created.Email)
	}
	if created.Status != "PENDING" {
		t.Errorf("expected status 'PENDING', got '%s'", created.Status)
	}

	// Test GET
	reqGet := httptest.NewRequest(http.MethodGet, "/api/growth/team-invites", nil)
	wGet := httptest.NewRecorder()

	s.handleTeamInvites(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wGet.Code)
	}

	var list []TeamInvite
	if err := json.NewDecoder(wGet.Body).Decode(&list); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(list) != 1 {
		t.Fatalf("expected 1 invite, got %d", len(list))
	}
	if list[0].ID != created.ID {
		t.Errorf("expected ID %s, got %s", created.ID, list[0].ID)
	}
}

func TestHandleFreeTierQuotas(t *testing.T) {
	s := &Server{}

	// Test GET (Initialize new quota)
	reqGet := httptest.NewRequest(http.MethodGet, "/api/growth/quotas?userId=user-123", nil)
	wGet := httptest.NewRecorder()

	s.handleFreeTierQuotas(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wGet.Code)
	}

	var quota FreeTierQuota
	if err := json.NewDecoder(wGet.Body).Decode(&quota); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if quota.UserID != "user-123" {
		t.Errorf("expected userId 'user-123', got '%s'", quota.UserID)
	}
	if quota.AgentMissions != 0 {
		t.Errorf("expected 0 agent missions, got %d", quota.AgentMissions)
	}
	if quota.MaxMissions != 5 {
		t.Errorf("expected 5 max missions, got %d", quota.MaxMissions)
	}
	if quota.Status != "ACTIVE" {
		t.Errorf("expected status 'ACTIVE', got '%s'", quota.Status)
	}

	// Test POST (Increment usage 5 times to exhaust)
	for i := 1; i <= 5; i++ {
		reqPost := httptest.NewRequest(http.MethodPost, "/api/growth/quotas?userId=user-123", nil)
		wPost := httptest.NewRecorder()
		s.handleFreeTierQuotas(wPost, reqPost)

		if wPost.Code != http.StatusOK {
			t.Fatalf("expected status 200 on post %d, got %d", i, wPost.Code)
		}

		var updatedQuota FreeTierQuota
		if err := json.NewDecoder(wPost.Body).Decode(&updatedQuota); err != nil {
			t.Fatalf("failed to decode response: %v", err)
		}

		if updatedQuota.AgentMissions != i {
			t.Errorf("expected %d agent missions, got %d", i, updatedQuota.AgentMissions)
		}

		expectedStatus := "ACTIVE"
		if i == 5 {
			expectedStatus = "EXHAUSTED"
		}
		if updatedQuota.Status != expectedStatus {
			t.Errorf("expected status '%s', got '%s'", expectedStatus, updatedQuota.Status)
		}
	}
}
