package orchestration

import (
	"context"
	"database/sql"
	"encoding/json"
	"net/http"
	"net/http/httptest"
	"testing"
	"io"
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
			('m3', 'IGNORED', '{"task":"ignored"}', false)
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
	if len(receivedPayloads) != 1 {
		t.Fatalf("expected 1 mission to be synced, got %d", len(receivedPayloads))
	}
	if receivedPayloads[0].ID != "m1" {
		t.Errorf("expected payload ID m1, got %s", receivedPayloads[0].ID)
	}
	if receivedPayloads[0].Status != "PENDING" {
		t.Errorf("expected status PENDING, got %s", receivedPayloads[0].Status)
	}

	// Verify sanitization
	expectedPayload := `{"details":" email is [REDACTED_EMAIL]","task":"test-mission"}`
	if receivedPayloads[0].Payload != expectedPayload {
		t.Errorf("expected sanitized payload %q, got %q", expectedPayload, receivedPayloads[0].Payload)
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

func TestHybridMCPRAGDaemon_ProcessSync_Bursting(t *testing.T) {
	sqlDB_burst, err := sql.Open("sqlite", ":memory:")
	if err != nil {
		t.Fatalf("failed to open sqlite db: %v", err)
	}
	defer sqlDB_burst.Close()
	dbWrapper := &db.DB{Provider: db.NewSqliteProvider(sqlDB_burst)}
	defer dbWrapper.Close()


	ctx := context.Background()
	_, err = dbWrapper.Exec(ctx, `
		CREATE TABLE agent_missions (
			id TEXT PRIMARY KEY,
			status TEXT,
			payload TEXT,
			synced_to_cloud BOOLEAN DEFAULT FALSE
		)
	`)
	if err != nil && !dbWrapper.IsSQLite() {
		// table might exist in PG, clear it for safety if we were testing PG, but sync_daemon only runs in sqlite
	}

	_, err = dbWrapper.Exec(ctx, "INSERT INTO agent_missions (id, status, payload, synced_to_cloud) VALUES ('mission-burst-1', 'BURSTING', '{\"foo\":\"bar\"}', 0)")
	if err != nil {
		t.Fatalf("failed to insert burst mission: %v", err)
	}

	var receivedPayloads []SyncDaemonPayload
	ts := httptest.NewServer(http.HandlerFunc(func(w http.ResponseWriter, r *http.Request) {
		body, _ := io.ReadAll(r.Body)
		json.Unmarshal(body, &receivedPayloads)
		w.WriteHeader(http.StatusOK)
	}))
	defer ts.Close()

	daemon := NewHybridMCPRAGDaemon(dbWrapper, 1*time.Second, ts.URL)
	daemon.ProcessSync(ctx)

	if len(receivedPayloads) != 1 || receivedPayloads[0].ID != "mission-burst-1" {
		t.Fatalf("ProcessSync did not sync BURSTING mission. Received: %+v", receivedPayloads)
	}

	var synced int
	err = dbWrapper.QueryRow(ctx, "SELECT synced_to_cloud FROM agent_missions WHERE id = 'mission-burst-1'").Scan(&synced)
	if err != nil || synced != 1 {
		t.Fatalf("synced_to_cloud was not updated for BURSTING mission: %v, synced=%d", err, synced)
	}
}
