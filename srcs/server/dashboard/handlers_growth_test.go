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

func TestHandleTeamInvites(t *testing.T) {
	s := &Server{}

	// Test POST
	payload := `{"inviterId": "user-A", "inviteeId": "user-B"}`
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

	if created.InviterID != "user-A" {
		t.Errorf("expected inviterId 'user-A', got '%s'", created.InviterID)
	}
	if created.InviteeID != "user-B" {
		t.Errorf("expected inviteeId 'user-B', got '%s'", created.InviteeID)
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
		t.Fatalf("expected 1 team invite, got %d", len(list))
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

func TestHandleDownloads(t *testing.T) {
	s := &Server{}

	// Test POST
	payload := `{"os": "Mac", "version": "1.0.0"}`
	req := httptest.NewRequest(http.MethodPost, "/api/growth/downloads", bytes.NewBufferString(payload))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	s.handleDownloads(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var created Download
	if err := json.NewDecoder(w.Body).Decode(&created); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if created.OS != "Mac" {
		t.Errorf("expected OS 'Mac', got '%s'", created.OS)
	}
	if created.Version != "1.0.0" {
		t.Errorf("expected Version '1.0.0', got '%s'", created.Version)
	}

	// Test GET
	reqGet := httptest.NewRequest(http.MethodGet, "/api/growth/downloads", nil)
	wGet := httptest.NewRecorder()

	s.handleDownloads(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wGet.Code)
	}

	var list []Download
	if err := json.NewDecoder(wGet.Body).Decode(&list); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(list) != 1 {
		t.Fatalf("expected 1 download, got %d", len(list))
	}
	if list[0].ID != created.ID {
		t.Errorf("expected ID %s, got %s", created.ID, list[0].ID)
	}
}

func TestHandleOnboardingFunnel(t *testing.T) {
	s := &Server{}

	// Test POST
	payload := `{"userId": "user-A", "step": "step-1"}`
	req := httptest.NewRequest(http.MethodPost, "/api/growth/onboarding-funnel", bytes.NewBufferString(payload))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	s.handleOnboardingFunnel(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var created OnboardingFunnel
	if err := json.NewDecoder(w.Body).Decode(&created); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if created.UserID != "user-A" || created.Step != "step-1" {
		t.Errorf("unexpected values: %+v", created)
	}

	// Test GET
	reqGet := httptest.NewRequest(http.MethodGet, "/api/growth/onboarding-funnel", nil)
	wGet := httptest.NewRecorder()

	s.handleOnboardingFunnel(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wGet.Code)
	}

	var list []OnboardingFunnel
	if err := json.NewDecoder(wGet.Body).Decode(&list); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(list) != 1 {
		t.Fatalf("expected 1 funnel, got %d", len(list))
	}
	if list[0].ID != created.ID {
		t.Errorf("expected ID %s, got %s", created.ID, list[0].ID)
	}
}

func TestHandleOnboardingMetrics(t *testing.T) {
	s := &Server{
		onboardingFunnels: []OnboardingFunnel{
			{Step: "step1"},
			{Step: "step1"},
			{Step: "step2"},
		},
	}

	req := httptest.NewRequest(http.MethodGet, "/api/growth/onboarding-metrics", nil)
	w := httptest.NewRecorder()

	s.handleOnboardingMetrics(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var metrics []OnboardingMetric
	if err := json.NewDecoder(w.Body).Decode(&metrics); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if len(metrics) != 2 {
		t.Fatalf("expected 2 metric groups, got %d", len(metrics))
	}

	counts := make(map[string]int)
	for _, m := range metrics {
		counts[m.Step] = m.Count
	}

	if counts["step1"] != 2 {
		t.Errorf("expected step1 to have count 2, got %d", counts["step1"])
	}
	if counts["step2"] != 1 {
		t.Errorf("expected step2 to have count 1, got %d", counts["step2"])
	}
}

func TestHandleTeamInviteAccept(t *testing.T) {
	s := &Server{
		teamInvites: []TeamInvite{
			{ID: "inv-1", InviterID: "user-A", InviteeID: "user-B", Status: "PENDING"},
		},
	}

	payload := `{"id": "inv-1"}`
	req := httptest.NewRequest(http.MethodPost, "/api/growth/team-invites/accept", bytes.NewBufferString(payload))
	req.Header.Set("Content-Type", "application/json")
	w := httptest.NewRecorder()

	s.handleTeamInviteAccept(w, req)

	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}

	var updated TeamInvite
	if err := json.NewDecoder(w.Body).Decode(&updated); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if updated.Status != "ACCEPTED" {
		t.Errorf("expected status 'ACCEPTED', got '%s'", updated.Status)
	}

	if s.teamInvites[0].Status != "ACCEPTED" {
		t.Errorf("expected internal state to be 'ACCEPTED'")
	}
}

func TestHandleReferralClickAndConvert(t *testing.T) {
	s := &Server{
		referrals: []Referral{
			{ID: "ref-1", UserID: "user-A", ReferralCode: "CODE123", Clicks: 0, Conversions: 0},
		},
	}

	// Test Click
	payloadClick := `{"id": "ref-1"}`
	reqClick := httptest.NewRequest(http.MethodPost, "/api/growth/referrals/click", bytes.NewBufferString(payloadClick))
	reqClick.Header.Set("Content-Type", "application/json")
	wClick := httptest.NewRecorder()

	s.handleReferralClick(wClick, reqClick)

	if wClick.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wClick.Code)
	}

	var updatedClick Referral
	if err := json.NewDecoder(wClick.Body).Decode(&updatedClick); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if updatedClick.Clicks != 1 {
		t.Errorf("expected 1 click, got %d", updatedClick.Clicks)
	}

	// Test Convert
	payloadConvert := `{"id": "ref-1"}`
	reqConvert := httptest.NewRequest(http.MethodPost, "/api/growth/referrals/convert", bytes.NewBufferString(payloadConvert))
	reqConvert.Header.Set("Content-Type", "application/json")
	wConvert := httptest.NewRecorder()

	s.handleReferralConvert(wConvert, reqConvert)

	if wConvert.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wConvert.Code)
	}

	var updatedConvert Referral
	if err := json.NewDecoder(wConvert.Body).Decode(&updatedConvert); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if updatedConvert.Conversions != 1 {
		t.Errorf("expected 1 conversion, got %d", updatedConvert.Conversions)
	}

	if s.referrals[0].Clicks != 1 || s.referrals[0].Conversions != 1 {
		t.Errorf("expected internal state to be updated")
	}
}

func TestHandleWaitlist(t *testing.T) {
    s := &Server{}

    // Test POST
    payload := `{"email": "test@example.com"}`
    req := httptest.NewRequest(http.MethodPost, "/api/growth/waitlist", bytes.NewBufferString(payload))
    req.Header.Set("Content-Type", "application/json")
    w := httptest.NewRecorder()

    s.handleWaitlist(w, req)

    if w.Result().StatusCode != http.StatusOK {
        t.Errorf("expected 200 OK, got %d", w.Result().StatusCode)
    }
    var created WaitlistEntry
    json.NewDecoder(w.Result().Body).Decode(&created)
    if created.Email != "test@example.com" {
        t.Errorf("expected email test@example.com, got %s", created.Email)
    }

    // Test GET
    reqGET := httptest.NewRequest(http.MethodGet, "/api/growth/waitlist", nil)
    wGET := httptest.NewRecorder()
    s.handleWaitlist(wGET, reqGET)

    if wGET.Result().StatusCode != http.StatusOK {
        t.Errorf("expected 200 OK, got %d", wGET.Result().StatusCode)
    }
    var list []WaitlistEntry
    json.NewDecoder(wGET.Result().Body).Decode(&list)
    if len(list) != 1 {
        t.Errorf("expected 1 waitlist entry, got %d", len(list))
    }
}

func TestHandleViralCoefficientMetrics(t *testing.T) {
	server := &Server{}

	// Add mock referrals
	server.referrals = []Referral{
		{UserID: "user1", Conversions: 2},
		{UserID: "user2", Conversions: 4},
	}

	req, err := http.NewRequest("GET", "/api/growth/viral-coefficient-metrics", nil)
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler := http.HandlerFunc(server.handleViralCoefficientMetrics)

	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}

	var resp map[string]interface{}
	json.Unmarshal(rr.Body.Bytes(), &resp)
	if resp["viral_coefficient"] != float64(3.0) { t.Errorf("expected 3.0, got %v", resp["viral_coefficient"]) }
	if resp["organization_id"] != "default" { t.Errorf("expected default, got %v", resp["organization_id"]) }
}

func TestHandleQuota(t *testing.T) {
	s := &Server{
		referrals: []Referral{
			{UserID: "user1", Conversions: 2},
			{UserID: "user2", Conversions: 1},
		},
	}
	req := httptest.NewRequest(http.MethodGet, "/api/growth/quota?userId=user1", nil)
	w := httptest.NewRecorder()
	s.handleQuota(w, req)
	if w.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", w.Code)
	}
	var metrics QuotaMetrics
	if err := json.NewDecoder(w.Body).Decode(&metrics); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}
	// user1 has 2 conversions: Base 100 + (2 * 50) = 200
	if metrics.Used != 10 || metrics.Max != 200 {
		t.Errorf("expected 10/200, got %d/%d", metrics.Used, metrics.Max)
	}
}
