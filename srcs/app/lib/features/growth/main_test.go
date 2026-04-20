package main

import (
	"bytes"
	"context"
	"encoding/json"
	"github.com/onehumancorp/mono/srcs/server/services/growth_legacy"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/alicebob/miniredis/v2"
	"github.com/onehumancorp/mono/srcs/server/lib/analytics"
	"github.com/redis/go-redis/v9"
)

func TestReferralEndpoint(t *testing.T) {
	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, nil)

	req, err := http.NewRequest("POST", "/growth/referral", bytes.NewBuffer([]byte(`{"sender_id":"123","receiver_email":"test@example.com"}`)))
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}
}

func TestGrowthReferralsAPI(t *testing.T) {
	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, nil) // In-memory

	// Test invite
	reqInvite, err := http.NewRequest("POST", "/api/v1/growth/referrals/invite", bytes.NewBuffer([]byte(`{"invitee_email":"new@example.com"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqInvite.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrInvite := httptest.NewRecorder()
	mux.ServeHTTP(rrInvite, reqInvite)

	if status := rrInvite.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	// Test stats
	reqStats, err := http.NewRequest("GET", "/api/v1/growth/referrals/stats", nil)
	if err != nil {
		t.Fatal(err)
	}
	reqStats.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrStats := httptest.NewRecorder()
	mux.ServeHTTP(rrStats, reqStats)

	if status := rrStats.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
	// Test bulk invite
	reqBulkInvite, err := http.NewRequest("POST", "/api/v1/growth/referrals/bulk_invite", bytes.NewBuffer([]byte(`{"invitee_emails":["bulk1@example.com", "bulk2@example.com"]}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqBulkInvite.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrBulkInvite := httptest.NewRecorder()
	mux.ServeHTTP(rrBulkInvite, reqBulkInvite)

	if status := rrBulkInvite.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for bulk_invite: got %v want %v", status, http.StatusOK)
	}
}

func TestReferralAcceptEndpoint(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, rdb)

	// Create an invite first to get a valid referral
	reqInvite, err := http.NewRequest("POST", "/api/v1/growth/referrals/invite", bytes.NewBuffer([]byte(`{"invitee_email":"test@example.com"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqInvite.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrInvite := httptest.NewRecorder()
	mux.ServeHTTP(rrInvite, reqInvite)

	if status := rrInvite.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for invite: got %v want %v", status, http.StatusOK)
	}

	// Now fetch stats to get the referral ID (or just fetch directly if we could parse)
	// We'll just rely on the repo since it's the easiest way to get the generated ID
	keys, err := rdb.Keys(context.Background(), "growth:referral_index:*").Result()
	if err != nil || len(keys) == 0 {
		t.Fatalf("Expected to find referral in redis")
	}
	// Extract the ID from growth:referral_index:ref-12345
	refID := keys[0][len("growth:referral_index:"):]

	// Now accept the invite
	reqAccept, err := http.NewRequest("POST", "/api/v1/growth/referrals/accept", bytes.NewBuffer([]byte(`{"referral_id":"`+refID+`"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqAccept.Header.Set("X-Spiffe-Id", "spiffe://example.org/newuser")

	rrAccept := httptest.NewRecorder()
	mux.ServeHTTP(rrAccept, reqAccept)

	if status := rrAccept.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for accept: got %v want %v", status, http.StatusOK)
	}
}

func TestQuotaCheckEndpoint(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, rdb)

	req, err := http.NewRequest("POST", "/growth/quota/check", bytes.NewBuffer([]byte(`{"tenant_id":"tenant-1"}`)))
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}
}

func TestQuotaIncrementEndpoint(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, rdb)

	req, err := http.NewRequest("POST", "/growth/quota/increment", bytes.NewBuffer([]byte(`{"tenant_id":"tenant-1"}`)))
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v",
			status, http.StatusOK)
	}
}

func TestABTestEndpoints(t *testing.T) {
	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, nil)

	// Test Impression
	reqImp, err := http.NewRequest("POST", "/api/v1/growth/ab_test/impression", bytes.NewBuffer([]byte(`{"experiment_id":"exp-1","variant":"A"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqImp.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrImp := httptest.NewRecorder()
	mux.ServeHTTP(rrImp, reqImp)

	if status := rrImp.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	// Test Conversion
	reqConv, err := http.NewRequest("POST", "/api/v1/growth/ab_test/conversion", bytes.NewBuffer([]byte(`{"experiment_id":"exp-1","variant":"A"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqConv.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrConv := httptest.NewRecorder()
	mux.ServeHTTP(rrConv, reqConv)

	if status := rrConv.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestExportEndpoint(t *testing.T) {
	s, err := miniredis.Run()
	if err != nil {
		t.Fatalf("failed to start miniredis: %v", err)
	}
	defer s.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: s.Addr(),
	})
	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, rdb)

	// Add test data
	repo := growth.NewReferralRepository(rdb)
	repo.SaveReferral(context.Background(), &growth.GrowthReferral{
		ID:           "test-1",
		InviterID:    "user-1",
		InviteeEmail: "test1@ex.com",
		Status:       "SIGNED_UP",
	})
	repo.SaveReferral(context.Background(), &growth.GrowthReferral{
		ID:           "test-2",
		InviterID:    "user-2",
		InviteeEmail: "test2@ex.com",
		Status:       "PENDING",
	})

	req, err := http.NewRequest(http.MethodGet, "/api/v1/growth/analytics/export", nil)
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("X-Spiffe-Id", "admin")

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp map[string]interface{}
	json.Unmarshal(rr.Body.Bytes(), &resp)

	if resp["total_referrals"].(float64) != 2 {
		t.Errorf("expected 2 total referrals, got %v", resp["total_referrals"])
	}
	if resp["total_signups"].(float64) != 1 {
		t.Errorf("expected 1 signup, got %v", resp["total_signups"])
	}
}

func TestTeamEndpoints(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, rdb)

	// Test Send Invite
	reqSend, err := http.NewRequest("POST", "/api/v1/growth/teams/invite", bytes.NewBuffer([]byte(`{"team_id":"team-1","invitee_email":"test@example.com"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqSend.Header.Set("X-Spiffe-Id", "spiffe://example.org/inviter")

	rrSend := httptest.NewRecorder()
	mux.ServeHTTP(rrSend, reqSend)

	if status := rrSend.Code; status != http.StatusOK {
		t.Fatalf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var respSend map[string]string
	json.Unmarshal(rrSend.Body.Bytes(), &respSend)
	inviteID := respSend["invite_id"]
	if inviteID == "" {
		t.Fatalf("Expected invite_id in response")
	}

	// Test Accept Invite
	reqAccept, err := http.NewRequest("POST", "/api/v1/growth/teams/accept", bytes.NewBuffer([]byte(`{"invite_id":"`+inviteID+`"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqAccept.Header.Set("X-Spiffe-Id", "spiffe://example.org/invitee")

	rrAccept := httptest.NewRecorder()
	mux.ServeHTTP(rrAccept, reqAccept)

	if status := rrAccept.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for accept: got %v want %v", status, http.StatusOK)
	}
}

func TestViralCoefficientEndpoint(t *testing.T) {
	mr, err := miniredis.Run()
	if err != nil {
		t.Fatalf("an error '%s' was not expected when opening a stub database connection", err)
	}
	defer mr.Close()

	rdb := redis.NewClient(&redis.Options{
		Addr: mr.Addr(),
	})

	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, rdb)

	req, err := http.NewRequest("GET", "/api/v1/growth/analytics/viral_coefficient", nil)
	if err != nil {
		t.Fatal(err)
	}
	req.Header.Set("X-Spiffe-Id", "admin")

	rr := httptest.NewRecorder()
	mux.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp map[string]float64
	json.Unmarshal(rr.Body.Bytes(), &resp)

	if _, ok := resp["viral_coefficient"]; !ok {
		t.Errorf("expected viral_coefficient in response, got %v", resp)
	}
}
