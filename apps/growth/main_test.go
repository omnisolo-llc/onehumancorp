package main

import (
	"bytes"
	"net/http"
	"net/http/httptest"
	"testing"
	"encoding/json"

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

func TestReferralsAcceptAPI(t *testing.T) {
	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, nil) // In-memory

	// 1. Create a referral
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

	// 2. Accept the referral (happy path)
	// We need to know the ID, but the invite endpoint currently does not return it in a predictable way for the test without parsing JSON
	var respData map[string]string
	json.NewDecoder(rrInvite.Body).Decode(&respData)
	link := respData["link"] // ohc://join?ref=ID
	id := link[15:] // strip "ohc://join?ref="

	reqAcceptSuccess, err := http.NewRequest("POST", "/api/v1/growth/referrals/accept", bytes.NewBuffer([]byte(`{"invite_id":"` + id + `"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqAcceptSuccess.Header.Set("X-Spiffe-Id", "spiffe://example.org/newuser")

	rrAcceptSuccess := httptest.NewRecorder()
	mux.ServeHTTP(rrAcceptSuccess, reqAcceptSuccess)

	if status := rrAcceptSuccess.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for valid accept: got %v want %v", status, http.StatusOK)
	}

	// Test with invalid ID
	reqAcceptFail, err := http.NewRequest("POST", "/api/v1/growth/referrals/accept", bytes.NewBuffer([]byte(`{"invite_id":"fake-id"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqAcceptFail.Header.Set("X-Spiffe-Id", "spiffe://example.org/newuser")

	rrAcceptFail := httptest.NewRecorder()
	mux.ServeHTTP(rrAcceptFail, reqAcceptFail)

	if status := rrAcceptFail.Code; status != http.StatusBadRequest {
		t.Errorf("handler returned wrong status code for invalid accept: got %v want %v", status, http.StatusBadRequest)
	}
}
