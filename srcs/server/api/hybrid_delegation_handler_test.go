package api

import (
	"bytes"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleHybridDelegation(t *testing.T) {
	hub := orchestration.NewHub()

	// Create an in-memory SIPDB
	sipDB, err := orchestration.NewSIPDB(":memory:")
	if err != nil {
		t.Fatalf("Failed to create in-memory SIPDB: %v", err)
	}
	hub.SetSIPDB(sipDB)

	handler := HandleHybridDelegation(hub)

	payload := DelegationRequest{
		OriginalQuery: "deep research topic",
		Content:       "sanitized local knowledge",
	}
	body, _ := json.Marshal(payload)

	req, err := http.NewRequest("POST", "/api/hybrid/delegate", bytes.NewBuffer(body))
	if err != nil {
		t.Fatal(err)
	}

	rr := httptest.NewRecorder()
	handler.ServeHTTP(rr, req)

	if status := rr.Code; status != http.StatusOK {
		t.Errorf("handler returned wrong status code: got %v want %v", status, http.StatusOK)
	}

	var resp DelegationResponse
	if err := json.NewDecoder(rr.Body).Decode(&resp); err != nil {
		t.Fatal(err)
	}

	if resp.Status != "success" {
		t.Errorf("expected status 'success', got '%s'", resp.Status)
	}
	if resp.MissionID == "" {
		t.Error("expected non-empty mission_id")
	}
}
