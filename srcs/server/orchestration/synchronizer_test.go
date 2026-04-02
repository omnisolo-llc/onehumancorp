package orchestration

import (
	"context"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestSwarmSynchronizer_ProcessSyncTick(t *testing.T) {
	t.Setenv("DATABASE_URL", "sqlite://file::memory:?mode=memory")
	ctx := context.Background()

	// Initialize test database
	dbWrapper, err := db.New(ctx)
	if err != nil {
		t.Fatalf("Failed to initialize database: %v", err)
	}
	defer dbWrapper.Close()

	// Create necessary tables
	_, err = dbWrapper.Exec(ctx, `
		CREATE TABLE swarm_memory (
			key TEXT PRIMARY KEY,
			value TEXT NOT NULL,
			updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
		);
		CREATE TABLE local_cloud_sync_log (
			sync_id TEXT PRIMARY KEY,
			memory_id TEXT NOT NULL,
			cloud_mission_id TEXT,
			synced_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
			FOREIGN KEY (memory_id) REFERENCES swarm_memory(key) ON DELETE CASCADE
		);
	`)
	if err != nil {
		t.Fatalf("Failed to create tables: %v", err)
	}

	// Insert test data
	rawJSON := `{"id":"123", "secret":"super_secret_password", "info":"safe_data"}`
	_, err = dbWrapper.Exec(ctx, "INSERT INTO swarm_memory (key, value) VALUES ($1, $2)", "mem-1", rawJSON)
	if err != nil {
		t.Fatalf("Failed to insert test data: %v", err)
	}

	// Mock cloud API
	var receivedPayload MemoryPayload
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/sync/autodream" {
			json.NewDecoder(r.Body).Decode(&receivedPayload)
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(CloudResponse{MissionID: "cloud-mission-999"})
			return
		}
		w.WriteHeader(http.StatusNotFound)
	}))
	defer ts.Close()

	// Create synchronizer
	sync := NewSwarmSynchronizer(dbWrapper, ts.Client(), ts.URL)

	// Run process sync tick
	sync.ProcessSyncTick(ctx)

	// Validate API call
	if receivedPayload.MemoryID != "mem-1" {
		t.Errorf("Expected MemoryID 'mem-1', got '%s'", receivedPayload.MemoryID)
	}

	// Validate sanitization
	var payloadData map[string]interface{}
	json.Unmarshal([]byte(receivedPayload.Context), &payloadData)
	if payloadData["secret"] != "[REDACTED]" {
		t.Errorf("Expected secret to be redacted, got '%v'", payloadData["secret"])
	}
	if payloadData["info"] != "safe_data" {
		t.Errorf("Expected info to be safe_data, got '%v'", payloadData["info"])
	}

	// Validate database state updated (sync logged)
	var syncCount int
	err = dbWrapper.QueryRow(ctx, "SELECT COUNT(*) FROM local_cloud_sync_log").Scan(&syncCount)
	if err != nil {
		t.Fatalf("Failed to query sync log count: %v", err)
	}
	if syncCount != 1 {
		t.Errorf("Expected 1 sync log entry, got %d", syncCount)
	}

	// Run tick again, should not sync again
	sync.ProcessSyncTick(ctx)

	err = dbWrapper.QueryRow(ctx, "SELECT COUNT(*) FROM local_cloud_sync_log").Scan(&syncCount)
	if err != nil {
		t.Fatalf("Failed to query sync log count: %v", err)
	}
	if syncCount != 1 {
		t.Errorf("Expected 1 sync log entry, got %d", syncCount)
	}
}

func TestSwarmSynchronizer_SanitizeContext(t *testing.T) {
	sync := &SwarmSynchronizer{}
	rawJSON := `{"token":"12345", "password":"abcd", "public":"yes"}`
	sanitized := sync.sanitizeContext(rawJSON)

	var result map[string]interface{}
	json.Unmarshal([]byte(sanitized), &result)

	if result["token"] != "[REDACTED]" {
		t.Errorf("Expected token to be redacted")
	}
	if result["password"] != "[REDACTED]" {
		t.Errorf("Expected password to be redacted")
	}
	if result["public"] != "yes" {
		t.Errorf("Expected public to be 'yes'")
	}

	// Test fallback for non-JSON strings
	rawString := "This is a long string "
	for i := 0; i < 100; i++ {
		rawString += "very long "
	}
	sanitizedString := sync.sanitizeContext(rawString)
	if len(sanitizedString) > 1000 {
		t.Errorf("Expected string to be truncated to 1000 chars")
	}
}
