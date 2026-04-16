package dashboard

import (
	"bytes"
	"context"
	"encoding/json"
	"log/slog"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/domain"
	"github.com/onehumancorp/mono/srcs/server/orchestration"
	"github.com/onehumancorp/mono/srcs/server/telemetry"
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
		"id":      "mission-pii-1",
		"status":  "PENDING",
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
		"notes":   "ssn 123-45-6789",
	}
	internalBytes, _ := json.Marshal(internalPayload)

	payloads := []map[string]interface{}{
		{
			"id":      "mission-pii-2",
			"status":  "COMPLETED",
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

func TestHandleSyncRAG_PIIRedaction(t *testing.T) {
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
		"memory_id":     "ctx-pii-3",
		"context":       "Context with PII: bob@example.com and phone 999-888-7777.",
		"source_plugin": "test-plugin",
	}
	body, _ := json.Marshal(payload)

	req := httptest.NewRequest(http.MethodPost, "/api/sync/rag", bytes.NewReader(body))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	srv.handleSyncRAG(rr, req)

	if rr.Code != http.StatusOK {
		t.Fatalf("expected 200, got %d", rr.Code)
	}

	// Fetch from SIPDB to verify
	memories, err := db.GetEpisodicMemoriesByPlugin(context.Background(), "test-plugin")
	if err != nil || len(memories) == 0 {
		t.Fatalf("memory not found in sipdb")
	}
	memory := memories[0]

	if bytes.Contains([]byte(memory.Context), []byte("bob@example.com")) {
		t.Errorf("PII email leaked in RAG context")
	}
	if bytes.Contains([]byte(memory.Context), []byte("999-888-7777")) {
		t.Errorf("PII phone leaked in RAG context")
	}
	if !bytes.Contains([]byte(memory.Context), []byte("[REDACTED_EMAIL]")) {
		t.Errorf("Expected [REDACTED_EMAIL] in context")
	}
}

func TestHandleMCPInvoke_PIIRedactionInLogs(t *testing.T) {
	org := domain.NewSoftwareCompany("test-org", "Test Org", "CEO", time.Now())
	hub := orchestration.NewHub()
	defer hub.Close()

	srv := NewServer(org, hub, nil, nil)

	telemetry.Verbosity = 2
	defer func() { telemetry.Verbosity = 1 }()

	// Capture slog output
	var buf bytes.Buffer
	handler := slog.NewTextHandler(&buf, nil)
	originalLogger := slog.Default()
	slog.SetDefault(slog.New(handler))
	defer slog.SetDefault(originalLogger)

	reqBody := `{"toolId": "telegram-mcp", "action": "Contact me at leaking@example.com", "params": {"content": "Hello"}}`
	req := httptest.NewRequest(http.MethodPost, "/api/mcp/invoke", bytes.NewReader([]byte(reqBody)))
	req.Header.Set("Content-Type", "application/json")
	rr := httptest.NewRecorder()

	srv.handleMCPInvoke(rr, req)

	// The handler doesn't necessarily need to succeed, we just want to check the log
	output := buf.String()
	if bytes.Contains([]byte(output), []byte("leaking@example.com")) {
		t.Errorf("PII email leaked in MCP invoke logs: %s", output)
	}
	if !bytes.Contains([]byte(output), []byte("[REDACTED_EMAIL]")) {
		t.Errorf("Expected [REDACTED_EMAIL] in MCP invoke logs: %s", output)
	}
}
