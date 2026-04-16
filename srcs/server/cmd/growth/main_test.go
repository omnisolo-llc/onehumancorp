package main

import (
"bytes"
"net/http"
"net/http/httptest"
"testing"

"github.com/onehumancorp/mono/srcs/server/lib/analytics"
)

func TestReferralEndpoint(t *testing.T) {
tracker := analytics.NewTracker()
mux := NewGrowthMux(tracker)

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

func TestQuotaCheckEndpoint(t *testing.T) {
tracker := analytics.NewTracker()
mux := NewGrowthMux(tracker)

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
tracker := analytics.NewTracker()
mux := NewGrowthMux(tracker)

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
