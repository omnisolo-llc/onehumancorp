1. **Define Team Referral Struct & Implement API Endpoint:** Use `run_in_bash_session` to append the struct and endpoint to `srcs/server/dashboard/handlers_growth.go`.
```bash
cat << 'CODE_EOF' >> srcs/server/dashboard/handlers_growth.go

// TeamReferralAnalyticsResponse represents the computed K-factor for team-based growth.
type TeamReferralAnalyticsResponse struct {
	TotalTeamReferrals   int     `json:"totalTeamReferrals"`
	TotalTeamConversions int     `json:"totalTeamConversions"`
	UniqueTeamInviters   int     `json:"uniqueTeamInviters"`
	TeamKFactor          float64 `json:"teamKFactor"`
}

func (s *Server) handleTeamReferralAnalytics(w http.ResponseWriter, r *http.Request) {
	if r.Method != http.MethodGet {
		http.Error(w, "method not allowed", http.StatusMethodNotAllowed)
		return
	}

	s.mu.RLock()
	refs := append([]Referral(nil), s.referrals...)
	s.mu.RUnlock()

	var totalConversions int
	var totalReferrals int
	inviters := make(map[string]bool)

	for _, ref := range refs {
		// Filter for team referrals
		if len(ref.ReferralCode) >= 5 && ref.ReferralCode[:5] == "TEAM-" {
			totalReferrals++
			totalConversions += ref.Conversions
			inviters[ref.UserID] = true
		}
	}

	uniqueInviters := len(inviters)
	kFactor := 0.0
	if uniqueInviters > 0 {
		kFactor = float64(totalConversions) / float64(uniqueInviters)
	}

	res := TeamReferralAnalyticsResponse{
		TotalTeamReferrals:   totalReferrals,
		TotalTeamConversions: totalConversions,
		UniqueTeamInviters:   uniqueInviters,
		TeamKFactor:          kFactor,
	}
	writeJSON(w, res)
}
CODE_EOF
tail -n 45 srcs/server/dashboard/handlers_growth.go
```

2. **Register Endpoint:** Use `run_in_bash_session` to inject the route in `srcs/server/dashboard/server.go`.
```bash
sed -i '/mux.HandleFunc("\/api\/growth\/viral-coefficient", server.handleViralCoefficient)/a \	mux.HandleFunc("/api/growth/team-analytics", server.handleTeamReferralAnalytics)' srcs/server/dashboard/server.go
grep -C 2 "team-analytics" srcs/server/dashboard/server.go
```

3. **Add Unit Tests:** Use `run_in_bash_session` to append tests to `srcs/server/dashboard/handlers_growth_test.go`.
```bash
cat << 'TEST_EOF' >> srcs/server/dashboard/handlers_growth_test.go

func TestHandleTeamReferralAnalytics(t *testing.T) {
	s := &Server{
		referrals: []Referral{
			{UserID: "user-1", ReferralCode: "TEAM-ABC", Conversions: 2},
			{UserID: "user-1", ReferralCode: "TEAM-DEF", Conversions: 1},
			{UserID: "user-2", ReferralCode: "TEAM-GHI", Conversions: 0},
			{UserID: "user-3", ReferralCode: "INDIV-123", Conversions: 3}, // Non-team
		},
	}

	reqGet := httptest.NewRequest(http.MethodGet, "/api/growth/team-analytics", nil)
	wGet := httptest.NewRecorder()

	s.handleTeamReferralAnalytics(wGet, reqGet)

	if wGet.Code != http.StatusOK {
		t.Fatalf("expected status 200, got %d", wGet.Code)
	}

	var res TeamReferralAnalyticsResponse
	if err := json.NewDecoder(wGet.Body).Decode(&res); err != nil {
		t.Fatalf("failed to decode response: %v", err)
	}

	if res.TotalTeamReferrals != 3 {
		t.Errorf("expected 3 team referrals, got %d", res.TotalTeamReferrals)
	}
	// user-1: 3, user-2: 0 -> Total Conversions = 3 (user-3 ignored)
	if res.TotalTeamConversions != 3 {
		t.Errorf("expected 3 team conversions, got %d", res.TotalTeamConversions)
	}
	// unique team inviters: user-1, user-2 -> 2
	if res.UniqueTeamInviters != 2 {
		t.Errorf("expected 2 unique team inviters, got %d", res.UniqueTeamInviters)
	}
	// KFactor = 3 / 2 = 1.5
	if res.TeamKFactor != 1.5 {
		t.Errorf("expected team kFactor 1.5, got %f", res.TeamKFactor)
	}
}
TEST_EOF
tail -n 45 srcs/server/dashboard/handlers_growth_test.go
```

4. **Run tests:** Use `run_in_bash_session` to execute `bazelisk test //srcs/server/dashboard/...`

Complete pre-commit steps to ensure proper testing, verification, review, and reflection are done.
