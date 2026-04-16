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

	// Test accept invite
	reqAccept, err := http.NewRequest("POST", "/api/v1/growth/referrals/accept", bytes.NewBuffer([]byte(`{"invite_id":"ref-12345"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqAccept.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrAccept := httptest.NewRecorder()
	mux.ServeHTTP(rrAccept, reqAccept)

	if status := rrAccept.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}
}

func TestABTestingEndpoints(t *testing.T) {
	tracker := analytics.NewTracker()
	mux := NewGrowthMux(tracker, nil)

	// Test impression
	reqImpression, err := http.NewRequest("POST", "/api/v1/growth/ab/impression", bytes.NewBuffer([]byte(`{"experiment_id":"exp-1","variant":"A"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqImpression.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrImpression := httptest.NewRecorder()
	mux.ServeHTTP(rrImpression, reqImpression)

	if status := rrImpression.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	// Test conversion
	reqConversion, err := http.NewRequest("POST", "/api/v1/growth/ab/conversion", bytes.NewBuffer([]byte(`{"experiment_id":"exp-1","variant":"B"}`)))
	if err != nil {
		t.Fatal(err)
	}
	reqConversion.Header.Set("X-Spiffe-Id", "spiffe://example.org/myservice")

	rrConversion := httptest.NewRecorder()
	mux.ServeHTTP(rrConversion, reqConversion)

	if status := rrConversion.Code; status != http.StatusOK {
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
