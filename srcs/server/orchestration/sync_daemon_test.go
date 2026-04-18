package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"time"

	"github.com/onehumancorp/mono/srcs/server/db"
	_ "modernc.org/sqlite"
)

func TestHybridMCPRAGDaemon_ProcessSync(t *testing.T) {
	defer ClearSemaphore()
	// Setup SQLite in-memory db
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT false,
            cloud_mission_id TEXT,
            sync_error TEXT,
            last_synced_at TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO agent_missions (id, status, payload, synced_to_cloud, cloud_mission_id, sync_error, last_synced_at)
		VALUES
			('m1', 'PENDING', '{"task":"test-mission", "details":"[PRIVATE:secret] email is a@b.com"}', false, NULL, NULL, NULL),
			('m2', 'COMPLETED', '{"task":"synced-mission"}', true, NULL, NULL, NULL),
			('m3', 'IGNORED', '{"task":"ignored"}', false, NULL, NULL, NULL)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedPayload struct {
		LocalID string `json:"local_id"`
		Payload map[string]interface{} `json:"payload"`
	}
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/v1/missions/escalate" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedPayload); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(CloudSyncResponse{Status: "ACCEPTED", CloudID: "cloud-m1"})
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Process sync manually for testing
	daemon.ProcessSync(context.Background())

	// Validate received payload
	if receivedPayload.LocalID == "" {
		t.Fatalf("expected 1 mission to be synced, got %d", 1)
	}
	if receivedPayload.LocalID != "m1" {
		t.Errorf("expected payload ID m1, got %s", receivedPayload.LocalID)
	}
	if receivedPayload.Payload["task"] != "test-mission" {
		t.Errorf("expected test-mission, got %v", receivedPayload.Payload["task"])
	}

	// Verify sanitization
	expectedPayload := `{"details":" email is [REDACTED_EMAIL]"}`
	if receivedPayload.Payload["details"] != " email is [REDACTED_EMAIL]" {
		t.Errorf("expected sanitized payload %q, got %q", expectedPayload, receivedPayload.Payload["details"])
	}

	// Validate db status updated
	var synced bool
	err = sqlDB.QueryRow("SELECT synced_to_cloud FROM agent_missions WHERE id = 'm1'").Scan(&synced)
	if err != nil {
		t.Fatalf("failed to query m1 synced status: %v", err)
	}
	if !synced {
		t.Error("expected m1 to be synced_to_cloud = true")
	}
}

func TestHybridMCPRAGDaemon_StartStop(t *testing.T) {
	defer ClearSemaphore()
	// Setup SQLite in-memory db
	sqlDB, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB.Close()

	_, err = sqlDB.Exec(`
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
            created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
			synced_to_cloud BOOLEAN DEFAULT false,
            cloud_mission_id TEXT,
            sync_error TEXT,
            last_synced_at TIMESTAMP
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 10*time.Millisecond, "http://dummy")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	daemon.Start(ctx)

	time.Sleep(50 * time.Millisecond)

	daemon.Stop()

	// Wait a moment for the goroutine to actually exit before we defer-close the DB
	time.Sleep(10 * time.Millisecond)
	// No panic implies successful shutdown via stop channel
}
