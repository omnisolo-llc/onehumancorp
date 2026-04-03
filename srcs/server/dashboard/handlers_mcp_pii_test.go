package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
)

func TestHandleMissionsSync_PIIRedaction(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()

	// Create mock SIPDB
	db, err := orchestration.NewSIPDB("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to init sipdb: %v", err)
	}
	hub.SetSIPDB(db)

	srv := NewServer(org, hub, nil, nil)

	payload := map[string]interface{}{
		"id": "mission-pii-1",
		"status": "PENDING",
		"details": "My email is user@example.com and phone is 555-123-4567.",
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest(http.MethodPost, "/api/missions/sync", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	srv.handleMissionsSync(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rr.Code)
	}

	// Fetch from SIPDB to verify
	mission, ok, err := db.GetMission(context.Background(), "mission-pii-1")
	if err != nil || !ok {
		t.Fatalf("mission not found in sipdb")
	}

	if bytes.Contains([]byte(mission.Payload), []byte("user@example.com")) {
		t.Errorf("PII email leaked in mission payload")
	}
	if bytes.Contains([]byte(mission.Payload), []byte("555-123-4567")) {
		t.Errorf("PII phone leaked in mission payload")
	}
	if !bytes.Contains([]byte(mission.Payload), []byte("[REDACTED_EMAIL]")) {
		t.Errorf("Expected [REDACTED_EMAIL] in payload")
	}
}

func TestHandleHybridSyncMissions_PIIRedaction(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()

	// Create mock SIPDB
	db, err := orchestration.NewSIPDB("file::memory:?cache=shared")
	if err != nil {
		t.Fatalf("failed to init sipdb: %v", err)
	}
	hub.SetSIPDB(db)

	srv := NewServer(org, hub, nil, nil)

	internalPayload := map[string]interface{}{
		"contact": "alice@acme.com",
		"notes": "ssn 123-45-6789",
	}
	internalBytes, _ := json.Marshal(internalPayload)

	payloads := []map[string]interface{}{
		{
			"id": "mission-pii-2",
			"status": "COMPLETED",
			"payload": string(internalBytes),
		},
	}
	body, _ := json.Marshal(payloads)

	req := httptest.NewRequest(http.MethodPost, "/api/sync/missions", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	srv.handleHybridSyncMissions(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rr.Code)
	}

	// Fetch from SIPDB
	mission, ok, err := db.GetMission(context.Background(), "mission-pii-2")
	if err != nil || !ok {
		t.Fatalf("mission not found in sipdb")
	}

	if bytes.Contains([]byte(mission.Payload), []byte("alice@acme.com")) {
		t.Errorf("PII email leaked in mission payload")
	}
	if bytes.Contains([]byte(mission.Payload), []byte("123-45-6789")) {
		t.Errorf("PII SSN leaked in mission payload")
	}
	if !bytes.Contains([]byte(mission.Payload), []byte("[REDACTED_EMAIL]")) {
		t.Errorf("Expected [REDACTED_EMAIL] in payload")
	}
}
