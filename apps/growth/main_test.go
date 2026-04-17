package main

import (
	"bytes"
	"context"
	"net/http"
	"net/http/httptest"
	"testing"

	"github.com/alicebob/miniredis/v2"
	"github.com/onehumancorp/mono/lib/analytics"
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

func TestTeamInviteEndpoints(t *testing.T) {
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

	// Test Team Invite
	reqInvite, err := http.NewRequest("POST", "/api/v1/growth/team/invite", bytes.NewBuffer([]byte(`{"tenant_id":"tenant-1","invitee_email":"test@example.com"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqInvite.Header.Set("X-Spiffe-Id", "tenant-1")

	rrInvite := httptest.NewRecorder()
	mux.ServeHTTP(rrInvite, reqInvite)

	if status := rrInvite.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for team invite: got %v want %v", status, http.StatusOK)
	}

	// Fetch invites by tenant to get ID
	reqList, err := http.NewRequest("GET", "/api/v1/growth/team/invites?tenant_id=tenant-1", nil)
	if err != nil {
		t.Fatal(err)
	}
	reqList.Header.Set("X-Spiffe-Id", "tenant-1")

	rrList := httptest.NewRecorder()
	mux.ServeHTTP(rrList, reqList)

	if status := rrList.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for team list: got %v want %v", status, http.StatusOK)
	}

	// Accept the invite
	keys, err := rdb.Keys(context.Background(), "growth:team_invite_index:*").Result()
	if err != nil || len(keys) == 0 {
		t.Fatalf("Expected to find team invite in redis")
	}
	inviteID := keys[0][len("growth:team_invite_index:"):]

	reqAccept, err := http.NewRequest("POST", "/api/v1/growth/team/accept", bytes.NewBuffer([]byte(`{"invite_id":"`+inviteID+`", "user_email": "test@example.com"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqAccept.Header.Set("X-Spiffe-Id", "spiffe://example.org/newuser")

	rrAccept := httptest.NewRecorder()
	mux.ServeHTTP(rrAccept, reqAccept)

	if status := rrAccept.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for team accept: got %v want %v", status, http.StatusOK)
	}
}
