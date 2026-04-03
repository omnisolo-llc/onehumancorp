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
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
		VALUES
			('m1', 'CLOUD_ESCALATION', '{"task":"test-mission", "secret":"[PRIVATE:secret_data]"}', false),
			('m2', 'COMPLETED', '{"task":"synced-mission"}', true),
			('m3', 'PENDING_CLOUD', '{"task":"waiting-for-cloud"}', true)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := db.NewWithProvider(sqliteProv)

	// Mock cloud API
	var receivedPayloads []SyncDaemonPayload
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/sync/missions" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedPayloads); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			w.WriteHeader(http.StatusOK)
		} else if r.URL.Path == "/api/sync/missions/m3" && r.Method == http.MethodGet {
			w.WriteHeader(http.StatusOK)
			json.NewEncoder(w).Encode(SyncDaemonPayload{
				ID:      "m3",
				Status:  "DONE",
				Payload: `{"task":"done-from-cloud"}`,
			})
		} else if r.URL.Path == "/api/sync/missions/m1" && r.Method == http.MethodGet {
			w.WriteHeader(http.StatusNotFound)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Process sync manually for testing
	daemon.ProcessSync(context.Background())

	// Validate received payload
	if len(receivedPayloads) != 1 {
		t.Fatalf("expected 1 mission to be synced, got %d", len(receivedPayloads))
	}
	if receivedPayloads[0].ID != "m1" {
		t.Errorf("expected payload ID m1, got %s", receivedPayloads[0].ID)
	}
	if receivedPayloads[0].Status != "CLOUD_ESCALATION" {
		t.Errorf("expected status CLOUD_ESCALATION, got %s", receivedPayloads[0].Status)
	}

	// Validate payload was sanitized
	var parsed map[string]string
	err = json.Unmarshal([]byte(receivedPayloads[0].Payload), &parsed)
	if err != nil {
		t.Fatalf("failed to parse payload: %v", err)
	}
	if parsed["secret"] != "[REDACTED]" {
		t.Errorf("expected secret to be redacted, got %s", parsed["secret"])
	}

	// Validate db status updated to PENDING_CLOUD
	var status string
	var synced bool
	err = sqlDB.QueryRow("SELECT status, synced_to_cloud FROM agent_missions WHERE id = 'm1'").Scan(&status, &synced)
	if err != nil {
		t.Fatalf("failed to query m1 status: %v", err)
	}
	if status != "PENDING_CLOUD" {
		t.Errorf("expected m1 status to be PENDING_CLOUD, got %s", status)
	}
	if !synced {
		t.Error("expected m1 to be synced_to_cloud = true")
	}

	// Validate m3 was polled and updated to DONE
	var payload string
	err = sqlDB.QueryRow("SELECT status, payload FROM agent_missions WHERE id = 'm3'").Scan(&status, &payload)
	if err != nil {
		t.Fatalf("failed to query m3 status: %v", err)
	}
	if status != "DONE" {
		t.Errorf("expected m3 status to be DONE, got %s", status)
	}
	if payload != `{"task":"done-from-cloud"}` {
		t.Errorf("expected m3 payload to be updated, got %s", payload)
	}
}

func TestHybridMCPRAGDaemon_StartStop(t *testing.T) {
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
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := db.NewWithProvider(sqliteProv)

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 10*time.Millisecond, "http://dummy")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	daemon.Start(ctx)

	time.Sleep(50 * time.Millisecond)

	daemon.Stop()
	// No panic implies successful shutdown via stop channel
}
