package main

import (
	"bytes"
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

func TestTeamInviteEndpoints(t *testing.T) {
	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, nil)

	reqSend, err := http.NewRequest("POST", "/growth/team_invite/send", bytes.NewBuffer([]byte(`{"tenant_id":"t1","sender_id":"s1","receiver_email":"r1@example.com"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqSend.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrSend := httptest.NewRecorder()
	mux.ServeHTTP(rrSend, reqSend)

	if status := rrSend.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code for send: got %v want %v", status, http.StatusOK)
	}

	reqAccept, err := http.NewRequest("POST", "/growth/team_invite/accept", bytes.NewBuffer([]byte(`{"tenant_id":"t1","invite_id":"i1"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqAccept.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

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
