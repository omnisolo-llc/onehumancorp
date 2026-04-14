package dashboard

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
)

func TestHandleSovereignToCloudInvite(t *testing.T) {
	s := NewServer(":8080")
	s.RegisterRoutes()

	// 1. Success case
	reqBody := map[string]string{
		"inviter":  "local_user_1",
		"asset_id": "market_audit_7",
	}
	b, _ := json.Marshal(reqBody)
	req := httptest.NewRequest(http.MethodPost, "/api/growth/viral-bridge", bytes.NewReader(b))
	rec := httptest.NewRecorder()

	s.handleSovereignToCloudInvite(rec, req)

	if rec.Code != http.StatusAccepted {
		t.Errorf("Expected status 202, got %d", rec.Code)
	}

	// 2. Missing fields
	reqBody2 := map[string]string{
		"inviter": "local_user_1",
	}
	b2, _ := json.Marshal(reqBody2)
	req2 := httptest.NewRequest(http.MethodPost, "/api/growth/viral-bridge", bytes.NewReader(b2))
	rec2 := httptest.NewRecorder()

	s.handleSovereignToCloudInvite(rec2, req2)

	if rec2.Code != http.StatusBadRequest {
		t.Errorf("Expected status 400 for missing fields, got %d", rec2.Code)
	}

	// 3. Wrong method
	req3 := httptest.NewRequest(http.MethodGet, "/api/growth/viral-bridge", nil)
	rec3 := httptest.NewRecorder()
	s.handleSovereignToCloudInvite(rec3, req3)

	if rec3.Code != http.StatusMethodNotAllowed {
		t.Errorf("Expected status 405, got %d", rec3.Code)
	}
}
