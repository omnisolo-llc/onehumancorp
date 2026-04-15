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
			synced_to_cloud BOOLEAN DEFAULT false
		)
	`)
	if err != nil {
		t.Fatalf("failed to create agent_missions table: %v", err)
	}

	_, err = sqlDB.Exec(`
		INSERT INTO agent_missions (id, status, payload, synced_to_cloud)
		VALUES
			('m1', 'PENDING', '{"task":"test-mission", "details":"[PRIVATE:secret] email is a@b.com"}', false),
			('m2', 'COMPLETED', '{"task":"synced-mission"}', true),
			('m3', 'IGNORED', '{"task":"ignored"}', false),
			('m4', 'BURSTING', '{"task":"burst-mission"}', false)
	`)
	if err != nil {
		t.Fatalf("failed to insert test data: %v", err)
	}

	sqliteProv := db.NewSqliteProvider(sqlDB)
	dbWrapper := &db.DB{Provider: sqliteProv}

	// Mock cloud API
	var receivedPayloads []SyncDaemonPayload
	srv := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		if r.URL.Path == "/api/sync/missions" && r.Method == http.MethodPost {
			if err := json.NewDecoder(r.Body).Decode(&receivedPayloads); err != nil {
				w.WriteHeader(http.StatusBadRequest)
				return
			}
			w.WriteHeader(http.StatusOK)
		} else {
			w.WriteHeader(http.StatusNotFound)
		}
	}))
	defer srv.Close()

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Minute, srv.URL)

	// Process sync manually for testing
	daemon.ProcessSync(context.Background())

	// Validate received payload
	if len(receivedPayloads) != 2 {
		t.Fatalf("expected 2 missions to be synced, got %d", len(receivedPayloads))
	}

	var m1Payload, m4Payload *SyncDaemonPayload
	for i := range receivedPayloads {
		if receivedPayloads[i].ID == "m1" {
			m1Payload = &receivedPayloads[i]
		}
		if receivedPayloads[i].ID == "m4" {
			m4Payload = &receivedPayloads[i]
		}
	}

	if m1Payload == nil || m4Payload == nil {
		t.Fatalf("expected m1 and m4 to be synced, got payloads: %v", receivedPayloads)
	}

	if m1Payload.Status != "PENDING" {
		t.Errorf("expected m1 status PENDING, got %s", m1Payload.Status)
	}

	if m4Payload.Status != "BURSTING" {
		t.Errorf("expected m4 status BURSTING, got %s", m4Payload.Status)
	}

	// Verify sanitization
	expectedPayload := `{"details":" email is [REDACTED_EMAIL]","task":"test-mission"}`
	if m1Payload.Payload != expectedPayload {
		t.Errorf("expected sanitized payload %q, got %q", expectedPayload, m1Payload.Payload)
	}

	expectedM4Payload := `{"task":"burst-mission"}`
	if m4Payload.Payload != expectedM4Payload {
		t.Errorf("expected sanitized payload %q, got %q", expectedM4Payload, m4Payload.Payload)
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

	var m4Synced bool
	err = sqlDB.QueryRow("SELECT synced_to_cloud FROM agent_missions WHERE id = 'm4'").Scan(&m4Synced)
	if err != nil {
		t.Fatalf("failed to query m4 synced status: %v", err)
	}
	if !m4Synced {
		t.Error("expected m4 to be synced_to_cloud = true")
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

	// Wait a moment for the goroutine to actually exit before we defer-close the DB
	time.Sleep(10 * time.Millisecond)
	// No panic implies successful shutdown via stop channel
}
