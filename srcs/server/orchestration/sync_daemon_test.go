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
			('m1', 'CLOUD_ESCALATION', '{"task":"test-mission", "secret":"[PRIVATE:my-secret]"}', false),
			('m2', 'COMPLETED', '{"task":"synced-mission"}', true),
			('m3', 'CLOUD_PENDING', '{"task":"waiting-mission"}', false)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedPayloads []SyncDaemonPayload
	var polledIDs []string
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/sync/missions" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedPayloads); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			w.WriteHeader(http.StatusOK)
		} else if r.URL.Path == "/api/sync/missions/poll" && r.Method == http.MethodPost {
			var body map[string][]string
			if err := json.NewDecoder(r.Body).Decode(&body); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			polledIDs = body["ids"]

			// Return m3 as DONE
			results := []SyncDaemonPayload{
				{ID: "m3", Status: "DONE", Payload: `{"task":"done-mission"}`},
			}
			w.Header().Set("Content-Type", "application/json")
			json.NewEncoder(w).Encode(results)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Process sync manually for testing
	daemon.ProcessSync(context.Background())

	// Validate received payload (syncToCloud)
	if len(receivedPayloads) != 1 {
		t.Fatalf("expected 1 mission to be synced, got %d", len(receivedPayloads))
	}
	if receivedPayloads[0].ID != "m1" {
		t.Errorf("expected payload ID m1, got %s", receivedPayloads[0].ID)
	}
	if receivedPayloads[0].Status != "CLOUD_ESCALATION" {
		t.Errorf("expected status CLOUD_ESCALATION, got %s", receivedPayloads[0].Status)
	}
	if receivedPayloads[0].Payload != `{"secret":"[REDACTED]","task":"test-mission"}` {
		t.Errorf("expected sanitized payload, got %s", receivedPayloads[0].Payload)
	}

	// Validate db status updated to CLOUD_PENDING
	var status string
	err = sqlDB.QueryRow("SELECT status FROM agent_missions WHERE id = 'm1'").Scan(&status)
	if err != nil {
		t.Fatalf("failed to query m1 status: %v", err)
	}
	if status != "CLOUD_PENDING" {
		t.Errorf("expected m1 to be status = CLOUD_PENDING, got %s", status)
	}

	// Validate fetchFromCloud
	if len(polledIDs) != 2 {
		// m1 was just set to CLOUD_PENDING, and m3 was already CLOUD_PENDING
		t.Fatalf("expected 2 missions to be polled, got %v", polledIDs)
	}

	err = sqlDB.QueryRow("SELECT status, payload FROM agent_missions WHERE id = 'm3'").Scan(&status, &receivedPayloads[0].Payload)
	if err != nil {
		t.Fatalf("failed to query m3 status: %v", err)
	}
	if status != "DONE" {
		t.Errorf("expected m3 to be status = DONE, got %s", status)
	}
	if receivedPayloads[0].Payload != `{"task":"done-mission"}` {
		t.Errorf("expected payload updated, got %s", receivedPayloads[0].Payload)
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
	dbWrapper := &db.DB{Provider: sqliteProv}

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 10*time.Millisecond, "http://dummy")

	ctx, cancel := context.WithCancel(context.Background())
	defer cancel()

	daemon.Start(ctx)

	time.Sleep(50 * time.Millisecond)

	daemon.Stop()
	// No panic implies successful shutdown via stop channel
}
