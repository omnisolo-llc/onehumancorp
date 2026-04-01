package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"path/filepath"
	"strings"
	"testing"
)

func TestSyncMissions_SuccessAndConflict(t *testing.T) {
	tempDir := t.TempDir()
	dbPath := filepath.Join(tempDir, "test_sync.db")

	sipdb, err := NewSIPDB(dbPath)
	if err != nil {
		t.Fatalf("Failed to create provider: %v", err)
	}

	rawDB := sipdb.db

	// Create pending missions
	_, err = rawDB.ExecContext(context.Background(), "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('m1', 'PENDING', '{\"role\":\"test\",\"rag_context\":\"secret\",\"data\":\"value1\"}', CURRENT_TIMESTAMP)")
	if err != nil {
		t.Fatalf("Failed to insert mission: %v", err)
	}
	_, err = rawDB.ExecContext(context.Background(), "INSERT INTO agent_missions (id, status, payload, created_at) VALUES ('m2', 'PENDING', '{\"role\":\"test2\",\"data\":\"value2\"}', CURRENT_TIMESTAMP)")
	if err != nil {
		t.Fatalf("Failed to insert mission: %v", err)
	}

	var payloadsReceived []string
	var headersReceived []http.Header

	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		headersReceived = append(headersReceived, r.Header)

		var payload map[string]interface{}
		json.NewDecoder(r.Body).Decode(&payload)
		b, _ := json.Marshal(payload)
		payloadsReceived = append(payloadsReceived, string(b))

		if payload["role"] == "test" {
			w.WriteHeader(http.StatusConflict) // 409 Conflict should be treated as success
		} else {
			w.WriteHeader(http.StatusOK)
		}
	}))
	defer ts.Close()

	count, err := sipdb.SyncMissions(context.Background(), ts.URL)
	if err != nil {
		t.Fatalf("SyncMissions returned error: %v", err)
	}
	if count != 2 {
		t.Errorf("Expected 2 synced missions, got %d", count)
	}

	if len(payloadsReceived) != 2 {
		t.Fatalf("Expected 2 payloads received, got %d", len(payloadsReceived))
	}

	for _, p := range payloadsReceived {
		if strings.Contains(p, "rag_context") || strings.Contains(p, "secret") {
			t.Errorf("Payload should not contain 'rag_context' or 'secret', got %s", p)
		}
	}

	for _, h := range headersReceived {
		if h.Get("X-Conflict-Resolution") != "client-wins" {
			t.Errorf("Expected X-Conflict-Resolution header to be 'client-wins', got %s", h.Get("X-Conflict-Resolution"))
		}
	}

	var status string
	err = rawDB.QueryRowContext(context.Background(), "SELECT status FROM agent_missions WHERE id = 'm1'").Scan(&status)
	if err != nil || status != "SYNCED" {
		t.Errorf("Expected status SYNCED for m1, got %s, err %v", status, err)
	}

	err = rawDB.QueryRowContext(context.Background(), "SELECT status FROM agent_missions WHERE id = 'm2'").Scan(&status)
	if err != nil || status != "SYNCED" {
		t.Errorf("Expected status SYNCED for m2, got %s, err %v", status, err)
	}
}
